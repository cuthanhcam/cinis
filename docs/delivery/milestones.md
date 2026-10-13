# Delivery plan

M0 in progress. Documentation bootstrap is a baseline; technical spikes and policy validation remain pending.

| Milestone | Scope / exit criteria | Initial effort |
| --- | --- | --- |
| M0 | Definition, toolchain selection, two spikes, numeric/protocol decisions | 20-35 h |
| M1 | Safe Rust GTC/cancel/partial fills; priority/lifecycle tests; CLI | 45-75 h |
| M2 | Reservations, settlement, ledger, deduplication; financial invariants | 45-80 h |
| M3 | .NET auth/API keys, gateway, projections, end-to-end flow | 45-75 h |
| M4 | Journal, snapshots, replay, consumer recovery, crash matrix | 55-95 h |
| M5 | Market-data snapshot/deltas/resync, diagnostics, runbook drills | 35-60 h |
| M6 | Clean-start demo, benchmark baseline, limitations, v0.1 review | 25-45 h |

These inherited planning hypotheses are not commitments: 270-465 focused hours before contingency, approximately 338-581 with 25%. Weekly capacity is unconfirmed. Re-estimate after M0, M1, and M2; no fixed deadline.

## Current gate and work items

The matching specification gate is accepted in [M0 review](m0-review.md); M0 as a whole remains open for the cross-language spike. The nearest outcome is a specified and correct matching core. First executable flow and fixture coverage do not close M1 until independent reference/generated tests and review are complete.

[Backlog](backlog.md) is the sole work-item source, including dependencies, estimate ranges, acceptance evidence, and status. Do not duplicate detailed tasks here or in GitHub Issues.
