# ADR-0011: Browser response headers for the API-only host

Date: 2026-10-20. Status: Accepted scaffold implementation following the maintainer's pipeline-strategy review.

## Context

The host currently returns JSON/Problem Details and has no HTML UI, authenticated financial endpoint or deployed TLS/proxy topology. The supplied pipeline note identifies security headers as a distinct response concern. Existing IExceptionHandler and diagnostic middleware already implement error/correlation behavior.

## Decision

Add a host-owned SecurityHeadersMiddleware before routing. Register final headers with OnStarting so success and translated error responses receive the same policy: nosniff, no-referrer, DENY framing and a CSP denying resource loading and framing. Assign header values rather than append duplicate values. Existing health no-store behavior remains an endpoint policy.

Keep existing diagnostics/error composition and generated request IDs. Do not trust arbitrary incoming correlation IDs as identity, log raw URLs/bodies, introduce an additional AsyncLocal context, or add a second exception handler. The existing Activity and ILogger scopes provide the current diagnostic context.

## Evidence and consequences

Integration tests cover actual health 200/503 headers, test endpoints returning 200/404/500 and an endpoint attempting to weaken the policy. They also confirm this HTTP scaffold does not emit an HSTS policy. Security headers do not complete authentication, proxy trust or API authorization. Frame policy is primarily relevant to browser-rendered content; JSON financial access still requires the existing security gates.

Future HTML/interactive documentation endpoints must review the restrictive CSP. Production TLS/HSTS, proxy behavior and any edge-level header policy require a deployment decision rather than assumptions in local middleware. See [pipeline contract](../../development/platform-http-pipeline.md) and [foundation review](../platform-foundation-review.md).
