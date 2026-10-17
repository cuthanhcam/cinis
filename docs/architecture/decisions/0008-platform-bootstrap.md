# ADR-0008: Fail-closed platform bootstrap before transport integration

Date: 2026-10-17. Status: Accepted implementation baseline for the authorized scaffold.

## Context

The maintainer requested the .slnx and planned modules first, followed by sequential implementation. The architecture is accepted, but Rust framing/trust and public account/key/revocation contracts are still open. A fabricated execution response or public unauthenticated financial endpoint would misrepresent the delivered system.

## Decision

Create the five planned production projects and five meaningful test projects, root .NET toolchain/build/package policy and lockfiles together. Implement a trusted local grant-check capability, cancel application workflow and experimental bounded byte framing. The host provisions no grants, registers a disabled execution adapter, exposes only health routes, and reports trading readiness false. No database, API-key authentication, TCP Rust adapter or financial endpoint is introduced by this step.

The explicit grant implementation matches principal/account/action exactly; it establishes no inherited administrator privilege or public identity model. Existing business decisions remain open. The execution adapter reports unavailable-before-send, never financial success. Known, unknown and unavailable transport outcomes remain distinct in the application boundary. The frame codec's four-byte big-endian length is experimental, not acceptance of a production wire schema.

## Consequences and verification

The modules compose and run independently for development. Tests exercise actual dependency constraints, denied access, application forwarding boundaries, malformed/fragmented framing and host health. Runtime readiness stays false even though build/smoke are green. Platform joins the required hosted gate alongside Documentation, Engine, Ops and Repository. Logged finite smoke and bounded foreground Serve reuse the owned-process runner; no production/background supervisor is claimed.

The authorized order is recorded as a P2 bootstrap exception in the implementation plan. P1 protocol and P3 authentication/financial integration still need their own evidence. Revisit the disabled adapter only when an accepted contract and actual Rust transport exist. See the [platform guide](../../development/platform.md) for source, commands, test coverage and limitations.
