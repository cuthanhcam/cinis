# ADR-0006: Rust module boundaries and classified command outcomes

Date: 2026-10-16. Status: Accepted implementation convention. Owner: repository maintainer. Business policies from ADR-0002/0005 remain unchanged.

## Context and alternatives

Matching and ledger files mixed types, mutation and full-state audits. Ledger used execution's rejection enum, creating a reverse dependency on orchestration. Execution responses intentionally repeat historical events on retries, but a future adapter had no invocation classification. Multiple crates would add manifests/dependency maintenance without a second runtime/dependency boundary. Mutable book/ledger exposure would weaken financial ownership.

## Decision

Keep one dependency-free crate with private cohesive matching, ledger and execution submodules. Roots hold private state and explicit reexports; children implement mutation and audits. Execution composes matching and ledger, never the reverse. Keep ledger storage private and translate private LedgerError into unchanged public rejections. Preserve existing public exports and execute signature.

Add execute_with_outcome returning ExecutionOutcome with DecisionSource::New or Retained outside the original ExecutionResponse. New includes success and retained business rejection. Retained means authorized same-payload receipt retrieval with no new transition/sequence. execute returns only the original response for compatibility. Compile the public example as a doctest and deny broken Rustdoc links.

## Consequences

Adapters can distinguish retries without guessing from a sequence or republishing historical effects. No publisher, persistence, authentication or exactly-once delivery is added; a crash between decision and publication still requires M4's durable feed. Matching/financial rules, capacities and volatility remain unchanged. Ledger operations still require whole tentative-transaction rollback on failure; an operation is not independently atomic.

More files are organized by responsibility, not per type. Rust privacy controls mutable access; review still checks semantic dependency direction. Future transport/persistence crates need concrete boundaries and accepted protocols.

## Verification and revisit

Existing matching fixtures/oracle and financial/generated tests must pass unchanged. outcomes.rs tests success/rejection classification, authorization/conflict no-mutation, capacity retries and API compatibility. The Rustdoc example/generation and both demos must pass, as must all hosted workflows on the exact integration SHA.

Revisit crate boundaries for async/network/storage dependencies, actual reuse or independent tools. Revisit transaction mechanics only with equivalent financial/differential/failure evidence and measured need. See the [Rust engine guide](../../development/rust-engine.md).
