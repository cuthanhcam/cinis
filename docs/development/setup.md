# Development setup

## Available now

Git, Python 3.11 or later, and rustup. The repository pins Rust 1.95.0 with rustfmt/Clippy. On Windows install the MSVC C++ build tools required by the Rust MSVC toolchain. From the repository root:

```sh
git status --short --branch
python scripts/check_docs.py
git diff --check
```

The Rust crate at src/engine implements a matching-only CLI; see [testing](testing.md) for commands. There is no .NET solution, database deployment, network gateway, or durable application yet.

## Toolchain baseline task

Rust 1.95.0 and components are pinned in rust-toolchain.toml; Cargo.lock is committed for this application crate. Engine CI runs format, Clippy, tests, and demo. During the .NET spike inspect installed SDKs and introduce global.json, actual solution/projects, and build/test CI together. No placeholder platform workflow or environment file is needed yet.

## Local input

init.md is preserved historical planning input and intentionally ignored. English documents under docs/ are the maintained project baseline. The existing origin points to https://github.com/cuthanhcam/cinis.git; main is published first, followed by integrated develop work. No cloud resources, runtime ports, or secrets were configured.
