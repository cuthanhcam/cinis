# ADR-0001: Single execution owner and synthetic venue

Date: 2026-10-11. Status: accepted bootstrap direction; implementation pending.

## Context

Cinis combines Rust systems and .NET backend practice. Separate financial writers introduce distributed reservation/recovery problems before a correct baseline.

## Options

1. .NET reserves funds, Rust only matches.
2. Rust owns financial/matching transitions; .NET owns access/management.
3. Multiple execution services with distributed accounting.

## Decision

Choose option 2, one market, synthetic assets, one execution writer. PostgreSQL holds management data and execution projections. Begin with safe Rust and ordinary collections, and framed loopback TCP before advanced IPC.

## Consequences and review

Risk and matching share a deterministic state boundary. Gateway overhead remains measurable. Recovery requires an explicit journal/snapshot contract. Revisit after v0.1 evidence justifies partitioning, an external ledger, or transport changes; record migration consequences first.
