# Differential and generated matching tests

Status: Verified locally, 2026-10-15, Rust 1.95.0 on Windows/MSVC. Owner: repository maintainer.

## Independent implementation

The production baseline uses a Vec of tracked orders, sorts crossing candidates, preflights the whole fill path, then applies changes. The test-only reference in src/engine/tests/differential.rs uses BTreeMap order storage and a price/sequence index. It repeatedly selects the best resting order and mutates a speculative transaction, committing only on success. It never calls Engine matching, cancellation, or invariant methods to decide expected results.

The reference shares command/event/order record types to compare observable behavior, but has its own validation, selection, lifecycle accounting, and terminal response logic. Sharing the specification is intentional; a reference model is not proof that the specification itself is correct. Fixed MC examples remain independent expected-output anchors.

## Reproducible corpus

Eight seed tests each run 256 commands under four (tracked-orders, event-budget) configurations: (0,32), (96,0), (96,2), (256,32). Total generated commands: 8,192. A ninth targeted reference test covers partial-fill cancellation, wrong owner, repeated/terminal cancel, unknown order, and duplicate terminal ID. Seventeen structured fixture tests remain in the suite: 26 integration tests total.

Generator: fixed SplitMix64 algorithm with wrapping arithmetic, no external RNG. Seeds: 1, 7, 42, 2026, 65535, 123456789, 9223372036854775808, 18446744073709551615. Commands vary sides, price/quantity boundaries, invalid IDs, account ownership, duplicates, cancels, and capacity. The corpus is bounded and deterministic, not exhaustive fuzzing or shrinking property tests.

After every generated command compare exact result/events with the reference, complete tracked-order records, and a second Engine fed the identical stream. Check Engine invariants. On rejection compare the complete pre/post Engine including private sequencing state. Panic messages include seed, zero-based step, limits, and the offending command for event mismatches.

## Reproduction

```sh
cargo test --locked --manifest-path src/engine/Cargo.toml --test differential
cargo test --locked --manifest-path src/engine/Cargo.toml --test differential seed_42 -- --exact
```

To extend a failing prefix, preserve the seed, generator implementation/revision, limits, and step, then promote a minimized regression to a structured fixture. Future randomized fuzzing must record seed/input, and future optimizations must continue comparing with this test-only model.

## Boundaries

These checks cover matching state only. No financial balances, ledger, command-envelope deduplication, journal, projection, network, or power-loss recovery is verified. Numeric overflow beyond admitted price/quantity bounds cannot be reached through the current API; sequence exhaustion is not exercised by this bounded corpus.
