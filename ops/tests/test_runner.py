"""Integration tests of process logging/failure behavior on Windows; no third-party packages."""
import base64
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import time
import unittest

MODULE = Path(__file__).resolve().parents[1] / "scripts" / "Cinis.Operations.psm1"


def ps_quote(value):
    return "'" + str(value).replace("'", "''") + "'"


@unittest.skipUnless(sys.platform == "win32", "Windows operations runner")
class RunnerTests(unittest.TestCase):
    def invoke(self, tool, arguments, timeout=20):
        directory = Path(self.work.name)
        encoded_arguments = ",".join(ps_quote(a) for a in arguments)
        script = (
            f"$ErrorActionPreference='Stop'; Import-Module {ps_quote(MODULE)} -Force; "
            f"$result=Invoke-CinisLoggedProcess -Tool {ps_quote(tool)} "
            f"-Arguments @({encoded_arguments}) -WorkingDirectory {ps_quote(directory)} "
            f"-LogDirectory {ps_quote(directory)} -StepName test -TimeoutSeconds {timeout}; "
            "exit ([int]$result.exitCode)"
        )
        encoded = base64.b64encode(script.encode("utf-16le")).decode("ascii")
        result = subprocess.run(
            ["powershell", "-NoProfile", "-NonInteractive", "-EncodedCommand", encoded],
            capture_output=True, text=True, timeout=40,
        )
        self.assertTrue((directory / "test.json").exists(), result.stdout + result.stderr)
        record = json.loads((directory / "test.json").read_text(encoding="utf-8-sig"))
        return result, record, directory

    def setUp(self):
        self.work = tempfile.TemporaryDirectory(prefix="cinis ops test ")
        self.addCleanup(self.work.cleanup)

    def fixture_runner(self, code, task="Run"):
        """Use the public script with a tiny real child, without starting a financial service."""
        import shutil
        directory = Path(self.work.name)
        scripts = directory / "ops" / "scripts"
        scripts.mkdir(parents=True)
        config = directory / "ops" / "config"
        config.mkdir()
        for name in ("Cinis.Operations.psm1", "Invoke-Cinis.ps1", "Get-CinisLogs.ps1"):
            shutil.copyfile(MODULE.parent / name, scripts / name)
        (config / "projects.psd1").write_text(
            "@{Engine=@{Kind='Job';Tasks=@{" + task + "=@(@{Tool=" + ps_quote(sys.executable)
            + ";Arguments=@('-u','-c'," + ps_quote(code) + ")})}}}", encoding="utf-8")
        return directory, ["powershell", "-NoProfile", "-NonInteractive", "-File",
                           str(scripts / "Invoke-Cinis.ps1"), "-Project", "Engine", "-Task", task]

    def test_latest_replaces_logs_and_exposes_child_run_identity(self):
        directory, command = self.fixture_runner(
            "from pathlib import Path; import os; print(Path('message.txt').read_text()); print(os.environ['CINIS_RUN_ID'])")
        latest = directory / "artifacts" / "logs" / "latest" / "engine" / "run"
        identities = []
        for message in ("first-run-content", "second-run-content"):
            (directory / "message.txt").write_text(message)
            result = subprocess.run([*command, "-LogMode", "Latest"], capture_output=True, text=True, timeout=30)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
            summary = json.loads((latest / "run.json").read_text(encoding="utf-8-sig"))
            identities.append(summary["runId"])
            self.assertEqual(summary["logMode"], "Latest")
            output = (latest / "01-python.stdout.log").read_text()
            self.assertIn(message, output)
            self.assertIn(summary["runId"], output)
        self.assertNotEqual(*identities)
        self.assertNotIn("first-run-content", output)
        read = subprocess.run(["powershell", "-NoProfile", "-File",
                               str(directory / "ops/scripts/Get-CinisLogs.ps1"), "-Latest",
                               "-Project", "Engine", "-Task", "Run"], capture_output=True, text=True, timeout=15)
        self.assertEqual(read.returncode, 0, read.stdout + read.stderr)
        self.assertIn("second-run-content", read.stdout)

    def test_serve_latest_lock_prevents_concurrent_truncation(self):
        directory, command = self.fixture_runner("import time; print('owned-output',flush=True); time.sleep(30)", "Serve")
        first = subprocess.Popen([*command, "-TimeoutSeconds", "15"], stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
        latest = directory / "artifacts/logs/latest/engine/serve"
        output = latest / "01-python.stdout.log"
        deadline = time.monotonic() + 10
        while time.monotonic() < deadline:
            if output.exists() and "owned-output" in output.read_text():
                break
            time.sleep(0.05)
        else:
            first.communicate(timeout=20)
            self.fail("Fixture child did not start.")
        second = subprocess.run(command, capture_output=True, text=True, timeout=15)
        self.assertNotEqual(second.returncode, 0)
        self.assertIn("owned-output", output.read_text())
        first.communicate(timeout=20)
        self.assertEqual(first.returncode, 124)
        summary = json.loads((latest / "run.json").read_text(encoding="utf-8-sig"))
        self.assertEqual(summary["exitCode"], 124)

    def test_serve_archive_opt_in_keeps_distinct_runs(self):
        directory, command = self.fixture_runner("print('finite fixture')", "Serve")
        for _ in range(2):
            result = subprocess.run([*command, "-LogMode", "Archive"], capture_output=True, text=True, timeout=20)
            self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        summaries = list((directory / "artifacts/logs").glob("*/run.json"))
        self.assertEqual(len(summaries), 2)
        self.assertFalse((directory / "artifacts/logs/latest").exists())

    def test_unsupported_project_task_fails_with_metadata(self):
        script = MODULE.parent / "Invoke-Cinis.ps1"
        result = subprocess.run(
            ["powershell", "-NoProfile", "-File", str(script), "-Project", "Platform", "-Task", "RunMatching"],
            capture_output=True, text=True, timeout=30,
        )
        self.assertEqual(result.returncode, 1, result.stdout + result.stderr)
        summary_line = next(line for line in result.stdout.splitlines() if "Summary: " in line)
        record = json.loads(Path(summary_line.split("Summary: ", 1)[1].strip()).read_text(encoding="utf-8-sig"))
        self.assertEqual(record["project"], "Platform")
        self.assertEqual(record["status"], "Failed")
        self.assertEqual(record["steps"], [])

    def test_quoted_arguments_and_streams(self):
        expected = ['literal "quoted" value', "C:\\folder with spaces\\", "", "a'b;$literal"]
        code = "import json,sys; print(json.dumps(sys.argv[1:])); print('diagnostic',file=sys.stderr)"
        result, record, directory = self.invoke(sys.executable, ["-c", code, *expected])
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)
        self.assertEqual(record["status"], "Succeeded")
        self.assertEqual(json.loads((directory / "test.stdout.log").read_text()), expected)
        self.assertIn("diagnostic", (directory / "test.stderr.log").read_text())
        self.assertIsNotNone(record["processId"])
        self.assertIsNotNone(record["finishedUtc"])
        self.assertNotIn("arguments", record)

    def test_nonzero_exit_is_preserved(self):
        result, record, _ = self.invoke(sys.executable, ["-c", "import sys; sys.exit(7)"])
        self.assertEqual(result.returncode, 7)
        self.assertEqual(record["exitCode"], 7)
        self.assertEqual(record["status"], "Failed")

    def test_large_dual_streams_do_not_deadlock(self):
        code = "import sys; print('o'*200000); print('e'*200000,file=sys.stderr)"
        result, record, directory = self.invoke(sys.executable, ["-c", code])
        self.assertEqual(result.returncode, 0)
        self.assertEqual(record["status"], "Succeeded")
        self.assertGreater((directory / "test.stdout.log").stat().st_size, 200000)
        self.assertGreater((directory / "test.stderr.log").stat().st_size, 200000)

    def test_missing_tool_records_failure(self):
        result, record, _ = self.invoke("cinis-deliberately-missing-tool", ["unused"])
        self.assertEqual(result.returncode, 1)
        self.assertEqual(record["status"], "Failed")
        self.assertIsNone(record["processId"])

    def test_timeout_stops_owned_process(self):
        result, record, _ = self.invoke(sys.executable, ["-c", "import time; time.sleep(30)"], timeout=1)
        self.assertEqual(result.returncode, 124)
        self.assertEqual(record["status"], "TimedOut")
        # Query only the PID captured by this invocation; no process-name based termination.
        check = subprocess.run(
            ["powershell", "-NoProfile", "-Command",
             f"if (Get-Process -Id {record['processId']} -ErrorAction SilentlyContinue) {{exit 1}}"],
            capture_output=True, timeout=10,
        )
        self.assertEqual(check.returncode, 0)


    def test_timeout_stops_descendants(self):
        code = (
            "import subprocess,sys,time; "
            "p=subprocess.Popen([sys.executable,'-c','import time; time.sleep(30)']); "
            "print(p.pid,flush=True); time.sleep(30)"
        )
        result, record, directory = self.invoke(sys.executable, ["-c", code], timeout=2)
        self.assertEqual(result.returncode, 124)
        child_id = int((directory / "test.stdout.log").read_text().strip())
        for process_id in (record["processId"], child_id):
            check = subprocess.run(
                ["powershell", "-NoProfile", "-Command",
                 f"if (Get-Process -Id {process_id} -ErrorAction SilentlyContinue) {{exit 1}}"],
                capture_output=True, timeout=10,
            )
            self.assertEqual(check.returncode, 0)

    def test_public_runner_stops_after_failure(self):
        import shutil
        directory = Path(self.work.name)
        scripts = directory / "ops" / "scripts"
        scripts.mkdir(parents=True)
        config = directory / "ops" / "config"
        config.mkdir()
        for name in ("Cinis.Operations.psm1", "Invoke-Cinis.ps1"):
            shutil.copyfile(MODULE.parent / name, scripts / name)
        marker = directory / "should-not-exist.txt"
        registry = (
            "@{Engine=@{Kind='Job';Tasks=@{Verify=@("
            f"@{{Tool={ps_quote(sys.executable)};Arguments=@('-c','import sys; sys.exit(9)')}},"
            f"@{{Tool={ps_quote(sys.executable)};Arguments=@('-c',"
            + ps_quote(f"from pathlib import Path; Path({str(marker)!r}).write_text('unexpected')")
            + ")})}}}"
        )
        (config / "projects.psd1").write_text(registry, encoding="utf-8")
        # Reproduce hosted pwsh -> Windows PowerShell module-path shadowing.
        shadow = directory / "shadow-modules" / "Microsoft.PowerShell.Utility"
        shadow.mkdir(parents=True)
        (shadow / "Microsoft.PowerShell.Utility.psd1").write_text(
            "@{ModuleVersion='1.0.0';PowerShellVersion='99.0'}", encoding="utf-8"
        )
        environment = os.environ.copy()
        environment["PSMODULEPATH"] = str(shadow.parent) + ";" + environment.get("PSMODULEPATH", "")
        result = subprocess.run(
            ["powershell", "-NoProfile", "-NonInteractive", "-File",
             str(scripts / "Invoke-Cinis.ps1"), "-Task", "Verify"],
            cwd=directory.parent, env=environment, capture_output=True, text=True, timeout=30,
        )
        self.assertEqual(result.returncode, 9, result.stdout + result.stderr)
        runs = list((directory / "artifacts" / "logs").glob("*/run.json"))
        self.assertEqual(len(runs), 1)
        summary = json.loads(runs[0].read_text(encoding="utf-8-sig"))
        self.assertEqual(summary["status"], "Failed")
        self.assertEqual(len(summary["steps"]), 1)
        self.assertFalse(marker.exists())


if __name__ == "__main__":
    unittest.main()
