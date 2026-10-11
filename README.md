# Cinis

A deterministic trading venue simulator with a Rust execution core and a .NET management platform.

Cinis explores matching, pre-trade risk, financial state, durable execution, recovery, and reproducible performance measurement. Cinis replaces the earlier working name Cineres.

## Status

M0: definition and repository bootstrap. No executable application exists yet. Specifications describe intended behavior; initial policy choices are a reviewable baseline, not verified implementation guarantees.

## v0.1 target

One spot market, synthetic funds, limit GTC orders, price-time priority, partial fills, cancellation, reservations, internal settlement, API keys, recovery, and market data.

Acceptance journey: funding -> place sell 10 at 100 -> buy 4 -> verify partial fill -> retry without duplicate effects -> restart/recover -> cancel remaining 6 -> reconcile balances.

## Navigation

- [Documentation index](docs/README.md)
- [Charter](docs/product/project-charter.md) and [scope](docs/product/scope-v0.1.md)
- [Architecture](docs/architecture/overview.md)
- [Milestones](docs/delivery/milestones.md)
- [Setup](docs/development/setup.md)
- [Contributing](CONTRIBUTING.md) and [Git workflow](GIT_GUIDELINES.md)

## Layout

| Path | Purpose |
| --- | --- |
| docs/ | Product, domain, architecture, verification, delivery |
| src/engine/ | Future Rust execution workspace |
| src/platform/ | Future .NET gateway and management solution |
| tools/ | Future CLI and workload generator |
| tests/ | Future shared acceptance and integration fixtures |
| deploy/ | Future local orchestration |
| research/ | Timeboxed spikes and reports |
| scripts/ | Repository checks |

Run `python scripts/check_docs.py` with Python 3.11+. Toolchain pins and application scaffolding are M0 follow-up work. Origin: https://github.com/cuthanhcam/cinis.git. Main holds foundation files; develop integrates specifications and tooling for pull request review.

Licensed under [MIT](LICENSE).
