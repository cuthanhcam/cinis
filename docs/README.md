# Documentation index

Maintained specifications use Cinis. Historical conversation input is local-only. Behavior is planned unless explicitly accompanied by implementation evidence.

## Product and domain

- [Charter](product/project-charter.md)
- [Scope](product/scope-v0.1.md) and [requirements](product/requirements.md)
- [Glossary](domain/glossary.md)
- [Trading rules](domain/trading-rules.md)
- [Financial model](domain/financial-model.md)
- [Order lifecycle](domain/order-lifecycle.md)

## Architecture

- [Overview](architecture/overview.md) and [state ownership](architecture/state-ownership.md)
- [Durability](architecture/durability.md)
- [Protocol draft](architecture/protocol.md)
- [Security model](architecture/security.md)
- [ADR-0001](architecture/decisions/0001-project-boundaries.md) and [ADR-0002](architecture/decisions/0002-matching-baseline.md)

## Delivery and evidence

- [Milestones](delivery/milestones.md), [backlog](delivery/backlog.md), and [M0 review](delivery/m0-review.md)
- [Risks](delivery/risks.md)
- [Acceptance scenarios](verification/scenarios.md)
- [Invariants](verification/invariants.md) and [strategy](verification/strategy.md)
- [Benchmarks](verification/benchmarks.md)
- [Setup](development/setup.md) and [testing](development/testing.md)
- [Runbook plan](operations/runbooks.md)
- [Spikes](../research/README.md) and [experiment template](../research/experiment-template.md)

## Review queue

Matching policies and numeric/capacity limits are accepted in ADR-0002. Financial limits and cross-language framing remain open before M2/M3. Before M4: decide journal format, synchronization, snapshot publication, integrity, retention. Historical discussion has multiple milestone drafts; the delivery plan here is the working baseline.
