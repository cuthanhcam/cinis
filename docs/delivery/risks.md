# Risk register

| ID | Risk / likelihood / impact | Trigger | Mitigation |
| --- | --- | --- | --- |
| R-01 | Rust learning / high / schedule | Spike cannot complete basic model | Safe collections, scope reduction, re-estimate |
| R-02 | Split financial ownership / medium / critical | Gateway/projection approves spending | Single execution owner and invariant review |
| R-03 | Recovery ambiguity / high / critical | ACK/sync boundary undefined | Explicit contract and crash tests |
| R-04 | Premature optimization / high / high | Shared memory/unsafe before correctness | Baseline first and evidence-gated research |
| R-05 | Scope expansion / high / high | Multi-market/real funds enter v0.1 | Scope review and explicit deferral |
| R-06 | Protocol mismatch / medium / high | Rust/C# interpret units differently | Shared fixtures and malformed-frame tests |
| R-07 | Misleading benchmarks / medium / high | Core numbers presented as end-to-end | Separate boundaries/raw reproducible data |
| R-08 | Unbounded history / medium / medium | Journal/dedup exceeds storage budget | Explicit lab budget; halt safely; retention design later |

Maintainer owns this register. Review at milestones and record triggered risks, actual effort, and mitigation changes.
