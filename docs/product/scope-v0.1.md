# v0.1 scope

Status: planned behavior. Detailed policies are a reviewable initial baseline.

| ID | Requirement | Acceptance evidence |
| --- | --- | --- |
| FR-001 | One spot market and synthetic funding | Deterministic instrument/account fixture |
| FR-002 | Limit GTC, partial fills, price-time priority, cancellation | SC-01 through SC-06 |
| FR-003 | Reservation, settlement, balance queries, ledger | SC-07/16 and invariant suite |
| FR-004 | Account-scoped API keys with read/trade scopes | Ownership and scope rejection tests |
| FR-005 | Retry original result without duplicate effects | SC-08/09/15 |
| FR-006 | Journal, snapshots, restart recovery | Crash matrix and state comparison |
| FR-007 | Market-data snapshots/deltas and gap recovery | Slow-client/resync tests |
| FR-008 | CLI demo, diagnostics, baseline benchmark | Clean-start acceptance and evidence report |

## Acceptance journey

Fund buyer with 1,000 quote atoms and seller with 10 base atoms. Sell 10 at 100; buy 4 at 100. Observe fill 4, resting sell 6. Retry the buy with the same command ID: no new trade. Restart: recover identical state. Cancel sell: release 6 base. Buyer holds 4 base and 600 quote; seller holds 6 base and 400 quote. Ledger totals reconcile; book is empty. Fixture: one atom per tick/lot, zero fees.

## Deferred

Multiple markets, real funds, derivatives, market/IOC/FOK orders, amend, nonzero fees, external identity federation, Aegis integration, advanced terminal, blockchain, replication, and low-level optimization.

## Nonfunctional acceptance

Replay reproduces logical state/results. Financial arithmetic is checked and integral. Durable ACK survives supported restart cases. Consumers tolerate duplicates and detect gaps. Benchmarks disclose environment, workload, measurement boundary, and durability policy. No latency threshold is promised before a baseline.
