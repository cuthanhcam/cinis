# Trading rules

Status: proposed v0.1 baseline. Policy changes require updated acceptance cases and an ADR when contracts change.

| ID | Rule |
| --- | --- |
| BR-001 | Positive integral ticks/lots; explicit market limits required before implementation acceptance. |
| BR-002 | Best price first; FIFO within a level uses execution acceptance sequence, never client clocks. |
| BR-003 | Buy crosses asks at/below its limit; sell crosses bids at/above its limit. Execute at resting price. |
| BR-004 | GTC remainder rests. No amendment; cancel/resubmit loses priority. |
| BR-005 | Reserve buy limit notional in quote or sell quantity in base before matching. Insufficient funds rejects atomically. |
| BR-006 | Settle every fill; release buy price improvement immediately. Fees are zero. |
| BR-007 | Cancel only remaining quantity and release its hold. Filled quantity is final. |
| BR-008 | Deduplicate by account ID and command ID. Same canonical payload returns original result; changed payload rejects as conflict. |
| BR-009 | Reject the entire incoming order if preflight determines it would self-match. No earlier fills or holds occur. |
| BR-010 | Capacity, arithmetic, and output-budget checks succeed before mutation. Exhaustion rejects atomically. |
| BR-011 | Paused markets reject new orders and permit authorized cancels. Market-state changes are sequenced. |
| BR-012 | Check cancel ownership. Unknown order rejects; terminal owned order returns its terminal state without new effects. |

Preflight and apply observe the same single-writer state. Deduplication precedes financial mutation and persists for the entire v0.1 journal history; expiry/compaction requires a future contract. Funding is a privileged synthetic adjustment command with idempotency and ledger entries. Trading-effective market configuration is immutable while active orders exist.
