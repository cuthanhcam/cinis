# Testing

Status: Implemented matching-only checks. On 2026-10-13, Rust 1.95.0 on Windows/MSVC passed rustfmt, Clippy with warnings denied, 17 fixture integration tests, and the CLI demo. Hosted CI results are separate from this local evidence.

From the repository root, with the pinned Rust toolchain installed:

```sh
cargo fmt --manifest-path src/engine/Cargo.toml --check
cargo clippy --locked --manifest-path src/engine/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src/engine/Cargo.toml
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
python scripts/check_docs.py
git diff --check
```

Matching fixtures live under tests/fixtures/matching. Every fixture starts empty, declares limits, executes commands with ordered expected output, and specifies all final order records. The runner checks invariants after every command and unchanged state on rejection. This is matching-only coverage: financial state, command retry/deduplication, network, and recovery remain unimplemented.

Independent reference-model and seeded generated tests remain backlog M1-04. There is no claimed property-generator seed yet. Financial and crash tests must be added with their implementation layers, not simulated by matching fixtures.

Fixtures cover both bid/ask price priority and FIFO, exact/partial/multiple fills, GTC remainder, terminal/unknown cancellation, bounds/invalid IDs, duplicate order IDs, cancel ownership, atomic self-trade rejection, unreachable self-trade, and tracked-order/event capacities. All fixtures rerun from the same empty state to check deterministic results.
