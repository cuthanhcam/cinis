# M1 matching review

Date: 2026-10-15. Status: local technical evidence recorded; maintainer review and effort calibration pending.

## Delivered

Safe-Rust GTC matching, price-time priority, resting execution price, partial fills, cancellation/terminal lookup, ownership, global instance-lifetime order IDs, atomic self-trade/capacity rejects, and CLI. Pinned compiler and lockfile. Seventeen exact fixtures plus nine reference/generated tests pass, including 8,192 generated commands. Windows ops runner adds logged verification and seven process-behavior tests.

## Evidence and structure

See [testing commands](../development/testing.md), [differential method](../verification/differential-testing.md), [ops runbook](../operations/local-development.md), and [ADR-0002](../architecture/decisions/0002-matching-baseline.md). Production matching code is unchanged by the independent reference test. No throughput or latency conclusion was drawn from test duration.

## Limits and unresolved questions

M1 is volatile, single-market, matching-only. The straightforward data structures allocate and scan/sort. Terminal history counts against the tracked-order cap. Numeric/account limits for financial state, ledger postings and aggregate overflow policy must be designed before M2. Protocol framing/canonicalization remains the M0 cross-language spike. Durability format and sync policy remain M4 work.

## Forecast and next gate

M1-04 now has local evidence. M1-05 remains open for maintainer review and actual-effort calibration; focused engineering effort was not measured. Keep the existing M2 effort range of 45-80 hours as an uncalibrated hypothesis, not a revised empirical estimate or completion date. Next implementation work is M2-01: ledger specification, financial account bounds, postings, reservation/settlement transaction boundary, and reconciliation examples before financial code.
