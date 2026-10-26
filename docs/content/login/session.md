---
title: Browser sessions
available: false
---

# Browser sessions

A session records one successful signup or signin. Core authentication persists sessions, renews JWTs without database writes, and changes organization through a separate operation. Browser cookie transport and OAuth remain planned; the current signup API returns credentials in JSON. The [login ADR](adr.md) explains the decisions behind this flow.

## What a session represents

A second login creates another session; refreshing credentials keeps the same session. Each record identifies its organization, owner, current credential identifier, original login time, and revocation state. Expiry comes from the signed JWT; normal refresh does not write an activity timestamp or expiry to the database. Session history distinguishes a new login from continued activity.

Device information is optional. A browser description or user-provided label may eventually help recognize a session, but neither proves which physical device or person is using it. Authentication does not require device fingerprinting.

Both token types embed the same Session struct. Session owns its ID, organization ID, user ID, current credential identifier, original login time, and revocation state; these fields are not repeated at the token level. Access JWTs additionally carry the current membership role. A signed snapshot does not replace the database record: refresh and organization switching load the current session to enforce revocation and credential changes. The complete JWT is not stored. Every lookup, renewal, and revocation uses the organization and user together, supported by a compound organization/user index.

## Read-only renewal

Normal refresh validates the signed JWT, loads its organization/user-scoped session, rejects revocation or a credential mismatch, and checks the active user and current membership. It issues new JWTs for the same session and organization, with the same credential identifier and a new 30-day refresh expiry.

There is no database update, credential consumption, automatic revocation, or transaction started by renewal. It can run inside an existing transaction. Expired JWTs are already invalid and do not require revoking the session.

Older refresh JWTs remain usable until their own expiry. Each valid renewal starts another 30-day JWT lifetime, so continued use can extend beyond 30 days from the original login without a fixed database deadline. A holder, including someone with a copied JWT, can keep renewing before expiry until explicit revocation or a session change.

## Organization switching

Switching is a separate operation. Verify the current refresh credential and target membership, change the session organization and credential identifier, save conditionally, and return both replacement tokens after commit. Signin starts in an owner/admin organization; switching may select a member organization. Previous refresh credentials are invalidated; already-issued access JWTs retain the short-expiry limitation described below.

Old-organization credentials no longer resolve the moved session. Even switching to the same organization replaces the identifier, invalidating previous refresh credentials. Concurrent switches cannot both save changes from the same previous state; a losing request is rejected without revoking the winner.

Ordinary renewal never changes organizations. It uses the current role when issuing an access JWT.

## Logout and revocation

Logout and administrative actions explicitly revoke the session. Logout also clears its cookies when cookie transport is implemented. Other login sessions remain independent.

A copied access JWT can remain usable until its short expiry, including its cached role. Immediate access-token invalidation requires an additional server-side check. Failed renewal or switching does not automatically revoke a session, and invalid signatures or expired JWTs never authorize changes.

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

Refresh tokens issued without an embedded session cannot be renewed through this flow; those users must sign in again. Old last-login timestamps remain for compatibility and are no longer updated. Historical sessions are not inferred from them. Original login age does not measure inactivity because renewal is read-only; session cleanup needs a separate retention policy, which remains undecided.

See [Email](email.md), [GitHub](github.md), and [GitLab](gitlab.md) for login-method details.
