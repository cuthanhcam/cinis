# Domain glossary

| Term | Meaning and example |
| --- | --- |
| Instrument | Base/quote pair, such as SYNBASE/SYNQUOTE |
| Base / quote | Traded asset / pricing asset; 4 base for 400 quote |
| Bid / ask | Buy / sell; bid 99 and ask 100 |
| Spread | Best ask minus best bid; 100 minus 99 = 1 |
| Tick / lot | Integer price / quantity step; fixture uses one atom each |
| Price level | Orders sharing price, such as two asks at 100 |
| Maker / taker | Resting order / incoming order that crosses it |
| GTC | Good till cancelled; unfilled remainder rests |
| Order | Instruction to buy/sell; sell 10 at 100 |
| Trade / fill | Matched quantity between orders; 4 at 100 |
| Execution report | Acceptance, rejection, fill, or cancel result/event |
| Reservation | Held funds; buy 4 at 100 holds 400 quote |
| Available / reserved | Spendable / held; total equals their sum |
| Settlement | Internal asset transfers caused by a trade |
| Ledger | Ordered asset movements for reconciliation |
| Sequence | Execution-owned command order; lower acceptance sequence wins FIFO |
| Idempotency | Same account/key/payload repeats one result without new effects |
| Projection | Rebuildable read model that can lag execution |
| Snapshot | Consistent execution state at a journal sequence |
