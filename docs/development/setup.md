# Development setup

## Available now

Git, Python 3.11 or later, and rustup. The repository pins Rust 1.95.0 with rustfmt/Clippy. On Windows install the MSVC C++ build tools required by the Rust MSVC toolchain. From the repository root:

```sh
git status --short --branch
python ops/scripts/check_docs.py
git diff --check
```

The Rust crate implements a synthetic financial CLI and matching-only CLI. The .NET [platform solution](../../src/platform/Cinis.Platform.slnx) now builds and runs a health-only scaffold; see the [platform guide](platform.md). Install SDK 10.0.301 or a permitted same-band patch from global.json. Database deployment, authenticated financial gateway and durable execution remain pending.

## Toolchain baseline task

Rust is pinned in rust-toolchain.toml with committed Cargo.lock. .NET global.json, shared build/central package policy, per-project package locks and the Platform workflow are implemented together. Locked restore, Release build, format, tests and smoke are documented in the platform guide. Both engine and platform run on Linux/Windows CI.

## Local input

init.md remains ignored historical input. English docs are maintained specifications. The existing origin is https://github.com/cuthanhcam/cinis.git; develop receives only tested integration commits. No cloud resources or secrets are provisioned. The local platform health host defaults to loopback port 5080.

## Automated local commands

Use [ops/](../../ops/README.md) on Windows for logged Build/Test/Run/Verify. The runner resolves the repository from its own path and does not require a particular caller working directory. Detailed layout and naming are defined in [repository layout](repository-layout.md).

## Scaffold tooling and environment

Shared editor suggestions live under .vscode/; private editor state remains ignored. .env.example is reference documentation for GH_TOKEN and RUST_BACKTRACE, not an automatically loaded environment file. First workflow validation downloads checksum-pinned actionlint for Windows/Linux x86_64. See [scaffold readiness](scaffold.md) and [hosted CI gate](ci.md).
