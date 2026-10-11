# Project charter

Status: initial baseline, 2026-10-11. Owner: repository maintainer.

## Problem and audience

Developers studying trading infrastructure need an integrated, explainable environment for following commands through API access, matching, financial state, and recovery. Primary audience: backend/systems developers. Secondary audience: strategy researchers. This value proposition is a hypothesis to validate with a reproducible demo and reviewer feedback.

## Outcomes

Build a deterministic simulated venue; demonstrate financial correctness and recovery; develop Rust systems and .NET backend skills; publish reproducible performance evidence. The project name is Cinis, replacing Cineres.

Success: a clean-start v0.1 demo completes the acceptance journey, invariants hold under generated workloads and recovery tests, and another developer reproduces documented results.

## Constraints and boundaries

One maintainer learning Rust, one market, synthetic funds, single execution writer. Weekly capacity is unconfirmed; forecasts assume 8-10 focused hours per week only for planning. Revisit scope and estimates after spikes, M1, and M2.

Real custody, production exchange operation, blockchain, trading strategies, replication, shared-memory IPC, and zero-allocation claims are outside v0.1. Aegis integration is a later option. learn-dotnet and learn-rust remain knowledge repositories; Cinis applies concepts to one coherent system.
