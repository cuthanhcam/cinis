# State ownership

Status: Rust financial execution ownership implemented in memory, 2026-10-15. Management, projections, and durable recovery remain planned.

| State | Sole authority / writer | Durable source / consumer |
| --- | --- | --- |
| Execution sequence and accepted order priority | Rust execution owner | Journal in M4; memory in current M2 |
| Orders, remaining quantity, terminal lookup | Rust execution owner | Snapshot/journal; PostgreSQL query projection |
| Available/reserved balances and reservation ownership | Rust execution owner | Snapshot/journal; balance projection |
| Settlement and authoritative ledger postings | Rust execution owner | Deterministically replayed commands; ledger projection |
| Deduplication key, canonical payload, original result | Rust execution owner | Snapshot/journal; gateway forwards retries |
| Trading-effective instrument/market state | Rust sequenced configuration | Execution history; admin submits changes |
| Identity, API-key hash, scopes, revocation | .NET management | Management database; gateway checks access |
| Administrative descriptive metadata | .NET management | Management database |
| Query orders/trades/balances and applied cursor | Projection consumer | Rebuildable PostgreSQL rows |
| Market-data snapshot/delta cursor | Market-data consumer | Recoverable execution event stream |
| Workload seed and benchmark output | Research toolkit | Versioned fixtures and report artifacts |

## Ledger and balances

Ledger postings record per-asset movements; execution balances are the incrementally maintained view used for risk. Both change within the same deterministic execution transition. Reconciliation independently aggregates postings and compares them to execution balances/holds, including the synthetic funding source. PostgreSQL balance and ledger rows are asynchronous copies, never spending authority.

A projection consumer applies event IDs and advances its cursor atomically in its own database transaction. It can lag, restart, and receive duplicates. Query responses expose the applied cursor. A projection rebuild must never overwrite engine balances.

## Trust and mutation boundaries

Gateway authentication does not replace engine ownership validation. The gateway cannot read a balance and approve spending independently. Admin funding and market changes also enter the sequencer. No two components may independently mutate one financial state. Current M2 exposes a trusted in-process API with explicit permissions and provides no network authentication guarantee. ExecutionCore privately owns its matcher/ledger; Engine remains a separate financial-bypass primitive for matching research.
