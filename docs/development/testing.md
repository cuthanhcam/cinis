# Testing

Status: Implemented matching-only checks. On 2026-10-13, Rust 1.95.0 on Windows/MSVC passed rustfmt, Clippy with warnings denied, 17 fixture integration tests, and the CLI demo. Hosted CI results are separate from this local evidence.

From the repository root, with the pinned Rust toolchain installed:

```sh
cargo fmt --manifest-path src/engine/Cargo.toml --check
cargo clippy --locked --manifest-path src/engine/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src/engine/Cargo.toml
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
python ops/scripts/check_docs.py
python ops/scripts/check_naming.py
git diff --check
```

Matching fixtures live under tests/fixtures/matching. Every fixture starts empty, declares limits, executes commands with ordered expected output, and specifies all final order records. The runner checks invariants after every command and unchanged state on rejection. This is matching-only coverage: financial state, command retry/deduplication, network, and recovery remain unimplemented.

Independent map-based reference and fixed-seed generated tests now implement M1-04. See [differential testing](../verification/differential-testing.md) for algorithm, all eight seeds, limits, and 8,192-command corpus. Financial and crash tests must be added with their implementation layers, not simulated by matching fixtures.

Fixtures cover both bid/ask price priority and FIFO, exact/partial/multiple fills, GTC remainder, terminal/unknown cancellation, bounds/invalid IDs, duplicate order IDs, cancel ownership, atomic self-trade rejection, unreachable self-trade, and tracked-order/event capacities. All fixtures rerun from the same empty state to check deterministic results.

## Local ops automation

See [ops runbook](../operations/local-development.md). On Windows PowerShell 5.1, the public Verify task runs format, lint, build, matching tests, and documentation checks with per-step logs. Run automation integration tests with:

```sh
python -m unittest discover -s ops/tests -v
```

On 2026-10-15, seven process tests passed: quoting/spaces/empty arguments, native nonzero code, missing executable, large dual-stream output, parent timeout, descendant timeout cleanup, and public fail-fast behavior. Verify and demo also completed locally. Hosted Windows CI is configured separately; no hosted outcome is inferred from local success.

On 2026-10-15, the Rust suite passed 26 integration tests (17 fixtures, eight generated-seed tests, one targeted reference test), with rustfmt and Clippy passing. This adds bounded generated properties and differential evidence; it does not claim exhaustive fuzzing.

The naming/layout migration adds four portable path-policy regression tests. The ops unittest suite now has 11 tests total (seven process tests plus four naming tests), passing locally on 2026-10-15. Verify also executes both documentation and naming checks from ops/scripts/.

## Hosted CI and scaffold gates

The ops test suite now includes five exact-commit CI-gate regression tests (16 tests total on Windows). The public Verify task also checks pinned actionlint and scaffold/ignore contracts. See [CI procedure](ci.md) for the separate hosted gate; passing locally does not authorize an untested develop update. Host-native PowerShell module loading is regression-tested with an incompatible Utility module in the inherited module path.
