# Execution ledger and financial state

Status: Accepted M2 implementation baseline, 2026-10-15. Owner: repository maintainer. Scope: one synthetic spot market, in-memory execution; no persistence or external accounting integration.

## Units and bounds

Use the ADR-0002 instrument: one lot = one base atom; one price tick = one quote atom per base atom. Prices/quantities remain positive u64 through 1,000,000,000. Notional is checked u64 multiplication. Each account's available plus reserved balance per asset is bounded at 1,000,000,000,000,000,000 atoms. Total issued supply per asset is bounded at 9,000,000,000,000,000,000 atoms. A synthetic funding command adds 1 through the account limit. All updates use checked integer arithmetic; no rounding or floats.

Default capacities: 10,000 tracked orders, 1,024 result events per command, 10,000 funded accounts, 100,000 ledger transfers, 10,000 retained command results. Terminal orders and rejected business results count toward their respective history limits. No eviction/expiry or reset-to-reclaim feature exists. Capacity exhaustion rejects a new command atomically; at receipt capacity only existing authorized identical retries remain available. Choose a fresh instance for an independent lab run; do not mistake that for recovery.

## Ledger accounts and postings

For each asset, buckets are Available(account), Reserved(account), and a synthetic Issuance source. A LedgerTransfer contains a stable (execution sequence, per-command ordinal) identity, asset, source bucket, destination bucket, and positive amount. One transfer represents two balanced legs: source decreases, destination increases. Issuance is permitted to be negative in reconciliation and equals the negative total synthetic supply; user buckets never go negative.

Balances are incrementally maintained execution views of these transfers. Independently aggregating the ledger must reproduce every available/reserved bucket exactly, including the issuance source. PostgreSQL, when introduced, receives projections; it does not own this ledger or approve spending. No fee bucket is necessary while fees are zero; nonzero fees require a new rule and posting contract.

## Posting examples

| Action | Asset | From | To | Amount |
| --- | --- | --- | --- | --- |
| Fund buyer | Quote | Issuance | Available(buyer) | 1,000 |
| Reserve buy 4 at limit 100 | Quote | Available(buyer) | Reserved(buyer) | 400 |
| Reserve sell 10 | Base | Available(seller) | Reserved(seller) | 10 |
| Fill 4 at 90 | Quote | Reserved(buyer) | Available(seller) | 360 |
| Release buy improvement | Quote | Reserved(buyer) | Available(buyer) | 40 |
| Settle sold base | Base | Reserved(seller) | Available(buyer) | 4 |
| Cancel remaining sell 6 | Base | Reserved(seller) | Available(seller) | 6 |

A resting buy also releases its own limit-price obligation when filled by an incoming sell. Every fill transfers both assets, and every remaining hold equals the remaining order obligation. Price improvement never remains stranded. Zero-amount improvement produces no ledger record. Fully filled or already cancelled orders retain no hold; repeated terminal cancel produces a terminal report without new postings.

## Single-owner transaction boundary

ExecutionCore owns the matcher, account balances, ledger, market state, command sequence, and result cache. For a new admitted command it clones the financial/matching state into a tentative transaction. Matching is planned on an isolated matcher, funds are reserved, fills/cancel are posted, and invariants/reconciliation are checked before the state becomes visible. Any business rejection discards all tentative order, balance, posting, account-creation, issuance, and market effects, including late recipient-limit or ledger-capacity failure.

Cloning and full reconciliation favor an understandable correctness baseline. They allocate and copy history; this design makes no throughput, latency, or zero-allocation claim. Allocation failure/process abort are outside logical rollback guarantees. A future journal must commit the command before a durable final response; this version has no durable response.

## Sequencing and idempotency

Every new admitted request receives one global execution sequence, including a rejected business decision. Stable event IDs and posting IDs use separate typed namespaces with that sequence and their own ordinals. Matcher order.sequence remains the monotonic accepted-place priority ordinal; its ordering is consistent with global command order but it is not the global command sequence.

Deduplicate by (account_id, command_id). Typed ExecutionCommand equality is the current in-process canonical payload contract. Identical authorized retry returns the exact original Response, including rejection/events/IDs/postings, without committing additional effects. A retried response contains the same historical events/postings; a future dispatcher must deduplicate their identities rather than treat the response as a new execution. Changed payload returns DuplicateConflict. A failed insufficient-funds decision remains failed on the same key even after later funding; submit a new key to try a new decision. Network canonical serialization remains a separate M3 contract.

Authorization and valid envelope IDs are checked before cached results. Funding and market-state changes require administrative permission; place/cancel require trading permission. Permissions are a trusted in-process caller contract, not authentication or a cryptographic capability. An unauthorized retry cannot read an old privileged response. Invalid envelope, permission failure, duplicate conflict, receipt-capacity, and sequence exhaustion are admission errors and consume no sequence/cache slot. Business rejections consume one sequence/cache slot but no financial state.

## Market and failure behavior

Administrative pause/open commands are sequenced and idempotent. Pause rejects new orders, allows authorized cancels, and does not erase holds. Funding is allowed while paused. Exact retries return the original response after authorization even if market state changed. Duplicate order IDs are still distinct from duplicate command IDs.

Client timeout, restart, crash recovery, stable storage, and consumer delivery remain unimplemented. Returned events describe committed in-memory effects only. Retrying a returned response must not be treated by a consumer as a new event stream publication.
