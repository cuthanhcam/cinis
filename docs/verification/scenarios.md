# Acceptance scenarios

Status: tests planned. Fresh accounts, sufficient funding, zero fees, one atom per tick/lot unless stated. Makers and takers use different accounts except self-trade tests.

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
