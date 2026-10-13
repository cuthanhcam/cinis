# Cinis

Cinis is a Rust/.NET trading infrastructure lab for deterministic order execution, financial correctness, durable recovery, and reproducible performance research.

Cinis explores matching, pre-trade risk, financial state, durable execution, recovery, and reproducible performance measurement. Cinis replaces the earlier working name Cineres.

## Status

The first matching-only Rust core and CLI are implemented: limit GTC, price-time priority, resting-price fills, partial fills, cancellation, ownership validation, self-trade preflight, and bounded admission. Seventeen shared scenario fixtures are executable tests.

This prototype is volatile and has no financial state, command-envelope deduplication, gateway, or recovery. M0 remains open for the cross-language spike; M1 remains open for independent reference/generated testing and review. Full v0.1 behavior below is still planned.

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
| src/engine/ | Matching-only Rust crate and CLI |
| src/platform/ | Future .NET gateway and management solution |
| tools/ | Future CLI and workload generator |
| tests/ | Shared executable matching fixtures |
| deploy/ | Future local orchestration |
| research/ | Timeboxed spikes and reports |
| scripts/ | Repository checks |

## Run the first flow

```sh
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
cargo test --locked --manifest-path src/engine/Cargo.toml
```

Rust is pinned to 1.95.0. See [testing](docs/development/testing.md) for format/lint commands and [requirements](docs/product/requirements.md) for acceptance boundaries. Run `python scripts/check_docs.py` with Python 3.11+ for documentation checks. Origin: https://github.com/cuthanhcam/cinis.git. Main holds foundation files; develop integrates specifications and tooling for pull request review.

Licensed under [MIT](LICENSE).
