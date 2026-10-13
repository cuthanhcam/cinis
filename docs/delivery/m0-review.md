# M0 review and decision register

Reviewed 2026-10-13. Accepted means the working implementation baseline, not external approval or proof that code implements the guarantee.

| Decision | State | Evidence / next action |
| --- | --- | --- |
| Single spot sandbox, synthetic funds | Accepted | Scope and requirements |
| Price-time and resting execution price | Accepted | BR-002/003, ADR-0002 |
| Integer tick/lot representation and matching limits | Accepted | ADR-0002 |
| GTC, cancel, self-trade preflight | Accepted | BR-004/007/009/012 |
| Reservation, settlement, zero fees | Accepted specification | Financial model; M2 implementation pending |
| Single financial owner; projections never authorize risk | Accepted | State ownership |
| Full v0.1 acceptance journey | Accepted | Scope; not yet executable |
| Initial prototype ACK has no durability | Accepted | ADR-0002 and durability |
| Durable final response requires sync-before-apply | Accepted target | M4 storage-specific evidence pending |
| Financial numeric/account limits and posting schema | Proposed | Resolve before M2; write ledger.md |
| Network framing bytes, bounds, canonicalization, error encoding | Proposed | .NET/Rust spike before M3 |
| Snapshot/journal format, sync APIs, retention | Proposed | ADRs and crash tests before M4 |
| Zero-allocation, advanced IPC, replication, blockchain | Deferred | Post-v0.1 research |

M0 is not globally complete: the cross-language spike and later-stage contracts remain unresolved. The matching specification gate is accepted for starting M1. Weekly availability is still unconfirmed; retain estimate ranges without calendar promises.

## Matching evidence

The first Rust matching flow and 17 fixture tests are implemented; see [baseline report](../../research/matching-baseline.md). Rust is pinned to 1.95.0. This closes the small Rust feasibility question, not the cross-language spike or the full M1 correctness gate.
