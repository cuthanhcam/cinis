# Experiment: first deterministic matching flow

Status: Completed baseline check, 2026-10-13. Code revision: see feat(engine) commit on feature/m1-executable-matching. Focused engineering effort was not measured.

## Question and hypothesis

Can an understandable safe-Rust core complete sell 10 at 100, buy 4 at 100, and cancel remaining 6 with deterministic events and no crossed resting book?

## Setup and reproduction

Windows host, x86_64-pc-windows-msvc; rustc 1.95.0 (59807616e 2026-04-14). One dependency-free crate; ordinary Vec storage and sorted preflight candidates. Use the commands in docs/development/testing.md. No hardware performance measurement was attempted.

## Observations

The CLI returned Accepted for order 1, Accepted plus Trade(maker 1, taker 2, price 100, quantity 4) for order 2, and Cancelled(order 1, quantity 6). Resting count was zero. Seventeen fixtures passed, with invariant checks after each command and unchanged Engine state on rejection. rustfmt and Clippy with warnings denied passed locally.

## Conclusion and limitations

The small matching flow is feasible and reproducible with safe Rust. Numeric and capacity policy is explicit in ADR-0002. This is not a financial, protocol, recovery, property-generator, or latency experiment. Scan/sort storage is intentionally simple; a separately implemented reference model is still needed before optimization.

## Follow-up

Complete M1-04/05 in backlog.md, then specify ledger/account bounds before M2. The .NET/Rust spike remains open and has no claimed result.
