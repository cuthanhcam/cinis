# Local development runbook

Status: Implemented. Owner: repository maintainer. Last verified: 2026-10-15, Windows PowerShell 5.1. Scope: Rust matching-only job.

## Prerequisites

Git, rustup with pinned Rust 1.95.0/rustfmt/Clippy, MSVC build tools, and Python 3.11+. Run from a checkout with the toolchain installed. No database, cloud account, credentials, or .env file is required. The runner intentionally does not install prerequisites.

## Verify

```powershell
powershell -NoProfile -File .\ops\scripts\Invoke-Cinis.ps1 -Project Engine -Task Verify
```

Verify performs format check, Clippy with warnings denied, locked build, locked tests, then documentation and path-naming checks. It stops at the first failed step. Each step has a 600-second default timeout. Override explicitly for a slow first build with -TimeoutSeconds 1800. The timeout is per step, not the total workflow.

Success: process exit 0 and run.json status Succeeded. Every completed step also has exit 0. Failure: nonzero exit, remaining steps not started, available output and metadata retained. Inspect logs before rerunning; every invocation creates a new directory.

## Run the demo

```powershell
powershell -NoProfile -File .\ops\scripts\Invoke-Cinis.ps1 -Task Run
```

Expected stdout: sell order accepted; buy accepted; trade quantity 4 at 100; cancellation quantity 6; final book empty. Cargo build diagnostics appear in stderr even on success. No balances, persistent orders, background service, or listener is created. No shutdown/cleanup command is required after the job exits.

## Individual tasks and remote working directory

Build, Test, Format, Lint, and Docs are available separately. Task Run uses cargo run; all applicable Cargo actions use --locked. Formatter never rewrites source through this runner.

```powershell
powershell -NoProfile -File D:\Workspace\cinis\ops\scripts\Invoke-Cinis.ps1 -Task Test
```

Repository location is resolved from the script, not the current shell location. Paths with spaces are supported and covered by process-launch tests. Commands are fixed in the trusted registry rather than assembled from shell input.

## Failure and interruption

Native child failures preserve their exit code. Missing executables or runner failures return 1. Timeout returns 124 and terminates only the process tree owned by that step. Interruption cleanup is best effort; hard-killing PowerShell or the machine can leave Running metadata and requires inspection. Do not terminate processes by name or assume a stale PID still belongs to Cinis.

## Evidence

Use run ID, revision, task, step status, stdout/stderr, and relevant test output when reporting problems. Do not attach raw logs without reviewing sensitive content. Tool-specific versions are available through rustc --version, cargo --version, python --version; the runner records PowerShell version and repository revision without dumping environment variables.
