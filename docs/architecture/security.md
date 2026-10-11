# Security model

Authenticate API keys at the untrusted client/gateway boundary. Check read/trade scopes and account ownership; validate and rate-limit ingress. Administration and synthetic funding require separate privileges. Management never grants arbitrary execution-storage mutation.

Store verifiable hashes for bearer secrets, display once, and support revocation. Signed requests, if introduced, require separate canonicalization, timestamp, replay, secret-storage, and rotation contracts. Idempotency is not authentication or replay protection.

Restrict engine/database access to trusted local services for the first deployment. Never expose the engine port publicly. Exclude credentials from logs. Validate lengths, bounds, and versions across process boundaries.

Planned tests: wrong account cannot query/cancel; read key cannot trade; revoked key fails; trading key cannot fund; malformed input cannot mutate state. Identity-provider selection and Aegis integration remain deferred.
