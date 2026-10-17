# Platform scaffold and development

Purpose: run and extend the actual .NET solution without confusing bootstrap behavior with financial integration.
Status: Implemented scaffold, verified locally on Windows 2026-10-18 (Asia/Saigon). Owner: repository maintainer. Hosted evidence is checked per integration SHA.

## Delivered baseline

Open [Cinis.Platform.slnx](../../src/platform/Cinis.Platform.slnx). It contains five production projects and five test projects grouped as Host, Modules and Tests. The project-reference graph follows accepted ADR-0007. ReadModels, MarketData, database migrations and a Rust service connection are not implemented.

| Project | Actual responsibility |
| --- | --- |
| Cinis.Platform | ASP.NET Core composition root, loopback health endpoints, structured console logging and finite smoke mode |
| Cinis.Access | Pure account/action grant-check use case and owned store port |
| Cinis.Access.Infrastructure | Immutable in-memory trusted grant store, empty in default composition; no database or API-key verifier |
| Cinis.Trading | Cancel intent validation, account authorization port, single forwarding call and transport-outcome propagation |
| Cinis.ExecutionClient | Explicit disabled gateway plus experimental bounded stream-frame codec; no TCP client or Rust protocol yet |

The first slice is in-process application behavior tested with ports. It is not FR-004 authentication or an end-to-end trading gateway. No client can submit financial commands through HTTP in this baseline. A principal string supplied in a unit test is not proof of identity.

The disabled gateway always returns UnavailableBeforeSend and never reports a financial success. Readiness is always 503 while execution is unconfigured. This is an explicit capability boundary, not a placeholder success or a fallback matching engine.

## Toolchain and package policy

The root global.json selects SDK 10.0.301 with latestPatch roll-forward inside that feature band and no preview SDKs. This is the installed/tested baseline, not a claim to use the latest SDK. CI installs the declared SDK. Record the resolved SDK/runtime when reporting measurements, and upgrade through normal reviewed checks.

Directory.Build.props owns net10.0/C# 14, nullable analysis, implicit usings, deterministic builds, warnings-as-errors, NuGet audit of all dependencies, and restore lockfile generation. Directory.Packages.props owns exact central package versions. NuGet.config clears inherited feeds and uses nuget.org. Commit every project packages.lock.json, and use locked restore in CI/verification. Refresh locks deliberately when package dependencies change; never disable warnings/audit to bypass a failure.

The test baseline uses stable xUnit 2.9.3, Visual Studio runner 4.0.1, Microsoft.NET.Test.Sdk 18.11.0 and ASP.NET Core integration hosting 10.0.13. No EF Core or Npgsql dependency is added before database work exists. The package family may evolve independently of application architecture; exact package compatibility is validated by restore/build/test on each supported CI host.

## Build, test and finite run

From repository root:

```sh
dotnet restore src/platform/Cinis.Platform.slnx --locked-mode
dotnet format src/platform/Cinis.Platform.slnx whitespace --no-restore --verify-no-changes
dotnet build src/platform/Cinis.Platform.slnx --no-restore -c Release
dotnet test src/platform/Cinis.Platform.slnx --no-build --no-restore -c Release
dotnet run --project src/platform/Cinis.Platform --no-build --no-restore -c Release -- --smoke-test
```

Smoke resolves the actual service graph, proves empty grants deny a cancel and the execution adapter sends nothing, prints the result and exits. It deliberately does not start an HTTP server. Both it and tests run without database, credentials, Rust listener or Aegis.

Logged Windows tasks:

```powershell
powershell -NoProfile -File ops/scripts/Invoke-Cinis.ps1 -Project Platform -Task Verify
powershell -NoProfile -File ops/scripts/Invoke-Cinis.ps1 -Project Platform -Task Run
powershell -NoProfile -File ops/scripts/Get-CinisLogs.ps1 -Tail 50
```

Platform Verify performs locked restore, format check, Release build, tests, smoke and repository docs/naming/scaffold checks. Engine remains the default project, with its existing tasks unchanged. RunMatching is engine-only; an unsupported project/task combination fails and records metadata rather than silently passing.

## Foreground host and logs

```powershell
powershell -NoProfile -File ops/scripts/Invoke-Cinis.ps1 -Project Platform -Task Serve -TimeoutSeconds 300
```

Serve binds http://127.0.0.1:5080 through fixed non-secret task arguments. JSON console records, stderr and run/step metadata are captured under ignored artifacts/logs/<run-id>/. Use a second terminal to read the reported files or Get-CinisLogs. This is bounded foreground debugging, not background supervision, restart policy or production service management. At the deadline the runner stops only its owned process tree and returns 124; a timeout is not reported as a successful service completion. Existing process-runner tests cover owned-tree cleanup. Shutdown cooperation/rotation for production hosting remains a later service-lifecycle gate.

For interactive development with Ctrl+C:

```sh
dotnet run --project src/platform/Cinis.Platform --no-launch-profile -c Release
```

Direct launch defaults to the same loopback address unless explicitly overridden through ASP.NET Core configuration. ASPNETCORE_URLS and ASPNETCORE_ENVIRONMENT are optional host variables; .env files are not loaded automatically. Do not expose this scaffold as a public trading service. The runner owns only the processes it starts; it must not kill an existing process to free a port.

| Endpoint | Expected response |
| --- | --- |
| GET /health/live | 200, alive in scaffold mode |
| GET /health/ready | 503, execution_not_configured |
| POST /orders, /funding, /api-keys | 404; no business routes registered |

Liveness never implies trading readiness. Host logs and health do not provide a journal or durability evidence.

Verified with a real loopback HTTP process on 2026-10-18: live returned 200, ready returned 503/execution_not_configured, the logged runner returned the expected 124 at its 20-second deadline, and its owned listener was gone afterwards. Ignored artifacts/platform/serve-evidence.json and the run logs hold this local diagnostic evidence. This is bounded debug lifecycle evidence, not production supervision or recovery certification.

## Current test evidence

37 .NET cases: Access 8, Trading 7, ExecutionClient 11, host integration 5, architecture 6. Access checks exact principal/account/action matching, empty/default deny, malformed grants, immutable input copy and cancellation. Trading checks invalid/unauthorized intents never contact execution, preserves full u64 identities, and propagates known/unknown/unavailable outcomes without generating a new intent or retrying.

Codec checks big-endian lengths, fragmented reads, exact u64 payload bytes, zero/oversized lengths rejected before payload allocation, truncated frames, rejected writes without a header and consecutive frame boundaries. It is an experiment with four-byte length headers only; version/message schema, connection/correlation, trust and actual Rust interop are still TD-01/02/03/04 work. Do not treat the generic byte codec as a frozen financial wire protocol.

Architecture tests inspect the actual .slnx and project-reference allowlist, prove forbidden-reference fixtures fail, and verify core assemblies/projects lack framework/adapter dependencies. They do not yet implement every proposed future Domain/Features namespace rule; add those rules with the corresponding code. Integration tests use the actual host/DI graph and verify readiness denial and absent financial endpoints. No database, authentication/revocation, crash recovery or TCP round-trip test is claimed.

## Continue in order

1. Complete the cross-language spike with bounded frames, versioning, exact numeric encoding, correlation, disconnect and same-ID retry evidence; accept its protocol/trust ADR.
2. Close account/key/revocation decisions, implement real credential validation and persistence adapters, then expose the first authorized financial endpoint through Rust.
3. Add event-feed/source identity, projections/cursors and real PostgreSQL integration tests only with an accepted feed contract.
4. Implement durable journal/replay and service lifecycle evidence before durable results and full restart acceptance.

Use the [architecture](../architecture/platform-architecture.md), [implementation plan](../architecture/platform-implementation-plan.md), [backlog](../delivery/backlog.md) and [protocol draft](../architecture/protocol.md) as the canonical references. Schema/wire representations remain separate from these application contracts.
