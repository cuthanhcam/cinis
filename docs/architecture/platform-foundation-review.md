# Platform enterprise-foundation review

Status: Review of the maintainer-provided checklist, 2026-10-20. Existing ADRs remain authoritative; recommendations below do not silently accept new technology or business contracts.

Purpose: map the checklist to delivered evidence and remaining gates. The [delivery backlog](../delivery/backlog.md) remains the single work tracker; this document is a coverage review, not a second backlog.

| Checklist area | Current evidence | Remaining decision or acceptance gate |
| --- | --- | --- |
| Structured logs and identity | Custom JSON console formatter; service/environment/version/process/run identity; request/trace/span scopes; completion duration/status; safe exception event | Serilog is an alternative, not installed. Add it only for a concrete sink/operational need; preserve privacy tests and avoid duplicate file writers |
| Files, retention and privacy | Runner captures stdout/stderr; Latest overwrites between runs, exclusive writer lock; Archive opt-in; ignored artifacts; CI seven-day retention; scope allowlist | No in-run file rotation, local byte quota or archive expiry. Define storage budgets before sustained service/load operation; Latest is not rolling retention |
| Distributed telemetry | ASP.NET Core Activity/W3C correlation and response IDs | No OpenTelemetry exporter, collector, dashboards or metric acceptance evidence. Choose environment, sampling, cardinality limits and privacy policy before adding packages |
| Middleware and filters | RequestDiagnosticsMiddleware combines correlation/completion; IExceptionHandler translates unexpected failures; NoStoreEndpointFilter protects health | SecurityHeadersMiddleware now applies an API-only browser response policy with success/error coverage. Endpoint validation waits for concrete input contracts; MVC filters wait for an actual MVC host |
| Business results and HTTP errors | Typed CancelOrderResult and ExecutionSubmission preserve rejected/unavailable/unknown distinctions; correlated Problem Details for unexpected/empty HTTP errors | Shared generic/non-generic results, stable error taxonomy and financial HTTP mapping need a concrete endpoint contract; no financial HTTP route exists |
| Security | Localhost defaults, local AllowedHosts, no exposed trading routes, exact trusted in-process grants, no stack trace in HTTP problems | Authentication, API-key lifecycle, account isolation/revocation and policy authorization remain NET-05/ARCH-02 gates. Define deployment trust before CORS, HTTPS/proxy policy and rate-limit budgets; never accept arbitrary forwarded headers or install permissive policies |
| Health and resilience | Live 200, ready 503, no-store; bounded experimental request channel; cancellation/deadlines and conservative outcome tests | Dependency checks require real dependencies. No blind financial retry: uncertain submissions retain identity and require the Rust idempotency contract. Circuit breaking needs explicit failure/recovery semantics |
| Configuration and bootstrap | Explicit AddPlatformModules, AddPlatformHttp and UsePlatformHttp; immutable validated transport options; standard host/logging configuration | Do not introduce a second bootstrap facade solely for naming. Add validated options when configurable dependencies exist, including startup rejection tests; avoid fabricated connection strings |
| Developer operations and CI | Logged Verify/Run/Serve, Latest/Archive, timeout/owned-tree cleanup; localhost process checks; 61 .NET and 21 ops tests; five exact-SHA CI gates | Custom config-file override, cooperative Ctrl+C/service shutdown, production supervision and Docker runtime health remain unimplemented. No container or database migration workflow is claimed |
| Documentation and maintenance | XML API comments, architecture/module boundaries, pipeline/logging guides and ADRs | Apply [C# declaration layout](../development/csharp-conventions.md) in future changes; update evidence as capabilities land |

## Result-contract design gate

The checklist correctly separates application results from HTTP responses. Retain that boundary: HTTP status, trace ID and request ID belong in the host mapping, not in core business outcomes. Expected validation/business failures should remain typed results; unexpected exceptions go through the existing exception handler.

The illustrated public positional CinisOperationResult<T> can represent contradictory states, such as IsSuccess=true with errors. Before adopting it, define construction invariants, success-without-data semantics, an immutable/copied error collection, stable error codes and generic/non-generic consistency. Prefer controlled construction if the type is introduced. Clients must depend on codes, not human messages. Avoid adding a SharedKernel merely to store an unused record.

Financial transport adds a further requirement: a lost reply is neither known success nor known rejection. Preserve UnknownOutcome, NotSent and validated business-result distinctions at the adapter boundary. Do not replace ExecutionSubmission with a Boolean result or map uncertainty to a new command ID, definitive rejection or automatic retry. A completed frame alone is not a financial acknowledgement. See [transport semantics](../development/platform-transport.md).

The first authenticated financial endpoint must specify success statuses, validation/authorization/conflict mappings, uncertain-result lookup/reconciliation, exact numeric representation and schema/correlation before implementation. Use ProblemDetails/ValidationProblemDetails for HTTP errors and ordinary successful representations rather than competing error envelopes.

## Sequence and completion evidence

1. Finish NET-05 protocol/trust/account decisions and the cross-language evidence already required by the [implementation plan](platform-implementation-plan.md).
2. For the first concrete endpoint, define application outcomes/error codes and HTTP mappings; introduce a common result abstraction only if actual consumers benefit.
3. Implement credential validation and endpoint protections with isolation/abuse tests before exposing trading commands.
4. Add telemetry export, bounded operational retention and service lifecycle/container evidence against an explicit runtime environment.
5. Add database health/migrations and projection recovery only with the corresponding implemented persistence/feed contract.

This review does not relabel missing security or production operations as complete. See [ADR-0007](decisions/0007-platform-architecture.md), [ADR-0010](decisions/0010-platform-http-diagnostics.md), [pipeline](../development/platform-http-pipeline.md) and [operations logs](../operations/logging.md) for accepted current behavior.

## Pipeline-strategy note: considered implementation

The maintainer's additional middleware note is applied through the existing host composition plus [ADR-0011](decisions/0011-api-response-security-headers.md). Each component retains a bounded responsibility: response headers, request diagnostics, unexpected-error translation, status mapping or endpoint policy. Request identity and completion remain together as one diagnostic scope; no independent AsyncLocal or external logging context is required.

The example ordering is a starting point, not a universal ASP.NET Core sequence. Cinis resolves routing before request diagnostics to record route templates; diagnostics wraps error translation to see final status. Server/host rejection and route-selection failures can happen before that diagnostic boundary. Security headers register before routing. Authentication/authorization will be placed after routing and before protected endpoints when those capabilities exist; no empty authentication registrations are shipped.

Incoming traceparent can supply untrusted W3C correlation while a request ID is generated locally. Do not echo arbitrary X-Correlation-ID, log raw paths/query/body, enable body buffering just for diagnostics, or log the same exception from multiple middleware components. The built-in IExceptionHandler remains the unexpected-error boundary; future custom business exceptions require deliberate mappings rather than guessing from exception names. Source-generated logging can be evaluated with evidence if allocation cost becomes material; no zero-allocation claim follows from this middleware design.
