---
title: Browser sessions
available: false
---

# Browser sessions

A session records one successful signup or signin. Core authentication now persists sessions and rotates refresh credentials. Browser cookie transport and OAuth remain planned; the current signup API returns credentials in JSON. The [login ADR](adr.md) explains the decisions behind this flow.

## What a session represents

A second login creates another session; refreshing credentials keeps the same session. Each record identifies its owner, current refresh credential, original login time, last refresh time, expiry, and whether it has been revoked. Session history distinguishes a new login from continued activity.

Device information is optional. A browser description or user-provided label may eventually help recognize a session, but neither proves which physical device or person is using it. Authentication does not require device fingerprinting.

Both token types reference the same database session using its ID, organization ID, and user ID. Access JWTs additionally carry the current membership role; refresh JWTs carry the rotating credential identifier. The complete JWT is not stored. Every lookup, renewal, and revocation uses the organization and user together, supported by a compound organization/user index.

## Renewal and workspace selection

Successful refresh replaces both credentials and extends the session for 30 days. It preserves the session ID and original login time. A user must sign in again after 30 days without successful refresh.

Before renewal, the server verifies the signed credential, session status, user status, and current organization membership. Missing, revoked, expired, or foreign sessions cannot refresh. Only the current refresh credential may advance a session; concurrent requests cannot both replace it.

Refresh loads the database session referenced by the verified token. The session's refresh behavior rejects revoked or expired state and rotates the credential. Saving succeeds only if the presented credential is still current; tokens are returned after that save commits.

If no organization is selected, keep the session's current organization. An explicit switch first verifies the target membership, then moves the same session and issues both credentials with its new reference. Signin starts in an owner/admin organization; later switching may select a member organization. New access credentials reflect the current role.

## Reuse, logout, and failure recovery

Presenting an already-used, valid refresh credential in the session's current organization revokes the whole session, including credentials issued by an earlier renewal. This applies to suspected theft, competing refreshes, and retrying after a lost response. After an organization switch, tokens referencing the previous organization fail lookup and cannot mutate the session in its new organization. The client must coordinate renewal across requests and tabs. There is no grace period; recovery from revocation requires signin.

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
