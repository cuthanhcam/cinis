# Testing

Status: commands for the first matching crate; executable evidence is tracked in backlog.md.

From the repository root, with the pinned Rust toolchain installed:

```sh
cargo fmt --manifest-path src/engine/Cargo.toml --check
cargo clippy --manifest-path src/engine/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path src/engine/Cargo.toml
cargo run --manifest-path src/engine/Cargo.toml --bin cinis-demo
python scripts/check_docs.py
git diff --check
```

Matching fixtures live under tests/fixtures/matching. Every fixture starts empty, declares limits, executes commands with ordered expected output, and specifies all final order records. The runner checks invariants after every command and unchanged state on rejection. This is matching-only coverage: financial state, command retry/deduplication, network, and recovery remain unimplemented.

Independent reference-model and seeded generated tests remain backlog M1-04. There is no claimed property-generator seed yet. Financial and crash tests must be added with their implementation layers, not simulated by matching fixtures.
