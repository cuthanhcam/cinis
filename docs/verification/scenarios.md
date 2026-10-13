# Acceptance scenarios

Status: full-system scenarios planned; matching-only MC fixtures are executable. Fresh accounts, sufficient funding, zero fees, one atom per tick/lot unless stated. Makers and takers use different accounts except self-trade tests.

| ID | Input | Expected result |
| --- | --- | --- |
| SC-01 | Sell 10 at 100; buy 4 at 100; cancel sell | Fill 4; release 6 base; empty book |
| SC-02 | A asks 2 at 101; B asks 2 at 100; C buys 3 at 101 | B fills 2 at 100; A fills 1 at 101 |
| SC-03 | A asks 2 at 100 before B asks 2; C buys 3 | A fills 2 then B fills 1 |
| SC-04 | Ask 2 at 101; buy 2 at 100 | Both rest; no trade |
| SC-05 | Ask 2 at 90; buy 4 at 100 | Fill 2 at 90; buy remaining 2 rests |
| SC-06 | Fully fill then cancel owned order | Terminal Filled result; no reversal |
| SC-07 | Buyer with 100 quote sends two buys costing 80 each | First holds 80; second rejects; available 20 |
| SC-08 | Retry same account/ID/payload | Original result; no extra financial effect |
| SC-09 | Reuse command ID with changed quantity | Duplicate conflict; no mutation |
| SC-10 | Zero quantity, negative price, or notional overflow | Invalid/overflow rejection; no mutation |
| SC-11 | A rests sell then submits crossing buy | Entire incoming order rejects; original remains |
| SC-12 | Full order capacity or inadequate event budget | Atomic rejection; state unchanged |
| SC-13 | Pause; place order; cancel owned resting order | New order rejects; cancel succeeds |
| SC-14 | Wrong account cancels order | Authorization rejection; no mutation |
| SC-15 | Journal committed, response lost; restart/retry | One execution; original result recovered |
| SC-16 | Buy 4 at 100 against sell 4 at 90 | Pay 360; release 40 improvement; no buy hold |

Add generated command sequences and compare against a simple reference implementation. Record actual test paths as implementation arrives; these are specifications only.

## Executable matching examples

The [shared fixture format and files](../../tests/fixtures/matching/README.md) are the source for initial state, commands, exact expected events, and final state. They are consumed directly by src/engine/tests/scenarios.rs; do not duplicate their expected outputs here.

| Matching fixture | Rule / full-system relation |
| --- | --- |
| MC-01 partial/cancel; MC-02 exact; MC-03 noncrossing | BR-003/004/007; matching portion of SC-01/04 |
| MC-04 price/multifill; MC-05 ask FIFO; MC-06 bid priority; MC-16 bid FIFO | BR-002/003; SC-02/03 |
| MC-07 incoming remainder | BR-003/004; SC-05 |
| MC-08 terminal/unknown cancellation | BR-012; SC-06 |
| MC-09 units/bounds/IDs | BR-001; matching validation portion of SC-10 |
| MC-10 duplicate order ID | ADR-0002; not SC-08 command deduplication |
| MC-11 cancel ownership | BR-012; core portion of SC-14, not gateway authentication |
| MC-12 atomic self-trade; MC-13 unreachable self-trade | BR-009; SC-11 |
| MC-14 tracked capacity; MC-15 event budget; MC-17 zero event budget | BR-010; SC-12 |

Insufficient funds (SC-07), transport-command retries (SC-08/09), pause (SC-13), financial release (SC-16), and recovery (SC-15) remain later-layer work.
