# Service lifecycle requirements

Status: Proposed extension contract; no persistent Cinis service exists. Updated: 2026-10-15.

## Current behavior

Engine Run is a finite synthetic-financial CLI demonstration; RunMatching retains the raw matching-only example. It starts, emits output, and exits. It opens no network port and requires no separate stop operation. Process success means completion only; it says nothing about service readiness or financial durability.

## Gate for the first gateway/execution service

| Concern | Required behavior and evidence |
| --- | --- |
| Identity | Unique service/run ID; owned process handle; command revision and instance identity |
| Readiness | Check an application readiness endpoint/protocol, not merely PID existence; distinguish liveness/readiness |
| Configuration | Document ports, binding addresses, required variables, safe defaults; add .env.example only with actual configuration |
| Logging | Structured service events with UTC time, level, service/run ID, request/command correlation; redact secrets |
| Shutdown | Application-cooperative shutdown first; bounded grace; stop owned tree only as fallback |
| Recovery | Before accepting commands, complete documented replay/invariant checks |
| Concurrency | Reject duplicate instance ownership/port conflicts; never kill unrelated instances |
| Security | Local-only defaults and explicit trust boundaries; no credentials in CLI arguments |
| Retention | Bounded log volume, rotation/quotas, and documented archive policy |
| Testing | Start-ready-run-stop, failed readiness, crash, restart, stale identity, and duplicate-instance tests |

Do not reuse a stale PID file as sole proof of ownership. A background mode needs verified process identity and explicit stop/status semantics. Production hosting should use the intended operating-system/container supervisor, not a PowerShell terminal kept open indefinitely.

Aegis integration remains optional and contract-based. Cinis must start independently for local verification. Detailed commands are added when the executable services exist; this page is not an executable service runbook.
