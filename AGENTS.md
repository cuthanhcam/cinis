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
