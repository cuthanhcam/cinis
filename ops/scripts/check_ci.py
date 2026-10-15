"""Read-only exact-commit GitHub push-workflow gate; never merges or pushes."""
import argparse
import json
import os
from pathlib import Path
import re
import subprocess
import sys
import time
import urllib.parse
import urllib.request

ROOT = Path(__file__).resolve().parents[2]
REQUIRED = ("Documentation", "Engine", "Ops", "Repository")


def assess(runs, revision, required=REQUIRED):
    selected = {}
    for run in runs:
        if run.get("head_sha") != revision or run.get("event") != "push" or run.get("name") not in required:
            continue
        name = run["name"]
        if name not in selected or (run["id"], run.get("run_attempt", 1)) > (selected[name]["id"], selected[name].get("run_attempt", 1)):
            selected[name] = run
    statuses = {}
    for name in required:
        run = selected.get(name)
        statuses[name] = "missing" if run is None else (
            run.get("conclusion") if run.get("status") == "completed" else run.get("status", "pending")
        )
    if any(value in {"failure", "cancelled", "timed_out", "action_required", "startup_failure", "skipped", "neutral", "stale"} for value in statuses.values()):
        return 1, statuses, selected
    return (0 if all(value == "success" for value in statuses.values()) else 2), statuses, selected


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repository", default="cuthanhcam/cinis")
    parser.add_argument("--revision")
    parser.add_argument("--wait", action="store_true")
    parser.add_argument("--timeout-seconds", type=int, default=900)
    arguments = parser.parse_args()
    revision = arguments.revision or subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=ROOT, text=True).strip()
    if not re.fullmatch(r"[a-f0-9]{40}", revision) or not re.fullmatch(r"[A-Za-z0-9_.-]+/[A-Za-z0-9_.-]+", arguments.repository):
        parser.error("Use a full commit SHA and owner/repository name.")
    if not 1 <= arguments.timeout_seconds <= 3600:
        parser.error("Timeout must be between 1 and 3600 seconds.")
    headers = {"User-Agent": "Cinis-CI-gate", "Accept": "application/vnd.github+json"}
    token = os.environ.get("GH_TOKEN") or os.environ.get("GITHUB_TOKEN")
    if token:
        headers["Authorization"] = "Bearer " + token
    query = urllib.parse.urlencode({"head_sha": revision, "event": "push", "per_page": 100})
    url = f"https://api.github.com/repos/{arguments.repository}/actions/runs?{query}"
    deadline = time.monotonic() + arguments.timeout_seconds
    previous = None
    while True:
        with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=30) as response:
            runs = json.load(response)["workflow_runs"]
        code, statuses, selected = assess(runs, revision)
        if statuses != previous:
            print(json.dumps({"revision": revision, "workflows": statuses}), flush=True)
            previous = statuses
        evidence = {"revision": revision, "repository": arguments.repository, "statuses": statuses,
                    "runs": {name: {key: run.get(key) for key in ("id", "head_sha", "status", "conclusion", "html_url", "run_attempt")} for name, run in selected.items()}}
        directory = ROOT / "artifacts" / "ci"
        directory.mkdir(parents=True, exist_ok=True)
        (directory / f"{revision}.json").write_text(json.dumps(evidence, indent=2) + "\n", encoding="utf-8")
        if code != 2 or not arguments.wait or time.monotonic() >= deadline:
            for name, run in selected.items():
                print(f"{name}: {run['html_url']}", flush=True)
            return code
        time.sleep(20)


if __name__ == "__main__":
    sys.exit(main())
