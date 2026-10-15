# ADR-0005: Atomic financial execution and retained command results

Date: 2026-10-15. Status: Accepted implementation baseline. Extends ADR-0002 without changing matching priority or numeric units.

## Context and alternatives

A correct book alone cannot prevent overspending. Options were gateway-managed holds, separate financial storage transactions, or one Rust execution owner with an atomic in-memory transition. Independent writers would require distributed recovery before a financial baseline exists.

## Decision

Use ExecutionCore as the financial entry point, retaining Engine only as a matching primitive/research API. Isolate tentative matcher/ledger state and commit only after checked postings and reconciliation. Use explicit account/supply/history limits from ledger.md, zero fees, administrative synthetic funding, and sequenced market state. Cache both successful and rejected business results by account/command identity; compare typed payloads and recheck current permissions before returning a cached response.

## Consequences

No overspend or partial settlement is observable after a business rejection. Rejected decisions consume result history/sequence but not order priority or financial state. Retries retain original identifiers. Whole-state copying/full checks cost memory and time and are deliberately unsuitable for latency claims. Bounded history has no compaction; capacity exhaustion can prevent a new cancel and must be treated as a lab halt/operational limit, never silent eviction.

## Verification and revisit

Verify funding, holds, price improvement on both taker sides, partial/multiple fills, cancel, account/supply/ledger/result capacity, permission and conflict behavior, cached rejects, deterministic generated streams, conservation, independent ledger aggregation, and reservation equality. Persisted deduplication, replay, and power-loss behavior require M4 design/evidence; no restart guarantee is added here. Revisit copying only with equivalent differential/financial evidence and measured need.
