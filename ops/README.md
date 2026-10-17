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
| scripts/Invoke-Cinis.ps1 | Public entry point: Build, Test, Run, RunMatching, Format, Lint, Docs, Workflows, Verify |
| scripts/Cinis.Operations.psm1 | Internal process launch, argument quoting, timeout, stream capture, metadata |
| scripts/check_docs.py | Portable local documentation validation |
| scripts/check_ci.py | Read-only exact-SHA hosted integration gate |
| scripts/check_workflows.py | Pinned/checksum-verified workflow validation |
| scripts/check_scaffold.py | Foundation inventory and ignore-policy checks |
| scripts/check_naming.py | Maintained-path naming validation |
| scripts/Get-CinisLogs.ps1 | Inspect latest or specific run without modifying files |
| tests/test_runner.py | Windows process integration tests, standard-library Python |

Generated output lives under artifacts/logs/<run-id>, which Git ignores. Documentation lives in [docs/operations](../docs/operations/README.md). Do not create an alternative operation/operations folder or a duplicate document tree.

Current registered project: Engine, with finite synthetic-financial (Run) and matching-only (RunMatching) CLI jobs. There is no running network service, database, gateway, service manager, or production deployment. See [service lifecycle](../docs/operations/service-lifecycle.md) before adding one.

Verify now also checks workflows and scaffold contracts. First workflow validation needs internet access to download pinned actionlint. Hosted integration requirements are documented in [CI procedure](../docs/development/ci.md); check_ci.py never pushes or merges.

## Platform tasks

Use Invoke-Cinis.ps1 -Project Platform -Task Verify or Run for locked verification/finite smoke. Serve is bounded foreground debugging at http://127.0.0.1:5080 with captured logs; it is not a supervisor and reaches timeout exit 124 at its configured deadline. Readiness stays 503. See [platform guide](../docs/development/platform.md).
