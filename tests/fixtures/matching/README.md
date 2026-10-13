# Matching fixtures

These files are both the structured specification and the integration-test input. Every file starts with an empty in-memory engine and default limits, unless LIMITS overrides them before commands. No funds or external state are required.

## Grammar

- `LIMITS <tracked-orders> <events-per-command>` overrides default 10000/1024.
- `P <order-id> <account-id> <B|S> <price-ticks> <quantity-lots> => <expected>` submits GTC.
- `C <order-id> <account-id> => <expected>` cancels.
- Expected outputs: `A <id>` acceptance; `T <maker> <taker> <price> <quantity>` trade; `C <id> <quantity>` cancellation; `X <id> <status>` terminal lookup; `! <Reject>` rejection.
- Multiple events are separated by ` | ` in exact order.
- `S <id> <account> <Buy|Sell> <price> <original> <remaining> <filled> <cancelled> <status>` declares every final tracked order, in acceptance order. No S rows means no tracked orders.

The runner checks exact outputs and final records, rejection atomicity including priority state, matching invariants after every command, and deterministic repeat execution. MC IDs refer to filenames. Full-system SC requirements remain separate; insufficient funds and transport-command retry are M2, not simulated here.
