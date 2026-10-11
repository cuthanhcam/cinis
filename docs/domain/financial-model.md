# Financial model

## Units

Integer price ticks, quantity lots, asset atoms. Demo fixture: one lot = one base atom; one tick = one quote atom per base atom. Quote notional = price ticks times quantity lots. Arbitrary decimal scales require an explicit conversion contract before use.

Check all arithmetic, parsing, and conversion. Reject out-of-range values before mutation. No floating point or implicit rounding. Numeric widths and market/account limits remain M0 spike decisions, required before M1 acceptance.

## Reservation and settlement

Buy limit P, quantity Q: hold P times Q quote. Sell Q: hold Q base. Fill q at resting price p: buyer receives q base and pays p times q quote; seller transfers q base and receives p times q quote. Reduce buy hold by P times q; release price improvement (P minus p) times q. Sell hold decreases by q. Remaining hold equals limit times remaining quantity for buys or remaining base for sells.

Example: buyer with 1,000 quote buys 4 at limit 100 against ask 90. Hold 400; pay 360; release 40; final quote 640 and base +4. Zero fees.

## Ledger

Rust applies book, balances, holds, deduplication, and balanced per-asset ledger movements as one logical transition. Stable event identities derive from command sequence plus ordinal. PostgreSQL stores rebuildable projections, not a second financial writer.

Funding transfers from an explicitly tracked synthetic issuance source into an account. Conservation includes that source or compares account totals against recorded funding. Trades and cancels create no assets. Nonzero fees would require fee accounts and rounding rules.

Commands plus deterministic replay reconstruct authoritative ledger state. Consumers persist their cursor with projection changes atomically and ignore already-applied event IDs.
