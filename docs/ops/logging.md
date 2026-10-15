# Logs and debugging

Status: Implemented local process-log contract. Owner: repository maintainer. Updated: 2026-10-15.

## Files and identity

Each invocation creates artifacts/logs/<UTC-timestamp>-<random-suffix>. Run IDs are unique even for concurrent invocations; output files are never reused between runs. Metadata timestamps are UTC ISO 8601; convert to local time when comparing with external events.

| File | Content |
| --- | --- |
| run.json | Schema version, run ID, revision, PowerShell version, project/task/kind, timeout, start/finish, status, exit code, completed steps |
| NN-tool.json | Step identity, tool, PID, timestamps, status/exit code, output filenames, failure exception type if applicable |
| NN-tool.stdout.log | Child standard output |
| NN-tool.stderr.log | Child standard error and compiler diagnostics |

Arguments and environment values are not serialized into metadata. Tool output is captured as emitted and may contain sensitive data if future services log it. Do not put credentials into task arguments or log authentication material. Separate stdout/stderr files avoid conflating diagnostics with application events, but do not provide a globally ordered combined event stream or per-line timestamps.

## Inspection

```powershell
powershell -NoProfile -File .\ops\scripts\Get-CinisLogs.ps1
powershell -NoProfile -File .\ops\scripts\Get-CinisLogs.ps1 -Stream stderr -Tail 100
# Use the run ID printed by Invoke-Cinis:
powershell -NoProfile -File .\ops\scripts\Get-CinisLogs.ps1 -RunId <run-id> -Tail 100
```

RunId accepts only the runner-generated format; directory traversal is rejected. Latest means newest directory name, which may still be running. For live inspection use Get-Content -LiteralPath <exact-log-path> -Tail 50 -Wait in a second terminal, then Ctrl+C to stop the viewer. The runner prints the last 30 lines of each stream after the step finishes; full output remains on disk. A long line can still produce large console output.

## Debug checklist

1. Check run.json and the first nonzero step; later steps may not have run.
2. Distinguish Rust build/test failures from launch failures. A missing tool produces failure metadata without a PID.
3. Read both streams. Nonempty stderr alone is not failure; the exit code decides.
4. Use the recorded revision and toolchain pin to reproduce the same task.
5. For a Rust panic, set RUST_BACKTRACE=1 in the invoking shell and rerun; environment is inherited but not dumped.
6. For a timeout, inspect workload/toolchain downloads before raising the per-step limit. Never treat timeout as success.

## Retention and integrity

Local logs have no automatic deletion or rotation yet. Review storage usage and archive/delete explicitly selected completed run directories under artifacts/logs when no longer needed. There is no cleanup script that could delete source or execution data. CI retains synthetic verification log artifacts for seven days. High-volume production services will need rotation, quotas, centralized collection, and structured correlation before deployment.

JSON metadata is rewritten after state changes, not transactionally committed or fsynced. Abrupt termination can leave stale/incomplete metadata; stdout/stderr are diagnostic evidence only. These logs do not provide audit-ledger or execution-journal durability.
