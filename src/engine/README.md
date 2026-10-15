# Rust execution core

One dependency-free safe-Rust crate pinned to Rust 1.95.0. ExecutionCore is the financial entry point; Engine is the lower-level matching-only primitive retained for research and its independent reference tests.

## Modules and ownership

| Source | Responsibility |
| --- | --- |
| src/lib.rs | Public API and compatibility exports |
| src/matching.rs | GTC price-time matching, self-trade/capacity preflight, order lifecycle |
| src/ledger.rs | Asset/balance/bucket types, balanced transfers, numeric bounds, reconciliation |
| src/execution/types.rs | Typed payloads/envelopes, permissions, response/events, errors, limits |
| src/execution/state.rs | Tentative financial transition, holds, settlement/release, market state, invariants |
| src/execution/mod.rs | Authorization, command sequencing, idempotency, atomic commit/retained decisions |
| src/main.rs | Funded financial demo with retry and reconciliation |
| src/bin/matching_demo.rs | Preserved matching-only demonstration |

No caller receives mutable access to the owned matcher or ledger through ExecutionCore. The primitive Engine API intentionally has no financial checks; it operates on a separate instance and must not be used as the financial gateway.

## Run and test

```sh
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-matching-demo
cargo test --locked --manifest-path src/engine/Cargo.toml
```

Default cinis-demo funds buyer with 1,000 quote and seller with 10 base, sells 10 at 100, buys 4, retries without another effect, cancels 6, and asserts final balances/ledger. This is volatile execution, not the full restart acceptance journey.

## API contract

ExecutionRequest carries account_id, command_id, and a typed Fund/Place/Cancel/SetMarketState payload. A trusted caller supplies Permissions. Invalid envelope/authorization/conflict/admission capacity returns AdmissionError with no state/sequence effect. An admitted request returns ExecutionResponse with a stable sequence and either Effects or a retained ExecutionReject. Business rejects do not mutate financial state but consume one sequence/cache slot.

An authorized identical retry returns the original response and identifiers, even after balances/market changed. It is historical result retrieval, not a new state transition or current-order query. Conflicting payload rejects. Permissions are not authentication; network identity and API keys remain M3.

See [ledger/limits](../../docs/architecture/ledger.md), [ADR-0005](../../docs/architecture/decisions/0005-financial-execution.md), and [financial evidence](../../docs/verification/financial-testing.md).

## Limits

Balances, holds, settlement, zero-fee ledger, market pause, and command deduplication are implemented in memory. No journal, snapshot/replay, gateway, database, or restart guarantee exists. Full-state copying, Vec-backed matching, and full reconciliation prioritize understandable correctness and allocate/copy history. No latency, throughput, zero-allocation, or fault-tolerance claim is made.
