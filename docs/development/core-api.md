# Core API guide

Status: Implemented in-process M2 baseline. Updated: 2026-10-15.

## Choose the right entry point

Use ExecutionCore for financial commands. Engine is a research/matching primitive with no balances; it is not the financial entry point. Both are exported for existing matching fixtures/reference tests, but ExecutionCore owns its matcher privately and exposes only immutable state views.

## Submit, inspect, and retry

```rust
use cinis_engine::{Asset, ExecutionCommand, ExecutionCore, ExecutionRequest, Permissions};

let mut core = ExecutionCore::default();
let request = ExecutionRequest {
    account_id: 1,
    command_id: 1,
    command: ExecutionCommand::Fund { asset: Asset::Quote, amount: 1_000 },
};
let response = core.execute(request.clone(), Permissions::ADMIN)?;
let effects = response.result.as_ref().expect("funding accepted");
assert_eq!(effects.postings.len(), 1);
assert_eq!(core.execute(request, Permissions::ADMIN)?, response);
core.check_invariants().expect("financial and matching state reconcile");
```

This excerpt belongs inside a function returning Result with AdmissionError; it is not a standalone source file. Permissions are supplied by a trusted in-process integration, not selected by an untrusted network client. A future gateway authenticates and authorizes before invoking the core. Synthetic funding is administrative, not a deposit/custody integration.

## Two failure classes

AdmissionError means no decision was admitted: invalid account/command ID, missing permission, changed payload on an existing key, full receipt history, or exhausted sequence. No financial/cache/sequence mutation occurs. Invalid configuration is also reported at construction.

ExecutionResponse.result can be Err(ExecutionReject): the request was admitted and receives a retained sequence/result, but financial/order/ledger/market state remains unchanged. A later retry gets that same business rejection. A new attempt after changed circumstances requires a new command ID.

Both successful and rejected admitted decisions count toward the bounded instance-lifetime cache. Exact authorized retries remain retrievable after capacity fills; new commands are blocked rather than old identities silently evicted.

## Responses and immutable views

Effects contains ordered ExecutionEvent values and the command's LedgerTransfer values, each with stable typed sequence/ordinal identity. A cached response repeats historical values. Never apply its postings again or infer current order status from an old acceptance; query the immutable current state instead. A future event consumer must deduplicate IDs.

state().account(id), accounts(), orders(), ledger(), issued(asset), is_market_open(), and resting_count() inspect current state. They provide no mutable references or persistence format. check_invariants validates matching, financial conservation, ledger reconstruction, holds, and receipt identity.

## Configuration and lifetime

ExecutionCore::with_limits(Limits) validates ordinal capacity and constructs an isolated instance. Numeric account/supply bounds are fixed in [ledger.md](../architecture/ledger.md); capacities are constructor choices. No runtime reconfiguration or history compaction exists. Default CLI builds a new in-memory instance each run; process exit loses balances/orders/results. A clone used in tests is not a durable snapshot.
