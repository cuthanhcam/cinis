# Contributing to Cinis

Cinis is a specification-led multi-language engineering lab. Read the README, scope, current backlog, affected rules, and AGENTS.md before changing behavior. Maintained prose/code/comments/commits use English. No released production service exists.

## Choose focused work

Use docs/delivery/backlog.md as the single implementation work-item source. Issues collect reports/proposals; they do not duplicate delivery tracking. Link the affected FR/QR/BR/INV/SC/MC IDs. Agree on the observable behavior and failure cases before implementing financial or recovery semantics.

Create a focused branch from current develop: feature/, fix/, docs/, chore/, or test/. Keep one coherent concern per commit; separate substantial policy/scaffold changes from trading behavior. Follow [naming](docs/development/naming-conventions.md) and [commit conventions](COMMIT_MESSAGE_GUIDELINES.md).

## Local checks

On Windows use the logged Verify task, then ops unittest discovery. Other hosts use the equivalent direct Cargo/Python commands in README and testing.md. Workflow changes must pass pinned actionlint. Do not update a lockfile or bypass --locked simply to hide a reproducibility problem.

For behavior changes add meaningful fixture/property/reference evidence. Update rules, examples, ADRs, runbooks, and backlog status together. Matching-only tests cannot establish balances, ledger, retries, or recovery guarantees. Keep generated logs/builds, private init.md, local databases, and credentials out of commits.

## Hosted integration gate

Push the work branch first; all four required workflows must exist and succeed for its exact SHA. For integration, create a candidate branch from latest develop, merge work into that branch, and push it. Wait for Documentation, Engine, Ops, and Repository to pass on the candidate merge SHA. Use check_ci.py --wait; missing, skipped, cancelled, stale, or failed results block integration.

Only then fast-forward develop to that same tested candidate commit and push. A fresh merge after checking another SHA creates an untested integration commit and does not meet this gate. If develop advanced meanwhile, rebuild/retest a candidate. See [CI procedure](docs/development/ci.md).

The maintainer creates/reviews the develop-to-main PR. Do not push directly to main, assume local branch protection exists, disable tests, or use continue-on-error to turn a required failure green.

## Pull request evidence

Use the template. Describe the concrete problem and resulting behavior, exact local commands/results, hosted run URLs/SHA, specification links, material limitations, and rollback/migration if needed. Keep unimplemented guarantees explicit and do not check validation boxes for future work.

## Definition of done

Acceptance behavior and relevant failure cases verified; linked docs match code; local required gates pass; required GitHub workflows pass on the exact integration commit; no secrets/generated/unrelated files; review findings addressed. Documentation completeness is separate from application completeness. Hosting ruleset/required reviewers are configured by the maintainer, not installed by these files.

Security concerns follow SECURITY.md. Questions follow [.github/SUPPORT.md](.github/SUPPORT.md).
