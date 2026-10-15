# Invariants and current evidence

Status: matching and volatile financial evidence implemented. Durable/network/projection guarantees remain planned.

| ID | Invariant | Current verification |
| --- | --- | --- |
| INV-001 | Available/reserved balances never negative | Checked ledger debits; insufficient quote/base tests; generated financial streams |
| INV-002 | Trades conserve assets; funding explicitly accounted | Ledger bucket aggregation, issuance source, independent funding totals, exact numerical cases |
| INV-003 | Original = filled + remaining + cancelled | Matching checks after every fixture/generated command and financial commit |
| INV-004 | Holds equal remaining obligations | ExecutionState order-obligation aggregation; partial/multifill/improvement/cancel cases |
| INV-005 | Price-time priority and resting execution price | MC fixtures and independent matching oracle; funded execution price cases |
| INV-006 | Duplicate command has at most one effect | Exact successful/rejected retries; full-core equality; conflicts; permission recheck; cache capacity |
| INV-007 | Replay restores logical state/results | Pending M4; volatile duplicate runs are not persisted replay evidence |
| INV-008 | Rejects do not mutate financial/book state | State equality on business reject, including late settlement failure; admission preserves entire core |
| INV-009 | Durable result remains recoverable | Pending M4; no durable ACK exists |
| INV-010 | Projections apply event effects once and expose lag | Pending consumers/M4/M5; stable IDs alone do not implement projections |

Business rejects intentionally retain one decision/cache entry and consume one command sequence; INV-008 excludes that metadata and applies to financial/order/ledger/market state. Admission errors and exact retries preserve the entire core. Event/posting identities are stable; returning historical events on retry does not authorize applying them twice.

See [matching method](differential-testing.md), [financial method and test mapping](financial-testing.md), [ledger semantics](../architecture/ledger.md), and [testing commands](../development/testing.md). Matching and financial checks run together in cargo test. Correctness evidence does not imply low latency, exhaustive fuzzing, network security, or crash safety.
