# Development setup

## Available now

Git and Python 3.11 or later. From the repository root:

```sh
git status --short --branch
python scripts/check_docs.py
git diff --check
```

There is no Rust workspace, .NET solution, database deployment, or application start command yet.

## Toolchain baseline task

During M0 inspect installed SDKs, choose versions, and commit rust-toolchain.toml and global.json alongside actual manifests. Record versions and reproduction commands. Introduce rustfmt/Clippy and .NET formatting/build/test checks with executable projects. Avoid pipelines that imply nonexistent apps were validated.

## Local input

init.md is preserved historical planning input and intentionally ignored. English documents under docs/ are the maintained project baseline. The existing origin points to https://github.com/cuthanhcam/cinis.git; main is published first, followed by integrated develop work. No cloud resources, runtime ports, or secrets were configured.
