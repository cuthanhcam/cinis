# Commit message conventions

Use English and the form `type(scope): imperative description`. Describe the delivered change, not a plan. Keep the subject focused; use the body for context, behavior, acceptance IDs, actual verification, and limitations.

## Types and scopes

| Type | Use |
| --- | --- |
| feat / fix | Observable feature or defect correction |
| test | New meaningful verification or regression evidence |
| docs | Specifications, decisions, guides, runbooks |
| refactor | Structure/internal change without intended behavior change |
| perf | Measured performance change with reproducible evidence |
| build / ci | Toolchain/build inputs or hosted verification |
| chore / revert | Repository maintenance or deliberate reversal |

Useful scopes: engine, platform, protocol, domain, recovery, ops, repo, docs, ci. Omit a scope when it adds no clarity. Avoid combining unrelated work just to reduce the commit count; avoid a separate commit for every file.

## Examples

- `fix(ops): load built-in modules from the active PowerShell host`
- `test(engine): compare partial cancellation with the independent model`
- `docs(domain): specify reservation release after a partial fill`
- `ci(repo): require hosted verification on the integration commit`

## Breaking changes and evidence

Mark breaking public behavior/contracts with ! and a BREAKING CHANGE footer explaining migration. A naming refactor must not silently change wire names. An Unreleased changelog entry is not a version release; tags/releases follow the reviewed release process.

Include backlog/rule IDs when they aid tracking. Never include credentials, private payloads, or unsupported performance/durability claims. Merge commits describe the integrated concern and retain branch history. Do not rewrite published main/develop history to hide failed CI or remove review evidence.
