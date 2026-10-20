# Logs and debugging

Status: Implemented local process-log contract. Owner: repository maintainer. Updated: 2026-10-20.

## Files and identity

Finite jobs default to Archive mode: artifacts/logs/<UTC-timestamp>-<random-suffix>. Serve defaults to Latest mode: artifacts/logs/latest/<project>/<task>/ (lowercase). Latest replaces known runner output files at the next invocation; unrelated files are retained. Every invocation still has a unique run ID in metadata and the child process. A per-project/task exclusive file lock prevents another Latest runner from truncating active logs. Failed lock acquisition starts no child and leaves active outputs intact. Metadata timestamps are UTC ISO 8601; convert to local time when comparing with external events.

| File | Content |
| --- | --- |
| run.json | Schema version 2, log mode, run ID, revision, PowerShell version, project/task/kind, timeout, start/finish, status, exit code, completed steps |
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

RunId accepts only the runner-generated format; directory traversal is rejected. Without RunId or Latest, inspection selects the most recently updated run.json, including active/latest runs. -Latest selects the stable project/task path. For live inspection use Get-Content -LiteralPath <exact-log-path> -Tail 50 -Wait in a second terminal, then Ctrl+C to stop the viewer. The runner prints the last 30 lines of each stream after the step finishes; full output remains on disk. A long line can still produce large console output.

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

## Repeatable service debugging

```powershell
# Overwrite the same files each time; Serve selects Latest automatically.
powershell -NoProfile -File ops/scripts/Invoke-Cinis.ps1 -Project Platform -Task Serve -TimeoutSeconds 300
powershell -NoProfile -File ops/scripts/Get-CinisLogs.ps1 -Latest -Project Platform -Task Serve -Tail 100
Get-Content -LiteralPath artifacts/logs/latest/platform/serve/01-dotnet.stdout.log -Tail 50 -Wait
# Preserve this run instead when comparing incidents.
powershell -NoProfile -File ops/scripts/Invoke-Cinis.ps1 -Project Platform -Task Serve -LogMode Archive -TimeoutSeconds 300
```

Stop/restart a live viewer when starting a replacement run. Both stdout and stderr are truncated before child resolution, preventing stale output even on failed starts. Missing tools and nonzero exits remain failures. Latest does not rotate within a long-running invocation; bound service duration and use Archive for evidence you need to retain. Known runner files are individually removed without recursive deletion or caller-controlled destinations. The lock is released on normal completion/failure; Windows releases the file handle after process termination. An empty lock file can remain and does not mean a runner is active. Archive runs have independent paths and can run concurrently, subject to the application's own port/resource constraints.

For Platform records, find the response X-Request-ID or X-Trace-ID in Scope, then compare RunId with run.json. Event 1000 includes status and elapsed time; event 1001 contains exception type/stack without exception messages. See the [HTTP logging contract](../development/platform-http-pipeline.md) for privacy boundaries, category configuration and pipeline ordering. CINIS_RUN_ID is inherited by the child only during launch; the runner restores its previous process environment. Run identity is diagnostic metadata, never authentication.
