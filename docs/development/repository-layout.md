# Repository layout

Status: Accepted convention, 2026-10-15. Owner: repository maintainer.

## Top-level responsibilities

| Path | Owns | Does not own |
| --- | --- | --- |
| src/engine/ | Rust execution crate, its unit/integration runners, Cargo.lock | Operational process management |
| src/platform/ | Future .NET gateway/management projects | Independent financial authority |
| tests/fixtures/ | Shared versioned acceptance input/expected output | Generated test output |
| tools/ | Product-adjacent CLI/load/replay tools as they appear | Repository maintenance wrappers |
| ops/config/ | Trusted local task registry | Secrets or trading business configuration |
| ops/scripts/ | Build/test/run/log automation and portable quality checks | Financial matching or durable execution |
| ops/tests/ | Native process/automation integration tests | Matching business tests |
| docs/ | Maintained English specifications, guides, decisions, runbooks | Executables or a second backlog |
| deploy/ | Future actual deployment manifests/IaC | Hypothetical environments |
| research/ | Timeboxed experiments and reproducible findings | Unverified performance claims |
| .github/ | CI and review/issue templates | Local log files |
| artifacts/ | Ignored build/diagnostic evidence | Source, secrets, or authoritative execution data |

Use docs/ rather than document/ or documents/; use ops/ rather than operation/ or operations/. Documentation topics use full names, including docs/operations/. Root scripts/ is consolidated into ops/scripts/. This keeps one documentation tree and one executable automation tree. Do not create empty production/cloud/service folders before they have a concrete responsibility.

## Documentation taxonomy

Product requirements describe desired behavior; domain specifications define exact semantics; architecture defines ownership/contracts and ADRs; verification defines evidence; delivery holds milestone outcomes and the single backlog; development describes contributor actions; ops holds runnable procedures and operational contracts. Templates live under docs/templates/ and are copied only when needed.

## Rust baseline

The single src/engine crate uses private cohesive matching/, ledger/ and execution/ modules. Module roots hold owned fields and explicit exports; mutation, settlement and audit implementations are separate children. execution composes matching and ledger; ledger errors are mapped at the orchestration boundary without importing execution. Binaries remain at src/main.rs and src/bin/matching_demo.rs. Integration tests exercise the public facade, including outcome classification. See the [Rust engine guide](rust-engine.md) for the complete tree and extension recipes, and [ADR-0006](../architecture/decisions/0006-rust-module-boundaries.md) for rationale.

The matcher currently uses Vec-backed scan/sort storage. Avoid calling it optimized or zero-allocation. Financial state is implemented according to ledger.md and ADR-0005; whole-state copying/full reconciliation are a correctness baseline, not a low-latency design. A multi-crate workspace is justified only by real dependency/reuse boundaries.

## Evolution

When .NET appears, add solution/project files and global.json together; introduce runtime configuration/deployment docs with an actual service. Keep source and CI commands linked. Changes to ownership or major directory responsibilities require an ADR and link updates; generated directories remain ignored. Portable checks live under ops/scripts/; any move updates CI, registry arguments, script-relative roots, and documentation together. See [naming conventions](naming-conventions.md) for language-specific path and identifier rules.
