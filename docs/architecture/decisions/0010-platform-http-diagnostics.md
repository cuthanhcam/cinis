# ADR-0010: Host-owned HTTP diagnostics and repeatable local service logs

Date: 2026-10-20. Status: Accepted for the current scaffold.

## Context

Repeated service debugging needs a stable log path, while CI/jobs need independent run histories. HTTP failures need correlation without logging financial identities or credential-bearing request data. The current host uses Minimal APIs and exposes health only.

## Decision

- Serve defaults to a fixed latest project/task log directory, replaced only after acquiring an exclusive writer lock. Archive mode remains available; finite jobs retain archived runs by default.
- A run ID is passed to the child as non-secret diagnostic metadata and the parent environment is restored immediately. Logs and locks stay under ignored artifacts/logs.
- HTTP middleware owns request correlation/completion; IExceptionHandler owns generic unexpected-error translation; a Minimal API filter applies health cache policy. All remain in the host.
- A JSON console formatter emits process identity and allowlisted correlation scopes. Application completion records use route templates. Framework request logging stays at Warning by default.
- Host configuration and launch scripts use http://localhost:5080, retaining loopback-only development binding and explicit allowed local hosts.

## Consequences and limits

Latest mode deliberately discards previous logs on the next non-concurrent run; use Archive when historical evidence matters. Locks prevent competing cooperative runners, not unrelated tools editing files. No financial durability, production supervisor, automatic restart or trace exporter is introduced. See the [pipeline contract](../../development/platform-http-pipeline.md) and [operator guide](../../operations/logging.md) for concrete behavior and tests.
