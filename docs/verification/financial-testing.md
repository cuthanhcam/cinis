# Financial execution verification

Status: Verified locally, 2026-10-15, Rust 1.95.0 on Windows/MSVC. Hosted validation is required separately for the exact integration commit. Owner: repository maintainer.

## Evidence layers

| Suite | Evidence |
| --- | --- |
| tests/financial.rs | 24 numeric/scenario/regression tests for funding, reservation, settlement/release, capacity, authorization, retries, market state |
| tests/financial_properties.rs | Four fixed seeds, 256 generated commands each, plus eight initialization funding commands per seed |
| execution module unit tests | Sequence exhaustion and event/posting ordinal capacity bounds |
| Existing matching suites | 17 exact MC fixtures and nine independent model tests remain intact |

Total: 56 Rust tests (54 integration and two unit tests). Generated financial corpus: 1,024 new request attempts, plus retries/conflicts and 32 initial funding commands. Seeds: 11, 29, 2026, 3735928559. Generator uses the fixed SplitMix64 algorithm in the test source. No shrinking/exhaustive fuzzing claim.

## Numeric acceptance

The funded partial-fill test implements the in-memory part of the v0.1 journey: buyer 1,000 quote; seller 10 base; sell 10 at 100; buy 4; exact retry; cancel remaining 6. Final buyer: base 4/quote 600; seller: base 6/quote 400; all holds zero; book empty; seven balanced transfers. Restart/recovery is deliberately absent.

Price improvement tests cover buy 4 at limit 100 against ask 90 (360 paid, 40 released), and partial buy 4 against ask quantity 2 (180 paid, 20 released, 200 held for remaining 2; cancelling releases 200). Incoming sell at 90 against resting bid 100 executes at 100 and leaves the correct buy remainder hold. Multi-fill settles each maker price and improvement separately.

## Failure and identity coverage

- Quote/base overspend rejects without financial mutation; cached insufficient-funds rejection remains the same after later funding.
- Self-trade and output/order capacity reject before visible holds/fills.
- A ledger limit reached after tentative reserve and quote settlement rolls back everything; recipient base/quote limits also roll back all earlier tentative effects.
- Funding account/supply bounds, invalid amounts, account creation capacity, receipt capacity, zero-event capacity, and maximum legal notional are covered.
- Wrong-owner cancellation, unknown order/account, duplicate order ID, malformed envelope IDs, unauthorized commands/retries, changed payload, and account-scoped command IDs are covered.
- Paused markets reject new orders, allow cancel/funding, and never reinstate an old order when a historical response is retried.
- Event and posting IDs use stable sequence/ordinal namespaces; terminal cancel creates no additional postings.

## Generated properties and independent evidence

Every generated attempt is fed to two separate cores and complete responses/state are compared. Admission errors preserve the entire core; business rejects preserve financial/matching state while retaining a decision. Exact retries preserve the entire core, and changed payloads produce DuplicateConflict.

Track synthetic issuance independently from successful fresh Fund commands in the test harness. Compare those totals with summed available/reserved user balances and issued counters after every command. ExecutionCore.check_invariants independently aggregates ledger transfers, checks issuance, matches each balance bucket, matches reservations to open order obligations, verifies matching invariants, and reconciles receipts/posting IDs. Each seed must actually produce accepted orders, trades, and cancellations rather than merely passing rejection-only checks.

The existing map-based differential oracle validates matching separately. There is not a second complete financial engine; deterministic duplicate runs alone are not an independent financial oracle. Exact numerical cases, external funding totals, and ledger aggregation provide complementary evidence, with that limitation explicit.

## Reproduction and remaining guarantees

```sh
cargo test --locked --manifest-path src/engine/Cargo.toml --test financial
cargo test --locked --manifest-path src/engine/Cargo.toml --test financial_properties
cargo test --locked --manifest-path src/engine/Cargo.toml --test financial_properties financial_seed_2026 -- --exact
cargo run --locked --manifest-path src/engine/Cargo.toml --bin cinis-demo
```

Preserve revision, seed, command index, and failing input when investigating generated failures. All evidence is volatile and in-process. Network authentication, client disconnect outcomes, persisted command deduplication, journal replay, projection consistency, and power-loss recovery remain unverified M3/M4/M5 work.
