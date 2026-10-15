# ADR-0003: One documentation tree and explicit local automation

Date: 2026-10-15. Status: Partially superseded by ADR-0004 for path naming and script placement; process/logging decisions retained.

## Context and alternatives

Cinis is a long-lived Rust/.NET lab and needs reproducible commands and debug evidence. Alternatives were scattered per-project scripts, a renamed/duplicated document tree, or a large service orchestrator before services exist.

## Decision

Keep docs/ as the canonical documentation root; use ops/ for trusted task configuration, process scripts, and their integration tests. Preserve scripts/ for portable repository checks, deploy/ for real future deployment manifests, and artifacts/logs/ for ignored per-run diagnostics.

Implement a Windows PowerShell 5.1 runner for actual Engine jobs now, with separate output streams, unique run IDs, metadata, preserved failures, bounded timeouts, and owned-process cleanup. Do not add a background service manager, fake platform project, or empty production hierarchy. Use project-level runbooks and extension requirements when persistent services arrive.

## Consequences and revisit

Current Windows developers share build/test/run commands; Linux engine CI keeps direct Cargo checks. Logs are diagnostic, never authoritative financial records. Runner tests and Windows CI are needed because shell argument/exit behavior differs from direct application calls. Revisit cross-platform automation or production supervision when a real requirement exists.

See [ADR-0004](0004-naming-and-automation-layout.md): current documentation uses docs/operations/ and portable checks now live under ops/scripts/. The scripts/ decision above is historical.
