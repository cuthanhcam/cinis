# Rust execution core

One dependency-free safe-Rust crate, pinned to Rust 1.95.0 at repository root. Matching uses a Vec-backed scan/sort baseline for clarity, not low-latency claims. Commands are typed in-process values; no async runtime or database exists inside matching.

From repository root:

```sh
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
cargo test --locked --manifest-path src/engine/Cargo.toml
```

Core API: Engine::execute(Command) returns ordered events or a rejection. Engine::orders exposes immutable tracked state; Engine::check_invariants checks matching consistency. Price/quantity are bounded integer ticks/lots. Rejections do not consume order IDs or acceptance sequence. Duplicate order IDs reject even after cancellation; command retries are future M2 behavior.

Tests consume [shared fixtures](../../tests/fixtures/matching/README.md). [ADR-0002](../../docs/architecture/decisions/0002-matching-baseline.md) defines units, policies, capacity, and limitations. Accepted means volatile completion only. Financial risk, ledger, journal/replay, market pause, protocol, and authentication are not implemented.
