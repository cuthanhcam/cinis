# Verification strategy

Status: Accepted plan. Implemented test locations and commands are recorded in [testing](../development/testing.md); future layers below are not claimed as delivered.

| Layer | Purpose | Evidence and gate |
| --- | --- | --- |
| Unit/scenario | Priority, fills, cancel, validation, capacity, ownership | Shared structured fixtures; M1 |
| Property | Quantity conservation, non-crossed book, reject atomicity, determinism across seeded streams | Invariant checks per fixture and generated command; fixed-seed corpus implemented |
| Differential | Main core versus deliberately simple independent reference | Required before data-structure optimization; map-based independent model and fixed-seed corpus implemented |
| Financial | No overspend; reservation equals obligations; ledger reconciliation | Implemented volatile M2 evidence; required before network trading |
| Protocol integration | Framing, correlation, disconnect, unknown outcomes | .NET/Rust spike and M3 |
| Crash/replay | Inject failure around append/sync/apply/respond; compare results and canonical state | M4 durability gate |
| Projection/market data | Duplicates, cursor atomicity, gaps, resync, slow clients | M4/M5 |
| Performance | Reproduce raw results with boundaries and environment | M6 baseline; optimization later |

Each fixture defines initial empty state/configuration, command, ordered expected events or rejection, and exact final order state. Assert quantity identities and no crossed resting book after every command, and unchanged logical state on rejection. Fixture IDs MC-* distinguish matching evidence from SC-* full-system requirements.

No test result implies a stronger layer: deterministic matching does not prove financial correctness, and a process restart does not prove power-loss durability. Preserve seed and failing command stream for generated failures. Update FR/BR/INV links when semantics change.

Current differential evidence and limitations are recorded in [differential testing](differential-testing.md). Shrinking/fuzzing frameworks are not implemented.

Financial checks now have separate [M2 evidence](financial-testing.md): exact amounts, external funding totals, ledger reconstruction, held-order obligations, late-failure rollback, and retained results. Network/crash layers remain planned.
