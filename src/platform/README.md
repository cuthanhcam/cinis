# .NET platform

Status: architecture planning only, 2026-10-16. No solution/project, SDK pin, listener or database stack is implemented yet.

The proposed platform is a modular monolith with ports/adapters, vertical use-case slices and explicit command/query separation. Rust remains the only financial authority; PostgreSQL projections cannot approve spending.

Start with the [platform architecture plan](../../docs/architecture/platform-architecture.md), [implementation sequence](../../docs/architecture/platform-implementation-plan.md), and [Proposed ADR-0007](../../docs/architecture/decisions/0007-platform-architecture.md). They define module responsibilities, the proposed solution graph, failure/transaction boundaries, architecture tests, worker ownership and scale/extraction gates.

Create projects only as actual features are implemented after architecture/protocol review. Keep future .NET tests with the solution under tests/ here; shared cross-language fixtures remain under repository tests/fixtures/. Follow [repository layout](../../docs/development/repository-layout.md) and [naming conventions](../../docs/development/naming-conventions.md). Operational automation belongs under repository ops/.
