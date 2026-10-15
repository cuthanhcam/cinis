# Contributing

Read the charter, domain rules, and current milestone. Start focused work from develop and use English. Explain delivered behavior, rationale, and verification in each pull request.

Update affected rules and acceptance cases together. Record significant architecture changes in an ADR. Report experiments with their limitations.

Before review, run `python ops/scripts/check_docs.py` and `git diff --check`. When executable projects exist, run relevant build and correctness tests and record actual results.

Definition of done: acceptance criteria satisfied, failure cases checked, docs updated, no secrets/generated files, review findings resolved. Documentation completion does not imply implementation completion.

See [Git guidelines](GIT_GUIDELINES.md), [commit conventions](COMMIT_MESSAGE_GUIDELINES.md), and [security](SECURITY.md).

Follow [naming conventions](docs/development/naming-conventions.md): shared paths use repository defaults, while C#, Rust, Python, and PowerShell keep their language-specific casing. Automation belongs under ops/scripts/; operational documentation belongs under docs/operations/.
