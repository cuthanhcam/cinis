# Invariants and verification strategy

| ID | Invariant | Planned evidence |
| --- | --- | --- |
| INV-001 | Available/reserved balances never negative | Checked transitions and generated workloads |
| INV-002 | Trades conserve assets; funding explicitly accounted | Per-asset ledger reconciliation |
| INV-003 | Original = filled + remaining + cancelled | Lifecycle properties |
| INV-004 | Holds equal remaining obligations | Partial fill/improvement/cancel tests |
| INV-005 | Price-time priority and resting execution price | Reference model and SC-02/03/16 |
| INV-006 | Duplicate command has at most one effect | Retry/conflict/crash tests |
| INV-007 | Replay restores logical state/results | Snapshot/journal canonical comparison |
| INV-008 | Rejects do not mutate financial/book state | Before/after state comparisons |
| INV-009 | Durable result remains recoverable | Journal/apply/respond crash matrix |
| INV-010 | Projections apply event effects once and expose lag | Duplicate delivery, gaps, restart |

Use table-driven unit tests, seeded generated sequences, a simple reference model, cross-process integration, then crash tests. Compare canonical logical state rather than memory layout. Replay includes funding, market-state changes, and deduplication.

M1 proves matching; M2 financial and duplicate behavior; M3 authorization/protocol; M4 recovery. Matching evidence now exists in src/engine/tests/scenarios.rs and the shared MC fixtures. Financial, command-deduplication, and recovery invariants remain unverified.

## Current matching evidence

| Invariant | Implemented evidence | Remaining |
| --- | --- | --- |
| INV-003 | Quantity conservation checked after every fixture command | Generated streams, M1-04 |
| INV-005 | MC-04/05/06/16 price/FIFO; MC-07 resting price | Independent reference comparison |
| INV-008 | Engine snapshot equality on every rejected fixture command | Financial reject atomicity in M2 |
| INV-001/002/004/006/007/009/010 | Not implemented | Financial, retry, recovery, projections |
