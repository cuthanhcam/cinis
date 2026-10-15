# Requirements

Status: Accepted working specification, 2026-10-13. Accepted describes the implementation baseline, not a delivered guarantee. Cinis is a Rust/.NET trading infrastructure lab for deterministic order execution, financial correctness, durable recovery, and reproducible performance research.

## Actors and workflows

| Actor | Workflow | Boundary |
| --- | --- | --- |
| Research developer | Run deterministic matching scenarios and inspect events/state | In-process core and CLI initially |
| Synthetic trader/bot | Fund, place, retry, cancel, query | Authorized gateway, M2/M3 |
| Administrator | Manage access, market state, synthetic funding | Separate privileges |
| Reviewer/operator | Reconcile, restart, replay, reproduce measurements | Verification tools and runbooks |

## Functional acceptance

FR IDs retain their meanings from [scope](scope-v0.1.md).

| ID | Acceptance criteria | Evidence / delivery |
| --- | --- | --- |
| FR-001 | Exactly one SYNBASE/SYNQUOTE fixture; positive integer ticks/lots | ADR-0002; M1 instrument checks, M2 funding |
| FR-002 | GTC remainder rests; best price then acceptance sequence; resting execution price; only remainder cancelled | Executable MC fixtures; M1 |
| FR-003 | Reserve before matching; settle fills; release improvement/remainder; reconcile per asset | INV-001/002/004; M2 |
| FR-004 | Read keys cannot trade; wrong owner cannot query/cancel; revoked keys fail | Authorization integration; M3 |
| FR-005 | Same account/command ID/payload returns old result; changed payload conflicts | Retry before/after recovery; M2/M4 |
| FR-006 | Durable final response survives supported process crash/restart with same results | Crash matrix; M4 |
| FR-007 | Snapshot/deltas form a consistent stream; detect gaps and resync; isolate slow clients | Integration/failure tests; M5 |
| FR-008 | Clean-start full journey and reproducible benchmark report with raw output | M6 |

M2 now implements in-process FR-001/002/003 and memory-only FR-005: synthetic funding, reservation/settlement/release, ledger reconciliation, and retained command outcomes. Trusted caller permissions are not FR-004 API-key authentication. Persistence/restart parts of FR-005/006 and all network/query projection behavior remain pending. A duplicate order ID rejection is different from command retry behavior.

## Quality requirements

| ID | Requirement | Acceptance |
| --- | --- | --- |
| QR-001 | Correctness and determinism | Identical command sequence gives identical events/state; check quantity and uncrossed-book invariants after every command |
| QR-002 | Durability | Non-durable prototypes say so; durable success only after tested sync boundary, M4 |
| QR-003 | Security | Account ownership enforced in core; scope/revocation verified at gateway, M3 |
| QR-004 | Capacity | Explicit order/event/numeric limits; reject atomically before mutation; never silently drop events |
| QR-005 | Performance | Separate core/durable/end-to-end boundaries; report workload/environment; no target before baseline |
| QR-006 | Operability | Document build/test/demo commands now; verified restore and projection rebuild before release |

## Full acceptance

Use the funding-to-recovery journey in scope-v0.1.md. The funded M2 demo completes the volatile portion, not the restart/recovery portion. Requirements are complete only when linked evidence exists; current delivery status lives in [backlog](../delivery/backlog.md).
