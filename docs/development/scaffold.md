# Scaffold readiness and maintenance

Status: Implemented repository foundation; application expansion gates remain explicit. Owner: repository maintainer. Updated: 2026-10-15.

## Delivered foundation

| Concern | Canonical files / checks |
| --- | --- |
| Purpose and onboarding | README, documentation index, setup/testing/ops runbooks |
| Product/domain/architecture | Requirements, rules, financial model, state ownership, ADRs |
| Contribution and conduct | CONTRIBUTING, CODE_OF_CONDUCT, commit/Git guidelines, AGENTS |
| Security and support | SECURITY, .github/SUPPORT, private-reporting guidance, safe issue fields |
| Git/editor hygiene | Annotated .gitignore/.gitattributes/.editorconfig; shared VS Code suggestions without private paths |
| Reviews and intake | CODEOWNERS, PR evidence checklist, bug/work/question templates |
| Dependency upkeep | Dependabot for actual Cargo, NuGet and GitHub Actions dependencies; reviewed action SHAs |
| Automation and diagnostics | ops registry/scripts/tests; unique logs, metadata, timeout, failure propagation |
| CI integrity | Five workflows on all work branches/PRs, actionlint, exact-SHA fail-closed gate |
| Reusable documentation | ADR, feature brief, runbook, experiment templates; language-aware naming |
| Environment reference | .env.example documents currently consumed GH_TOKEN/RUST_BACKTRACE; no automatic dotenv loading |

## Intentional application gates

Rust execution and the .NET scaffold now exist. .NET has five production/five test projects, locked dependencies and Linux/Windows CI; see [platform guide](platform.md). The health-only host defaults to no grants and no execution connection. Authentication, financial endpoints, PostgreSQL, containers and recovery remain undelivered.

Before financial gateway integration: complete framing/trust/access discovery, then add authenticated endpoints and Rust transport with failure tests. Financial source work now follows ledger.md and ADR-0005 with numerical/reconciliation tests. Before services: readiness/shutdown/log rotation and local binding policy. Before deploy: real image/manifest, volume/secret ownership, health checks, and cleanup/restore instructions.

## Maintainer checklist

- Keep supported commands, pins, task registry, CI, and docs synchronized.
- Verify required hosted workflows on the exact candidate commit before advancing develop.
- Check that source lockfiles, migrations, safe configs, and shared editor files remain tracked, while local outputs/credentials are ignored.
- Do not claim CODEOWNERS, issue forms, Dependabot, or private reporting are active on GitHub's default branch until that branch contains the configuration. Rulesets/settings require separate hosting configuration.
- Review Dependabot updates and CI action runtime changes; no auto-merge policy is installed.
- Record failures and regression coverage; do not increase timeouts or skip checks to conceal the cause.
- Keep one implementation backlog and avoid prematurely creating empty language/framework layers.

The code/config baseline is ready only when hosted validation for the delivered SHA is recorded. This page inventories implemented scaffold responsibilities; it does not claim v0.1 or all future application scaffolds are complete.
