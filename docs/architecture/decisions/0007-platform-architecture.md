# ADR-0007: Modular .NET platform with explicit adapter boundaries

Date: 2026-10-16. Status: Proposed; not implemented. Owner/approver: project maintainer.

## Context

Before creating the long-lived .NET solution, Cinis needs a dependency model and scaling plan that preserve Rust's sole financial authority. The existing blueprint proposes a modular host but does not yet specify project references, use-case organization or service extraction criteria. Current Rust execution and ops are implemented; .NET applications, authentication, storage and listeners are not.

## Options

1. Microservices immediately: independent deployment, with network/data coordination and operational cost before contract/load evidence.
2. One global layered application: small initial project count, but broad shared models/layers can erode ownership as unrelated capabilities grow.
3. Four projects for every module: strong potential separation, with repeated wiring and empty layers before complexity warrants them.
4. Modular monolith with selective adapter projects and feature slices: explicit boundaries while starting with one .NET deployment.

## Proposed decision

Recommend option 4. Apply ports-and-adapters dependency inversion, vertical slices within bounded contexts, and CQRS for Rust command authority versus derived query models. Use five production projects for the first authenticated command path only when the feature is implemented: Platform host, Access, Access.Infrastructure, Trading and ExecutionClient. Add ReadModels/infrastructure and MarketData with real capabilities.

Consumer modules own required ports. The host composes adapters; infrastructure depends on application contracts. Domain/application boundaries avoid ASP.NET/EF/Npgsql/wire dependencies. Internal module storage is not a cross-module API. Management uses module-owned relational transactions; Rust remains the only matching/reservation/settlement/receipt authority. No .NET/PostgreSQL transaction spans execution atomically.

Start with explicit handlers/DI, no mandatory mediator, generic repository, broker, distributed saga or management event-sourcing framework. Enforce allowed project/namespace dependencies and public contracts with architecture tests. A full Clean Architecture template is not required for every module; inward dependency and meaningful ownership are required.

## Consequences and scaling

Logical modules can evolve without a service for every capability. Multiple API replicas do not imply multiple Rust writers. Projection workers need explicit role/ordered cursor ownership before replicas; durable resumption waits for M4. Service extraction requires observed workload/failure/security/deployment needs plus stable contracts, owned data and operational/migration evidence.

More projects than a single Web API increase wiring, but isolate real application/adapter boundaries. The model still requires discipline: folders alone do not enforce ownership. Endpoint-to-handler mapping and adapter translation add code intentionally; avoid duplicating financial policies or inventing abstract layers without consumers.

## Approval and implementation gates

This ADR remains Proposed pending review of the [architecture plan](../platform-architecture.md). No business/access/protocol choice is implicitly accepted. Resolve BD-01/02/03 and relevant TD-01/02/03/04/05 decisions with spike evidence before the production gateway contract is implemented. Follow the [implementation sequence](../platform-implementation-plan.md); do not create hypothetical scaffolding now.

Verify architecture-test negative cases, policy/authorization, protocol failure/unknown outcomes, real database transactions/cursors, role isolation, logs/readiness/shutdown and all hosted checks. Revisit topology when measurement or business needs justify isolation, not simply because more modules exist.
