---
title: Browser sessions
available: false
---

# Browser sessions

A session records one successful signup or signin. Core authentication now persists sessions and rotates refresh credentials. Browser cookie transport and OAuth remain planned; the current signup API returns credentials in JSON. The [login ADR](adr.md) explains the decisions behind this flow.

## What a session represents

A second login creates another session; refreshing credentials keeps the same session. Each record identifies its owner, current refresh credential, original login time, last refresh time, expiry, and whether it has been revoked. Session history distinguishes a new login from continued activity.

Device information is optional. A browser description or user-provided label may eventually help recognize a session, but neither proves which physical device or person is using it. Authentication does not require device fingerprinting.

The refresh JWT carries the user ID, stable session ID, rotating credential identifier, and validity timestamps. The complete JWT is not stored. Access JWTs carry the user ID, organization ID, and membership role; refresh credentials contain no organization claims.

## Renewal and workspace selection

Successful refresh replaces both credentials and extends the session for 30 days. It preserves the session ID and original login time. A user must sign in again after 30 days without successful refresh.

Before renewal, the server verifies the signed credential, session status, user status, and current organization membership. Missing, revoked, expired, or foreign sessions cannot refresh. Only the current refresh credential may advance a session; concurrent requests cannot both replace it.

The frontend supplies the selected organization during routine refresh. If omitted, use the oldest active membership, breaking ties by organization ID. Signin starts in an owner/admin organization; later switching may select an organization where the user is a member. New access credentials reflect the current role.

## Reuse, logout, and failure recovery

Presenting an already-used, valid refresh credential revokes the whole session, including credentials issued by an earlier renewal. This applies to suspected theft, competing refresh requests, and retrying after a lost rotation response. The client must coordinate renewal across requests and tabs. There is no grace period; recovery requires signin.

Logout revokes the current session and clears its cookies when cookie transport is implemented. Administrative revocation blocks further renewal. Sessions from other logins remain independent.

A copied access JWT may still work until its short expiry, including its cached role. Immediate invalidation requires an additional server-side check. Invalid signatures and expired JWTs do not authorize changes to another session.

## Planned browser transport

Axum will set HttpOnly cookies after authentication. Frontend code will obtain identity information from the API without reading reusable credentials.

| Cookie | Expiry | Production attributes |
| --- | --- | --- |
| `__Host-access` | 5–15 minutes; exact lifetime undecided | `HttpOnly; Secure; SameSite=Lax; Path=/` |
| `__Host-refresh` | 30 days, renewed after successful refresh | `HttpOnly; Secure; SameSite=Lax; Path=/` |

Production requires HTTPS, no `Domain` attribute, and matching cookie and credential expiries. Local HTTP development uses separate settings without weakening production protections.

State-changing browser requests, including login confirmation, refresh, and logout, must pass Origin validation. Keep GET free of mutations. `SameSite=Lax` supplements CSRF protection; OAuth callbacks require request binding, including state validation.

HttpOnly blocks cookie reads, but injected scripts can still make requests. Safe rendering and a Content Security Policy remain necessary. Keep credentials out of browser storage and logs, and prevent caching of private identity or credential-setting responses.

## Existing users

Refresh tokens issued before sessions carried a session ID cannot be renewed through this flow; those users must sign in again. Old last-login timestamps remain for compatibility and are no longer updated. Historical session records are not inferred from them. Session-history retention remains undecided.

See [Email](email.md), [GitHub](github.md), and [GitLab](gitlab.md) for login-method details.
