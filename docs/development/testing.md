# Testing

Status: current volatile M2 baseline, verified locally with Rust 1.95.0 on Windows/MSVC. Hosted outcomes must be checked for the exact integration SHA.

## Rust checks and demos

```sh
cargo fmt --manifest-path src/engine/Cargo.toml --check
cargo clippy --locked --manifest-path src/engine/Cargo.toml --all-targets -- -D warnings
cargo test --locked --manifest-path src/engine/Cargo.toml
cargo doc --locked --manifest-path src/engine/Cargo.toml --no-deps
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-matching-demo
```

Current suite: 61 unit/integration Rust tests plus one compiled Rustdoc example, comprising 17 shared matching fixtures, nine independent matching tests, 24 financial scenarios, four generated financial seed tests, five outcome contract tests, and two execution boundary unit tests. All language/toolchain checks and both demos must pass. The default demo is funded; matching-only mode intentionally bypasses financial state.

[Matching fixtures](../../tests/fixtures/matching/README.md) define exact events/final state. The matching oracle covers 8,192 generated commands. [Financial verification](../verification/financial-testing.md) covers numeric holds/settlement/retries/capacity and 1,024 fresh generated financial attempts plus initialization, retries, and conflicts. Seeds and reproduction commands are in the respective reports. This is bounded deterministic generation, not exhaustive fuzzing/shrinking.

## Automation and repository checks

```sh
python -m unittest discover -s ops/tests -v
python ops/scripts/check_docs.py
python ops/scripts/check_naming.py
python ops/scripts/check_workflows.py
python ops/scripts/check_scaffold.py
git diff --check
```

Ops has 16 tests on Windows: seven process behavior tests, four path policy tests, five hosted gate tests. Non-Windows discovery explicitly skips the Windows process class. Process coverage includes spaces/quotes/empty arguments, nonzero exit, missing tool, large dual streams, timeout of owned process/descendants, and public fail-fast with shadowed PowerShell modules.

Windows public runner Verify executes format, lint, build, full Rust tests, documentation/naming, pinned actionlint, and scaffold/ignore checks with logs. Run uses the funded demo; RunMatching preserves the raw matcher. See [runbook](../operations/local-development.md).

## Required boundaries

Financial API tests use trusted in-process Permissions, not API-key authentication. Idempotency/cache and ledger are volatile, not durable. State copying is not a storage snapshot. Journal/crash/replay, client disconnect outcomes, and consumers require their own M3/M4/M5 tests.

[CI procedure](ci.md) requires all four named workflows to pass the exact integration commit before develop is advanced. Linux/Windows engine jobs run both demos. Local pass is necessary but insufficient hosted evidence.
