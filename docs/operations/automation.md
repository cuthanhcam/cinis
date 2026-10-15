# Automation contract

Status: Implemented runner contract. Updated: 2026-10-15.

## Invocation and failure semantics

Invoke-Cinis.ps1 accepts a known project/task and a bounded timeout. It loads trusted projects.psd1 through Import-PowerShellDataFile, creates one run directory, resolves executables from PATH, and invokes them directly through Start-Process without cmd /c, Invoke-Expression, or another shell. The child working directory is always the repository root. Windows argument quoting preserves spaces, embedded quotes, empty values, and trailing backslashes.

Child stdout/stderr are redirected to separate files. The runner waits in short intervals to enforce timeout. It caches the child process handle for reliable exit-code retrieval under Windows PowerShell 5.1. On timeout it terminates that owned PID and descendants with taskkill /T; no process-name matching is used. This is a developer-job mechanism, not a production supervisor. A child that deliberately detaches or a hard-killed runner is outside lifecycle guarantees.

Native exit codes propagate. Run summaries stop on the first failed step; missing tools are failure, not skipped success. Individual metadata is written before waiting and in finally. Metadata may remain incomplete if the host terminates abruptly.

## Extending the project registry

1. Introduce a real project and its pinned toolchain first.
2. Add its fixed commands to ops/config/projects.psd1 and extend the public project ValidateSet.
3. Keep credentials out of arguments; inject secrets through an approved runtime mechanism with application-side redaction.
4. Add task-specific tests and update the local runbook with prerequisites, success output, failure behavior, timeout, and cleanup.
5. For a persistent service, implement the lifecycle contract before labeling the registry entry a service.

Do not add a fake Platform entry, placeholder global.json, startup port, or .env.example before a .NET executable/configuration exists. Shared scripts should remain thin; business behavior belongs to application code and its tests. CI should call the same public runner where platform support allows.

## Verification and CI

python -m unittest discover -s ops/tests -v exercises native success/failure, missing tools, arguments and spaces, dual-stream output beyond pipe capacity, timeout, and process cleanup. A Windows CI job runs these tests, the Engine Verify task, and the demo. It uploads artifacts/logs even when a previous step fails, retaining them for seven days.

Direct engine CI remains on Linux; ops automation is separately verified on Windows. Local verification does not imply a hosted CI result. PowerShell 5.1 is the tested local baseline; future PowerShell versions require their own CI evidence.

## References and local precedent

- [Microsoft Start-Process documentation](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.management/start-process?view=powershell-5.1): working directory, process object, window behavior, and stream redirection.
- [PowerShell issue 5421](https://github.com/PowerShell/PowerShell/issues/5421): exit-code behavior when using redirected Start-Process children.
- [Cargo run](https://doc.rust-lang.org/cargo/commands/cargo-run.html): locked execution and separation of application and Cargo output.

Local precedents inspected: Aegis/eng/verify-backend.ps1, Aegis/eng/migrate-database.ps1, and learn-dotnet/scripts/Test-SolutionInventory.ps1. Reused principles: script-relative repository paths, explicit error handling, locked tool commands, and fail-fast verification. The log/timeout/metadata implementation is specific to Cinis.

## Hosted Windows compatibility regression

Built-ins are imported from the current PSHOME in public scripts and the process module. This avoids inherited PowerShell 7 module paths shadowing Windows PowerShell 5.1 modules. The public fail-fast test injects an incompatible Utility module into PSModulePath. Verify additionally validates workflows and scaffold contracts; exact-commit hosted success is a separate publication gate. See [CI procedure](../development/ci.md).
