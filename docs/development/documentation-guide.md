# Documentation standard

Status: Accepted project convention, 2026-10-15. This is the Cinis documentation standard, not a claim of external certification.

## Authoring contract

Write maintained material in English. Start with purpose, status, owner where operational responsibility matters, and last review/verification date. Distinguish Proposed, Accepted, Implemented, Verified, and Deferred. Accepted is a design baseline; Verified must identify actual commands, revision/environment, and limits.

Use relative file links, stable FR/QR/BR/INV/SC/MC identifiers, concrete examples, and explicit units. Record one canonical source for each rule or task, linking from other pages. Requirements live in requirements.md, semantics in domain documents, decisions in ADRs, and work items only in backlog.md.

A numerical or durability claim needs a measurement boundary and evidence. Examples cannot silently assume funded accounts, zero fees, integer scales, or restart safety. Label prototypes and deferred behavior.

## Required detail by document

| Type | Minimum content |
| --- | --- |
| Requirement | Actor, trigger, observable behavior, acceptance, dependencies, evidence/status |
| Domain rule | ID, units, ordering, rejects, example, financial effect where relevant |
| ADR | Context, options, decision/status/date, consequences, revisit trigger |
| Feature brief | Problem, boundaries, rules/contracts, failures, acceptance, estimate uncertainty |
| Runbook | Scope/status, prerequisites, steps, expected output, failure handling, rollback/cleanup, verification date |
| Experiment | Question/hypothesis, timebox, environment/revision, reproduction, observations, limitations, conclusion |

Use [templates](../templates/README.md) for new ADRs, feature briefs, and runbooks. The existing research experiment template remains canonical for experiments.

## Review and validation

Update affected documents with code changes. Run python ops/scripts/check_docs.py and git diff --check. The checker validates local file links, UTF-8, and final newlines; it does not verify heading anchors, external websites, domain correctness, or whether commands are truthful. Review those manually and execute runnable procedures before labeling Verified.

Maintain docs/README.md as the navigation entry point. Avoid duplicated tutorials or renamed document directories with conflicting versions. Retain historical local init.md unchanged and ignored; it is input, not the maintained specification.

Path and language names follow [naming conventions](naming-conventions.md). Run python ops/scripts/check_naming.py for maintained path checks; source identifiers and serialized contracts still require language-specific checks and review.
