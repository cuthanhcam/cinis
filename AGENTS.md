# Cinis agent guide

Read README.md, docs/README.md, and relevant specifications before editing.

- Use English for maintained docs, code, comments, and commits.
- Preserve init.md as ignored local history; do not stage or modify it.
- Keep financial state under one execution owner; projections cannot approve spending.
- Use checked integer arithmetic for ticks, lots, and asset atoms.
- Prefer safe Rust and straightforward collections before optimization.
- Distinguish planned behavior from delivered, verified functionality.
- Update rule IDs, scenarios, and ADRs when semantics change.
- Never commit credentials, generated output, or unsupported benchmark claims.
- Run python scripts/check_docs.py and git diff --check for docs changes. Add actual build/test checks when application projects exist.
- Follow GIT_GUIDELINES.md. Do not invent remote configuration or publish without authorization.

- Use docs/ as the single documentation root and ops/ for process automation; follow docs/development/repository-layout.md.
- Keep artifacts/logs ignored. Never log environment dumps or credentials; diagnostic logs are not execution journals.
- For runner changes run python -m unittest discover -s ops/tests -v on Windows and the public Verify/Run tasks.
