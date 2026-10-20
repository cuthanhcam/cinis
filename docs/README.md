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

- [.NET platform architecture and patterns](architecture/platform-architecture.md), [implementation sequence](architecture/platform-implementation-plan.md), and [Accepted ADR-0007](architecture/decisions/0007-platform-architecture.md)

- [Solution architecture blueprint](architecture/blueprint/README.md), [interactive Atlas](architecture/blueprint/architecture-atlas.html), and [diagram catalog](architecture/blueprint/diagrams.md)
- [Technology and algorithms](architecture/blueprint/technology-and-algorithms.md), [business/technical decision register](architecture/blueprint/decision-register.md), and [quality gates](architecture/blueprint/quality-and-gates.md)

- [Overview](architecture/overview.md) and [state ownership](architecture/state-ownership.md)
- [Ledger and financial execution](architecture/ledger.md)
- [Durability](architecture/durability.md)
- [Protocol draft](architecture/protocol.md)
- [Security model](architecture/security.md)
- [ADR-0001](architecture/decisions/0001-project-boundaries.md), [ADR-0002](architecture/decisions/0002-matching-baseline.md), and [ADR-0003](architecture/decisions/0003-repository-and-ops.md), and [ADR-0004](architecture/decisions/0004-naming-and-automation-layout.md), and [ADR-0005](architecture/decisions/0005-financial-execution.md)

## Delivery and evidence

- [Milestones](delivery/milestones.md), [backlog](delivery/backlog.md), [M0 review](delivery/m0-review.md), and [M1 review](delivery/m1-review.md)
- [Risks](delivery/risks.md)
- [Acceptance scenarios](verification/scenarios.md) and [financial evidence](verification/financial-testing.md)
- [Invariants](verification/invariants.md), [strategy](verification/strategy.md), and [differential testing](verification/differential-testing.md)
- [Benchmarks](verification/benchmarks.md)
- [Rust engine development architecture](development/rust-engine.md) and [ADR-0006](architecture/decisions/0006-rust-module-boundaries.md)
- [Platform scaffold and run guide](development/platform.md), [solution](../src/platform/Cinis.Platform.slnx), and [bootstrap ADR-0008](architecture/decisions/0008-platform-bootstrap.md)
- [Platform transport and source documentation](development/platform-transport.md) and [ADR-0009](architecture/decisions/0009-platform-transport-channel.md)
- [HTTP middleware, filters and logging](development/platform-http-pipeline.md), [ADR-0010](architecture/decisions/0010-platform-http-diagnostics.md)
- [Core API](development/core-api.md)
- [Setup](development/setup.md), [testing](development/testing.md), [CI/integration](development/ci.md), and [scaffold readiness](development/scaffold.md)
- [Ops and runbooks](operations/README.md)
- [Repository layout](development/repository-layout.md) and [documentation standard](development/documentation-guide.md), and [naming conventions](development/naming-conventions.md)
- [Document templates](templates/README.md)
- [Spikes](../research/README.md) and [experiment template](../research/experiment-template.md)

## Review queue

The [architecture decision register](architecture/blueprint/decision-register.md) prioritizes account/access, revocation, exact wire representation and complete event batches before M3; durability/capacity/retention before M4. New recommendations remain Proposed until explicitly accepted.

Matching policies and numeric/capacity limits are accepted in ADR-0002. Financial limits/ledger are implemented in ADR-0005; cross-language framing remains open before M3. Before M4: decide journal format, synchronization, snapshot publication, integrity, retention. Historical discussion has multiple milestone drafts; the delivery plan here is the working baseline.
