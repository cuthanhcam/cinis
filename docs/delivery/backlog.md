# Backlog

This file is the single work-item source. GitHub Issues are not maintained in parallel. Milestones describe outcomes and point here. Status: Todo / In progress / Done / Deferred; Done requires linked evidence.

| ID | Work item | Dependencies | Acceptance / evidence | Estimate | Status |
| --- | --- | --- | --- | --- | --- |
| M0-01 | Review definition and decisions | none | M0 decision register and ADR-0002 | 2-4 h | Done (specification) |
| M0-02 | Requirements and ownership | M0-01 | FR/QR acceptance and sole state owners | 3-6 h | Done (specification) |
| M0-03 | Structured matching examples | M0-01 | Initial state, commands, events, final state in shared fixtures | 3-6 h | In progress |
| M0-04 | Cross-language spike | M0-01 | Framing/disconnect/correlation report; resolve protocol questions | 4-8 h | Todo |
| M1-01 | Rust crate and pinned toolchain | M0-01 | Build, rustfmt, Clippy, tests, CI workflow | 2-4 h | In progress |
| M1-02 | First matching flow | M1-01, M0-03 | Sell 10, buy 4, cancel 6; demo and invariant checks | 4-8 h | In progress |
| M1-03 | Matching edge cases | M1-02 | Priority/FIFO/multifill/cancel/invalid/self-trade/capacity fixtures | 4-8 h | In progress |
| M1-04 | Independent reference and generated streams | M1-03 | Reproducible seed; compare events and logical state | 4-8 h | Todo |
| M1-05 | M1 review and estimate refresh | M1-04 | Evidence, limits, unresolved questions, actual effort | 2-4 h | Todo |
| M2-01 | Ledger and financial limits | M1-03 | ledger.md with postings, funding source, reconciliation, account bounds | 3-6 h | Todo |
| M2-02 | Financial state and command deduplication | M2-01 | INV-001/002/004/006, insufficient funds and retry scenarios | Refine at M2 | Todo |
| M3-01 | Gateway/auth/query integration | M0-04, M2-02 | FR-004 and end-to-end orders; independent from Aegis | Refine at M3 | Todo |
| M4-01 | Durability and crash recovery | M2-02 | Storage ADRs and crash matrix | Refine at M4 | Todo |
| M5-01 | Market data and operations | M4-01 | Gap/resync, bounded clients, verified runbooks | Refine at M5 | Todo |
| M6-01 | Full demo and baseline report | M5-01 | Full scope journey; benchmark-environment.md plus raw evidence | Refine at M6 | Todo |

Actual focused effort has not been measured; do not substitute elapsed assistant runtime for engineering estimates. Financial and durability scenarios stay pending until those layers exist.
