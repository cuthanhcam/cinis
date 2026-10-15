"""CI gate must not accept missing, stale, skipped, or failed workflow results."""
import importlib.util
from pathlib import Path
import unittest

SPEC = importlib.util.spec_from_file_location("cinis_ci_gate", Path(__file__).resolve().parents[1] / "scripts/check_ci.py")
GATE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(GATE)
REVISION = "a" * 40


def successful_runs():
    return [{"id": index, "name": name, "head_sha": REVISION, "event": "push", "status": "completed", "conclusion": "success"}
            for index, name in enumerate(GATE.REQUIRED, 1)]


class CiGateTests(unittest.TestCase):
    def test_all_required_workflows_success(self):
        self.assertEqual(GATE.assess(successful_runs(), REVISION)[0], 0)

    def test_missing_and_wrong_commit_are_pending(self):
        self.assertEqual(GATE.assess(successful_runs()[:-1], REVISION)[0], 2)
        self.assertEqual(GATE.assess(successful_runs(), "b" * 40)[0], 2)

    def test_skipped_failed_and_cancelled_are_not_success(self):
        for conclusion in ("failure", "skipped", "cancelled", "timed_out", "neutral"):
            runs = successful_runs()
            runs[0]["conclusion"] = conclusion
            self.assertEqual(GATE.assess(runs, REVISION)[0], 1)

    def test_newer_pending_run_overrides_older_success(self):
        runs = successful_runs()
        runs.append(dict(runs[0], id=100, status="in_progress", conclusion=None))
        self.assertEqual(GATE.assess(runs, REVISION)[0], 2)

    def test_pull_request_result_does_not_substitute_for_push(self):
        runs = successful_runs()
        runs[0]["event"] = "pull_request"
        self.assertEqual(GATE.assess(runs, REVISION)[0], 2)


if __name__ == "__main__":
    unittest.main()
