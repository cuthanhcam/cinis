# Cinis architecture blueprint

Purpose: give maintainers one reviewable map of the product, authority boundaries, technology choices, and evolution gates before further source development.
Status: architecture review baseline; existing ADRs remain authoritative. New recommendations are Proposed, not silently accepted.
Owner: project maintainer. Reviewed: 2026-10-15. Implementation baseline: develop at 6cf0828.

## Start here

1. Open [Architecture Atlas](architecture-atlas.html) directly in a browser. It works offline without a server, CDN, package install, or telemetry. Select a view, then select a component to inspect its responsibilities and failure boundary. GitHub displays HTML source; use the local file to interact.
2. Read [solution architecture](solution-architecture.md) for scope, bounded contexts, contracts, trust, deployment, and extension seams.
3. Review [diagram catalog](diagrams.md): context, containers, execution internals, order flow, ledger flow, recovery, deployment, evolution, and logical domain relationships. Mermaid source renders on GitHub; the Atlas is a complementary navigation view.
4. Review [technology and algorithms](technology-and-algorithms.md) for actual choices, alternatives, complexity, and evidence needed to change them.
5. Close the [decision register](decision-register.md) in priority order. It identifies business questions as well as technical choices; no new policy is accepted by publishing this package.
6. Use [quality and delivery gates](quality-and-gates.md) to judge whether a design is ready to implement or expand.

## Reading the status

| Label | Meaning |
| --- | --- |
| Implemented | Present in the current Rust/ops baseline; does not imply network security or persistence |
| Accepted | Existing specification/ADR decision; delivery may still be pending |
| Proposed | Recommended direction requiring review and evidence before acceptance |
| Deferred | Outside the v0.1 baseline; an extension seam, not a delivery promise |

Diagram edges describe logical information flow, not a claim of a deployed integration. Every target view is labeled. The execution path changes at M4: current responses mean in-memory completion; only the future tested journal boundary can justify a durable result.

## Sources of truth and maintenance

[Requirements](../../product/requirements.md) own FR/QR acceptance. [Trading rules](../../domain/trading-rules.md) and [financial model](../../domain/financial-model.md) own business semantics. [State ownership](../state-ownership.md), [ledger](../ledger.md), [protocol](../protocol.md), [durability](../durability.md), and [ADRs](../decisions/0005-financial-execution.md) own detailed contracts. This package explains their composition; it does not replace them.

[Backlog](../../delivery/backlog.md) remains the only work-item tracker. The decision register tracks choices and approval gates, not duplicate implementation tickets. On any boundary change, update the detailed contract first, then the affected Mermaid and HTML views in the same change. Keep implemented/target/deferred labels consistent. Validate local links/naming, JavaScript syntax, desktop/mobile rendering, keyboard navigation, and all required hosted workflows before advancing develop.

## Documentation verification

Verified locally on Windows on 2026-10-15: all ten Mermaid blocks rendered successfully with Mermaid CLI; all eight Atlas views and 47 selectable components were exercised in headless Chrome. Desktop and 390-pixel mobile screenshots were inspected. Keyboard Enter activation, hash-based view reload, local HTML links, print generation, JavaScript syntax, and absence of page errors/document overflow were checked. The narrow screen intentionally scrolls the diagram horizontally. Screenshots and QA dependencies remain ignored under artifacts/architecture and artifacts/tools; they are not runtime dependencies of the Atlas.

Repository Verify passed format, Clippy, build, all 56 Rust tests, documentation/naming, workflow lint and scaffold checks. This is documentation and baseline-regression evidence, not a test of proposed gateway/storage behavior. Hosted exact-SHA workflow results are reported by the integration gate separately.
