# Git workflow

| Branch | Purpose | Integration target |
| --- | --- | --- |
| main | Reviewed, reproducible baselines | none |
| develop | Next-milestone integration | main |
| docs/m0-specification | Refine product/domain specifications | develop |
| spike/rust-matching | Timeboxed safe-Rust experiment | develop if useful |
| spike/dotnet-rust-protocol | Framed command round-trip experiment | develop if useful |
| feature/m1-matching-core | First correct execution book | develop |
| chore/toolchain-ci | Pin SDKs and add actual build/test jobs | develop |

All prepared branches start at the bootstrap commit; they contain no hidden implementation. Only main and develop are long-lived. Update prepared work branches from develop before use and delete them after integration. Branches represent work, not separate architectures or environments.

## Daily work

```sh
git switch develop
git switch -c feature/short-description
# Edit and verify.
git add <specific-files>
git diff --cached
git commit -m "feat(engine): describe delivered behavior"
```

With a configured remote, fetch and update develop using `git pull --ff-only` before branching. Rebase private work as needed; coordinate before rewriting shared history. Use reviewed squash merges or deliberate merge commits. Never force-push main or develop.

## Releases

Create release/0.1.0 from develop only when stabilization begins. Merge the reviewed release into main, tag v0.1.0, and merge fixes back into develop. Use hotfix/<description> from main only for released defects. No release branch or version tag is created at bootstrap.

## Remote and protections

An existing origin is configured as https://github.com/cuthanhcam/cinis.git. Its hosted state has not been verified; bootstrap does not create a remote repository or push changes. Verify with `git remote -v`, then publish when intended using `git push -u origin <branch>`. Push work branches only when needed.

On the host, set main as default; require pull requests, resolved discussions, and Documentation checks for main/develop; block force pushes. These are recommendations, not locally installed protections.
