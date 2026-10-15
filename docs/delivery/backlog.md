# Backlog

This file is the single work-item source. GitHub Issues are not maintained in parallel. Milestones describe outcomes and point here. Status: Todo / In progress / Done / Deferred; Done requires linked evidence.

| ID | Work item | Dependencies | Acceptance / evidence | Estimate | Status |
| --- | --- | --- | --- | --- | --- |
| M0-01 | Review definition and decisions | none | M0 decision register and ADR-0002 | 2-4 h | Done (specification) |
| M0-02 | Requirements and ownership | M0-01 | FR/QR acceptance and sole state owners | 3-6 h | Done (specification) |
| M0-03 | Structured matching examples | M0-01 | Initial state, commands, events, final state in shared fixtures | 3-6 h | Done: 17 shared MC fixtures |
| M0-04 | Cross-language spike | M0-01 | Framing/disconnect/correlation report; resolve protocol questions | 4-8 h | Todo |
| M1-01 | Rust crate and pinned toolchain | M0-01 | Build, rustfmt, Clippy, tests, CI workflow | 2-4 h | Done: crate, pin, lockfile, workflow; local checks pass |
| M1-02 | First matching flow | M1-01, M0-03 | Sell 10, buy 4, cancel 6; demo and invariant checks | 4-8 h | Done: fixture tests and CLI verified locally |
| M1-03 | Matching edge cases | M1-02 | Priority/FIFO/multifill/cancel/invalid/self-trade/capacity fixtures | 4-8 h | Done: fixture tests and CLI verified locally |
| M1-04 | Independent reference and generated streams | M1-03 | differential.rs: 8,192 generated commands plus targeted reference test; verification report | 4-8 h | Done (local evidence) |
| M1-05 | M1 review and estimate refresh | M1-04 | m1-review.md records local evidence/limits; maintainer review and effort calibration pending | 2-4 h | In progress |
| M2-01 | Ledger and financial limits | M1-03 | ledger.md with postings, funding source, reconciliation, account bounds | 3-6 h | Todo |
| M2-02 | Financial state and command deduplication | M2-01 | INV-001/002/004/006, insufficient funds and retry scenarios | Refine at M2 | Todo |
| M3-01 | Gateway/auth/query integration | M0-04, M2-02 | FR-004 and end-to-end orders; independent from Aegis | Refine at M3 | Todo |
| M4-01 | Durability and crash recovery | M2-02 | Storage ADRs and crash matrix | Refine at M4 | Todo |
| M5-01 | Market data and operations | M4-01 | Gap/resync, bounded clients, verified runbooks | Refine at M5 | Todo |
| M6-01 | Full demo and baseline report | M5-01 | Full scope journey; benchmark-environment.md plus raw evidence | Refine at M6 | Todo |

Actual focused effort has not been measured; do not substitute elapsed assistant runtime for engineering estimates. Financial and durability scenarios stay pending until those layers exist.

M1-01 CI workflow is committed; local success is recorded in testing.md. Remote CI execution must be checked separately. M1-04 has local evidence; M1-05 remains open for maintainer review and effort calibration.

## Developer operations

| ID | Work item | Evidence | Status |
| --- | --- | --- | --- |
| OPS-01 | Local task runner and diagnostic logs | ops/scripts, seven integration tests; Verify/Run pass locally | Done |
| OPS-02 | Structure, documentation standard, runbooks, Windows CI | ADR-0003, docs/ops and ops workflow; hosted result tracked separately | Done (implementation) |
| OPS-03 | Persistent service lifecycle, readiness, shutdown, rotation | Add with actual gateway/execution service; docs/ops/service-lifecycle.md | Deferred to M3/M4 |
