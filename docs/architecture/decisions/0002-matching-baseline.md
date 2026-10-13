# ADR-0002: Explicit matching baseline

Date: 2026-10-13. Status: Accepted implementation baseline; subject to PR review.

## Context and decision

Avoid deciding business rules implicitly while implementing the first book. Use one immutable SYNBASE/SYNQUOTE instrument: one tick is one quote atom per base atom; one lot is one base atom. Wire-independent core values and IDs use u64; price/quantity ranges are 1 through 1,000,000,000 inclusive, with checked u64 multiplication. IDs/account IDs are nonzero. Fractional inputs cannot be represented in this core API; adapters must reject them, never truncate.

Choose best price then accepted-place sequence, resting-order execution price, GTC, zero fees, no amend. Incoming self-trade rejects atomically if the actual preflight fill path reaches the same account; deeper same-account orders outside the fill path do not reject. Cancel requires owner and releases only remaining quantity. Terminal cancellation returns terminal status without another change.

Order IDs are unique globally for the lifetime of this in-memory instance, including terminal orders. Duplicate order ID rejects; transport command retry/deduplication is separate M2 behavior. Rejected placements consume neither ID nor priority. Accepted placements increment sequence; gaps caused by other accepted commands are not required for FIFO correctness.

Default maximum tracked orders, including terminal orders, is 10,000. Per-place event budget is 1,024 including acceptance and fills. Preflight rejects before any mutation when adding a tracked order or producing all events would exceed the configured limit. This intentionally rejects even an immediately filled order when tracked history is full. Limits are explicit laboratory policy, not performance claims. Allocation failure and process abort are outside atomic rejection guarantees.

## Alternatives and consequences

Fixed decimal types and multiple instruments would add scale conversion; deferred until needed. Vec-backed scan-based matching is chosen for an understandable baseline; no complexity or latency optimization is claimed. Terminal history is bounded by admission rather than silently evicted.

M1 has no balances, funding, financial ledger, persistence, replay, gateway, or command-envelope deduplication. ACK means in-memory completion only; restart loses all state. M2 financial limits/ledger details and M4 storage format require additional decisions.
