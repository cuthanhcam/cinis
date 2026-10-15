# Ops

Automation for local builds, verification, and process runs. Supported host: Windows with PowerShell 5.1 or later. No third-party PowerShell module is required. Linux/macOS can use the direct Cargo commands; this runner does not claim support there.

## Quick start

From the repository root:

```powershell
powershell -NoProfile -File .\ops\scripts\Invoke-Cinis.ps1 -Task Verify
powershell -NoProfile -File .\ops\scripts\Invoke-Cinis.ps1 -Task Run
powershell -NoProfile -File .\ops\scripts\Get-CinisLogs.ps1 -Tail 50
```

Scripts also work when called by absolute path from a different directory. They do not change the caller's location or install tools. If machine policy prevents script execution, use your organization's approved policy; scripts do not disable policy automatically.

## Layout

| Path | Responsibility |
| --- | --- |
| config/projects.psd1 | Trusted registry of implemented projects and fixed task arguments |
| scripts/Invoke-Cinis.ps1 | Public entry point: Build, Test, Run, Format, Lint, Docs, Verify |
| scripts/Cinis.Operations.psm1 | Internal process launch, argument quoting, timeout, stream capture, metadata |
| scripts/Get-CinisLogs.ps1 | Inspect latest or specific run without modifying files |
| tests/test_runner.py | Windows process integration tests, standard-library Python |

Generated output lives under artifacts/logs/<run-id>, which Git ignores. Documentation lives in [docs/ops](../docs/ops/README.md). Do not create an alternative operation/operations folder or a duplicate document tree.

Current registered project: Engine, a finite matching-only CLI job. There is no running network service, database, gateway, service manager, or production deployment. See [service lifecycle](../docs/ops/service-lifecycle.md) before adding one.
