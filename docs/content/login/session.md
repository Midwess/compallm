---
title: Browser sessions
available: false
---

# Browser sessions

A session records one successful signup or signin. Core authentication persists sessions, renews JWTs without database writes, and changes organization through a separate operation. Signup/signin confirmation supports cookie and JSON token delivery. Protected APIs accept access cookies or Bearer access tokens, with automatic cookie renewal. Logout endpoints and OAuth remain planned. The [login ADR](adr.md) explains the decisions behind this flow.

## What a session represents

A second login creates another session; refreshing credentials keeps the same session. Each record identifies its organization, owner, current credential identifier, original login time, and revocation state. Expiry comes from the signed JWT; normal refresh does not write an activity timestamp or expiry to the database. Session history distinguishes a new login from continued activity.

Device information is optional. A browser description or user-provided label may eventually help recognize a session, but neither proves which physical device or person is using it. Authentication does not require device fingerprinting.

Both token types embed the same Session struct. Session owns its ID, organization ID, user ID, current credential identifier, original login time, and revocation state; these fields are not repeated at the token level. Access JWTs additionally carry the current membership role. A signed snapshot does not replace the database record: refresh and organization switching load the current session to enforce revocation and credential changes. The complete JWT is not stored. Every lookup, renewal, and revocation uses the organization and user together, supported by a compound organization/user index.

## Read-only renewal

Normal refresh validates the signed JWT, loads its organization/user-scoped session, rejects revocation or a credential mismatch, and checks the active user and current membership. It issues new JWTs for the same session and organization, with the same credential identifier and a new 30-day refresh expiry.

There is no database update, credential consumption, automatic revocation, or transaction started by renewal. It can run inside an existing transaction. Expired JWTs are already invalid and do not require revoking the session.

Older refresh JWTs remain usable until their own expiry. Each valid renewal starts another 30-day JWT lifetime, so continued use can extend beyond 30 days from the original login without a fixed database deadline. A holder, including someone with a copied JWT, can keep renewing before expiry until explicit revocation or a session change.

## Protected API requests

Public signup/signin code-request routes require no credentials. Their confirmation routes require a Bearer email-OTP challenge. Middleware verifies the selected credential and inserts its concrete type as a request extension: `UserAccessToken`, `RefreshToken`, or decrypted `EmailOtpAccessToken`. Handlers declare `RequireToken<T>` to require exactly that type; an absent type returns 401. Raw request access uses `extensions().get::<T>()`. User-token extraction exposes organization/role guards returning 403. A route on the authenticated router must explicitly require the token type appropriate for its operation. `GET /api/auth/me` returns session ID, user ID, organization ID, and role without returning credentials.

```rust
async fn handler(RequireToken(token): RequireToken<UserAccessToken>) {
    // A verified user access token is present; refresh/OTP tokens cannot satisfy this.
}
```

The concrete token is also available through `request.extensions().get::<UserAccessToken>()`. Middleware replaces token extensions on each request so the selected Bearer/cookie identity cannot inherit another credential type from an earlier layer.

An explicit Authorization header takes precedence over cookies. Require one signed, unexpired Bearer token; malformed, expired, or duplicate credentials return 401 without cookie fallback or renewal. Only its actual verified type is inserted. A refresh or OTP token cannot satisfy `RequireToken<UserAccessToken>`. Refresh extraction verifies the signed claims, not current database state; renewal and switching still use Core's session checks. OTP extraction verifies the encrypted challenge, not email ownership; confirmation must still match the emailed code. Confirmation handlers require `RequireToken<EmailOtpAccessToken>` and pass the verified claims to Core; JSON contains only the code and delivery selection. The legacy body token field is rejected.

Cookie access is validated without querying the database, preserving the documented short-lived cached-role tradeoff. If access is missing or expired, validate the refresh cookie through Core, use the new access claims for the current request, and set both replacement cookies on the response. Invalid signatures, wrong variants in the access-cookie slot, or ambiguous access cookies are rejected. With valid cookie access, a valid refresh cookie is also inserted as `RefreshToken` only when it matches that access token's session and credential identifier. Invalid or unrelated refresh cookies do not affect valid access, but cannot satisfy refresh extraction. After automatic renewal, both new token types are inserted. Bearer mode never adds cookie credentials. Refresh performs the existing database session/user/membership checks without a write. Any middleware authentication failure in cookie mode clears both authentication cookies, including Origin rejection or temporary storage failure. Bearer-mode errors leave browser cookies untouched. Failed renewal does not revoke sessions.

Cookie-authenticated mutations require an allowed Origin. Automatic renewal also requires an allowed Origin, or, for safe GET/HEAD requests without Origin, the browser's `Sec-Fetch-Site: same-origin`. This supports ordinary same-origin GET fetches without allowing cross-site requests to renew cookies. Bearer-only authentication does not use ambient cookies and does not require browser Origin validation. Protected responses use `Cache-Control: no-store`; 401 responses advertise Bearer authentication. Frontend assets and public routes never trigger automatic renewal.

## Organization switching

Switching is a separate operation. Verify the current refresh credential and target membership, change the session organization and credential identifier, save conditionally, and return both replacement tokens after commit. Signin starts in an owner/admin organization; switching may select a member organization. Previous refresh credentials are invalidated; already-issued access JWTs retain the short-expiry limitation described below.

Old-organization credentials no longer resolve the moved session. Even switching to the same organization replaces the identifier, invalidating previous refresh credentials. Concurrent switches cannot both save changes from the same previous state; a losing request is rejected without revoking the winner.

Ordinary renewal never changes organizations. It uses the current role when issuing an access JWT.

## Logout and revocation

Logout and administrative actions explicitly revoke the session. Logout also clears its cookies when cookie transport is implemented. Other login sessions remain independent.

A copied access JWT can remain usable until its short expiry, including its cached role. Immediate access-token invalidation requires an additional server-side check. Failed renewal or switching does not automatically revoke a session, and invalid signatures or expired JWTs never authorize changes.

## Credential delivery

Signup and signin confirmation require a Bearer OTP challenge and accept optional `delivery` in JSON, defaulting to `cookie`. Axum chooses transport only after Core authentication succeeds and the session commits. The mode is never stored in Session or JWT claims.

- `cookie`: set both HttpOnly cookies and return `{}`; token values are absent from JSON.
- `token`: return `{ "accessToken": "…", "refreshToken": "…" }` and set no cookies. This does not clear an existing browser session.

Both require JSON input. Cookie confirmation requires an allowed Origin; token mode does not rely on browser cookies. Protected API middleware reads these cookies and renews them automatically when needed.

| Cookie | Expiry | Production attributes |
| --- | --- | --- |
| `__Host-access` | 15 minutes (configured access lifetime) | `HttpOnly; Secure; SameSite=Lax; Path=/` |
| `__Host-refresh` | 30 days, renewed after successful refresh | `HttpOnly; Secure; SameSite=Lax; Path=/` |

Production requires HTTPS, no `Domain` attribute, and matching cookie and credential expiries. `AUTH_COOKIE_SECURE` defaults to true in production and false in development/tests. Nonsecure local cookies use `compallm-access` and `compallm-refresh` names. `AUTH_PUBLIC_ORIGIN` sets the exact browser origin, including scheme and port, and is required for cookie confirmation in production. No forwarded headers determine cookie security or the allowed origin.

With an empty configured origin in nonsecure local development, only a loopback HTTP Origin matching the request Host is accepted, supporting Hero's selected port. Missing, opaque, or untrusted origins receive 403 before authentication or persistence.

State-changing browser requests, including login confirmation, refresh, and logout, must pass Origin validation. Keep GET free of mutations. `SameSite=Lax` supplements CSRF protection; OAuth callbacks require request binding, including state validation.

HttpOnly blocks cookie reads, but injected scripts can still make requests. Safe rendering and a Content Security Policy remain necessary. Keep credentials out of browser storage and logs, and prevent caching of private identity or credential-setting responses.

## Existing users

Refresh tokens issued without an embedded session cannot be renewed through this flow; those users must sign in again. Old last-login timestamps remain for compatibility and are no longer updated. Historical sessions are not inferred from them. Original login age does not measure inactivity because renewal is read-only; session cleanup needs a separate retention policy, which remains undecided.

See [Email](email.md), [GitHub](github.md), and [GitLab](gitlab.md) for login-method details.
