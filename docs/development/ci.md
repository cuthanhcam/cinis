# Continuous integration and tested integration commits

Status: Implemented configuration and read-only gate. Owner: repository maintainer. Updated: 2026-10-17. Hosted results are verified per SHA rather than inferred from this status.

## Required workflows

| Workflow | Host / purpose | Evidence |
| --- | --- | --- |
| Documentation | Ubuntu 24.04; links/text, path naming, portable naming tests, whitespace | Whole workflow success |
| Engine | Ubuntu 24.04 and Windows 2025; pinned Rust, format, Clippy, tests, demo | Both matrix jobs succeed |
| Platform | Ubuntu 24.04 and Windows 2025; locked restore, format, warnings-as-errors build, unit/integration/architecture tests and smoke | Both matrix jobs succeed |
| Ops | Windows 2025; automation tests, public Verify and Run; diagnostic artifact upload | Tests and runs succeed; upload is not a substitute |
| Repository | Ubuntu 24.04; pinned/checksum-verified actionlint and CI-gate tests | Syntax/expressions and fail-closed gate verified |

All run on every pushed branch, pull request, and manual dispatch. No path filter can silently omit a required workflow. Concurrency cancels older runs for the same workflow/ref; a cancelled result is not success. Job timeouts bound waits. External actions are pinned to reviewed full SHAs with release comments; Dependabot updates them through reviewable PRs.

## Before publishing develop

1. Run local Verify and ops unit/integration tests; inspect the staged diff.
2. Push the work branch. Check all five workflows on its exact SHA.
3. Create integration/<description> from latest develop and merge the work branch there. Push this candidate; it contains the merge commit develop would receive.
4. Run `python ops/scripts/check_ci.py --wait` while on the candidate. The script only reads GitHub; it never merges or pushes. It requires all five named push workflows for exactly HEAD. Exit 0 means all succeeded; exit 1 means a failed/skipped/cancelled result; exit 2 means missing/pending/timeout. Evidence is saved to ignored artifacts/ci/<sha>.json.
5. Recheck that develop has not advanced. Switch to develop and `git merge --ff-only integration/<description>`. Verify HEAD is the tested SHA, then `git push origin develop`.
6. Observe develop's own runs too. A failure on that ref still requires investigation; do not assume tests eliminate all platform or network variability.

Do not check a work-branch SHA then create an untested merge directly on develop. If integration content changes, the SHA changes and the gate must run again. Do not force-push protected/integration branches to conceal failed evidence. main remains a separate maintainer PR/review action.

## API access and reproducibility

check_ci.py uses GitHub's workflow-runs API. Public repositories work without a token but are rate-limited. For private repositories or extended polling supply GH_TOKEN/GITHUB_TOKEN through the calling environment; the script never prints or stores it. Network/API errors fail rather than pretending results are green. Default wait is 900 seconds with 20-second polling; timeout is configurable up to 3600 seconds.

No local script installs hosting protections. The maintainer should require the named status checks, restrict main/develop pushes, disallow force pushes, and require resolved review discussions. CODEOWNERS becomes an enforced review control only when the ruleset requires it.

## Known hosted failure and regression

Ops run 38759169205 failed because a Python test launched Windows PowerShell 5.1 from a PowerShell 7 environment; inherited PSModulePath resolved built-in modules incorrectly, making Import-PowerShellDataFile unavailable. Public scripts and the process module now import the running host's built-ins by full PSHOME path. The regression test shadows the Utility module with an incompatible one. This fixes the actual host interaction rather than skipping the test or relying on a different shell.

The initial fix was verified on work-branch hosted runs 38760612316 (Documentation), 38760612279 (Engine), and 38760612236 (Ops). Those results validate that earlier SHA only; the final scaffold/integration SHA must independently pass all five workflows.

## Workflow lint tooling

check_workflows.py downloads actionlint 1.7.12 for Windows/Linux x86_64 from its official release, validates the pinned archive SHA256, extracts only the expected binary, and checks every workflow. Archives are cached under ignored artifacts/tools and revalidated. Optional shellcheck/pyflakes integration is disabled uniformly so the mandatory gate is reproducible across hosts; this does not claim full shell/Python source linting. First use needs internet access.

## References

- [GitHub workflow-runs API](https://docs.github.com/en/rest/actions/workflow-runs): hosted execution evidence.
- [GitHub secure-use guidance](https://docs.github.com/en/actions/reference/security/secure-use): immutable action references and limited permissions.
- [Microsoft PSModulePath](https://learn.microsoft.com/en-us/powershell/module/microsoft.powershell.core/about/about_psmodulepath): host-specific module resolution.
