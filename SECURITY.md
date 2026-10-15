# Security policy

## Supported scope

Cinis has no supported released version or production deployment. Current execution is a volatile matching-only prototype; v0.1 is a synthetic-funds sandbox. It does not provide custody, real-asset trading, production auth, or durable financial recovery.

## Private reporting

Do not post exploitable details, tokens, API secrets, private keys, or private account data in public issues. Use GitHub private vulnerability reporting if the maintainer has enabled it. Otherwise establish a private reporting channel with the maintainer through their GitHub profile; no dedicated reporting address or response-time SLA is configured. Do not invent an email contact.

A useful report includes affected revision, environment, minimal reproduction, trust boundary, observed/expected impact, and suggested mitigation. Avoid unnecessary private data. Coordinate public disclosure after triage and a remediation plan; no turnaround guarantee is implied.

## Repository controls

Credentials belong in approved environment/secret mechanisms, not source, command arguments, fixtures, logs, or examples. Review diagnostic output before sharing. If a credential is exposed, revoke/rotate it through its issuer; deleting a commit is not revocation.

Workflows use read-only repository contents, immutable action commit references, and separate untrusted PR/push validation without production secrets. Dependabot proposes reviewable dependency updates; it does not auto-merge. Required hosted checks must pass before integration. Formal hosting rules and private-reporting settings still require maintainer configuration.

## Runtime boundaries

The gateway/security architecture is planned. Restrict future engine access to trusted components and validate ownership, sizes, numeric bounds, and protocol versions. PostgreSQL projections never authorize spending. Diagnostic logs are not a journal or audit ledger. The [security model](docs/architecture/security.md) and durability specification define requirements to implement and verify later.
