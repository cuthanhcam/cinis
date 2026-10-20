# .NET platform

Status: implemented fail-closed scaffold, 2026-10-17. Open [Cinis.Platform.slnx](Cinis.Platform.slnx).

The solution contains five production projects and five test projects. Platform is the composition root; Access owns local grant checks; Access.Infrastructure provides an immutable in-memory store; Trading contains a cancel workflow and owned ports; ExecutionClient provides a disabled gateway and experimental bounded frame codec and single-flight loopback TCP channel. Rust remains the financial authority.

The default host has no grants and no financial/management HTTP routes. Liveness is available; trading readiness returns 503 because Rust transport is not configured. No authentication, database, Rust connection, durable feed or recovery is claimed.

See the [development/run guide](../../docs/development/platform.md), [architecture](../../docs/architecture/platform-architecture.md), [implementation sequence](../../docs/architecture/platform-implementation-plan.md), [ADR-0007](../../docs/architecture/decisions/0007-platform-architecture.md) and [bootstrap ADR-0008](../../docs/architecture/decisions/0008-platform-bootstrap.md).

```sh
dotnet restore src/platform/Cinis.Platform.slnx --locked-mode
dotnet build src/platform/Cinis.Platform.slnx --no-restore -c Release
dotnet test src/platform/Cinis.Platform.slnx --no-build --no-restore -c Release
dotnet run --project src/platform/Cinis.Platform --no-build --no-restore -c Release -- --smoke-test
```

Commands start at repository root. Windows logged tasks use Invoke-Cinis.ps1 -Project Platform with Verify, Run (finite smoke), or Serve (bounded foreground health host). Tests live under tests/ here; cross-language fixtures stay under repository tests/fixtures/. ReadModels/MarketData arrive with actual capabilities. Commit all package locks; Platform CI runs on Linux/Windows.

See [transport and XML documentation standards](../../docs/development/platform-transport.md) for resource bounds, outcome semantics, test evidence and maintenance guidance. The channel is not registered as a financial gateway.
