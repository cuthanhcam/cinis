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

## Next M0 work

Review self-trade, fee, deduplication, and funding policies. Run safe-Rust matching and .NET/Rust round-trip spikes, each timeboxed to 4-8 hours. Select/pin toolchains alongside real manifests and build/test jobs. Resolve numeric widths, market/account limits, framing bounds, errors, and capacity preflight semantics.

## M1 backlog

| ID | Work | Acceptance | Estimate |
| --- | --- | --- | --- |
| M1-01 | Typed prices, quantities, IDs, checked arithmetic | Zero/overflow/bounds reject tests | 3-6 h |
| M1-02 | Orders and price levels | Stable FIFO/deterministic iteration | 4-8 h |
| M1-03 | GTC insertion/crossing | SC-02/03/04/05 | 4-8 h |
| M1-04 | Cancel/terminal lookup | SC-01/06 with quantity accounting | 3-6 h |
| M1-05 | Self-trade/capacity preflight | SC-11/12 atomic rejects | 4-8 h |
| M1-06 | CLI/reference model | First scenario and seeded generated cases | 4-8 h |
| M1-07 | Review and limits | Reproduction guide, actual effort, re-estimate | 2-4 h |

Task ranges exclude some learning/integration covered by the milestone envelope. Split tasks exceeding eight hours. Funding/reservation is M2; matching-only M1 fixtures must label their financial bypass and cannot claim the complete acceptance journey.
