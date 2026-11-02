---
title: Browser sessions
available: false
---

# Browser sessions

A session records one successful signup or signin. Core authentication persists sessions, renews JWTs without database writes, and changes organization through a separate operation. Signup/signin confirmation supports cookie and JSON token delivery. Protected APIs accept access cookies or Bearer access tokens, with automatic cookie renewal. Signout is implemented; OAuth remains planned. The [login ADR](adr.md) explains the decisions behind this flow.

## What a session represents

A second login creates another session; refreshing credentials keeps the same session. Each record identifies its organization, owner, and revocation state. Expiry comes from the signed JWT; normal refresh does not write an activity timestamp or expiry to the database. No creation or update timestamp is stored, so sessions do not provide login-time history.

Device information is optional. A browser description or user-provided label may eventually help recognize a session, but neither proves which physical device or person is using it. Authentication does not require device fingerprinting.

Both token types embed the same Session struct. Session owns its ID, organization ID and name, user ID, and revocation state; these fields are not repeated at the token level. Access JWTs additionally carry the current membership role. A signed snapshot does not replace the database record: refresh and organization switching load the current session to enforce revocation and organization changes. The complete JWT is not stored. Every lookup, renewal, and revocation uses the organization and user together, supported by a compound organization/user index.

The PostgreSQL session lookup joins the organization to hydrate its current name without storing a duplicate name column in `user_sessions`. Membership queries return the organization with its member record for login, renewal, and switching. Organization names are display metadata: they do not participate in session identity matching or conditional update guards. Tokens without the required name field require signin again.

Repository lookups accept the token's Session instance directly and match its ID, organization, and owner. Validation compares those identity fields against the current record and rejects revoked sessions. No separate session-reference type is needed. The session schema and JWT payload omit generic creation/update timestamps and separate refresh credential identifiers.

## Read-only renewal

Normal refresh validates the signed JWT, loads its organization/user-scoped session, rejects revocation or an organization mismatch, and checks the active user and current membership. It issues new JWTs for the same session and organization, with a new 30-day refresh expiry.

There is no database update, credential consumption, automatic revocation, or transaction started by renewal. It can run inside an existing transaction. Expired JWTs are already invalid and do not require revoking the session.

Older refresh JWTs remain usable until their own expiry. Each valid renewal starts another 30-day JWT lifetime, so continued use can extend beyond 30 days from the original login without a fixed database deadline. A holder, including someone with a copied JWT, can keep renewing before expiry until explicit revocation or a session change.

## Protected API requests

Public signup/signin code-request routes require no credentials. Their confirmation routes require an email-OTP access token from cookie or Bearer. Middleware verifies the selected credential and inserts its concrete type as a request extension: `UserAccessToken`, `RefreshToken`, or decrypted `EmailOtpAccessToken`. Handlers declare `RequireToken<T>` to require exactly that type; an absent type returns 401. Raw request access uses `extensions().get::<T>()`. User-token extraction exposes organization/role guards returning 403. A route on the authenticated router must explicitly require the token type appropriate for its operation. `GET /api/auth/me` returns session ID, user ID, organization ID, `organizationName`, and the cached role without returning credentials. It returns the organization name snapshot embedded in the verified access token and performs no database query when that token is valid. Organization renames and membership changes become visible on renewal or a new login; organization switching issues the target organization name.

```rust
async fn handler(RequireToken(token): RequireToken<UserAccessToken>) {
    // A verified user access token is present; refresh/OTP tokens cannot satisfy this.
}
```

The concrete token is also available through `request.extensions().get::<UserAccessToken>()`. Middleware replaces token extensions on each request so the selected Bearer/cookie identity cannot inherit another credential type from an earlier layer.

An explicit Authorization header takes precedence over cookies. Require one signed, unexpired Bearer token; malformed, expired, or duplicate credentials return 401 without cookie fallback or renewal. Only its actual verified type is inserted. A refresh or OTP token cannot satisfy `RequireToken<UserAccessToken>`. Refresh extraction verifies the signed claims, not current database state; renewal and switching still use Core's session checks. OTP extraction verifies the encrypted challenge, not email ownership; confirmation must still match the emailed code. Confirmation handlers require `RequireToken<EmailOtpAccessToken>` and pass the verified claims to Core; JSON contains only the code and delivery selection. Token values in the JSON body are rejected.

Cookie access is validated without querying the database, preserving the documented short-lived cached-role tradeoff. If access is missing or expired, validate the refresh cookie through Core, use the new access claims for the current request, and set both replacement cookies on the response. The access cookie may contain a user or email-OTP access token; middleware inserts the actual verified type. An OTP access cookie needs no refresh token, never grants user access, and is not renewed. Invalid signatures, refresh tokens in the access-cookie slot, or ambiguous access cookies are rejected. With valid cookie access, a valid refresh cookie is also inserted as `RefreshToken` only when it matches that access token's session. Invalid or unrelated refresh cookies do not affect valid access, but cannot satisfy refresh extraction. After automatic renewal, both new token types are inserted. Bearer mode never adds cookie credentials. Refresh performs the existing database session/user/membership checks without a write. Any middleware authentication failure in cookie mode clears both authentication cookies, including Origin rejection or temporary storage failure. Bearer-mode errors leave browser cookies untouched. Failed renewal does not revoke sessions.

Cookie-authenticated mutations require an allowed Origin. Automatic renewal also requires an allowed Origin, or, for safe GET/HEAD requests without Origin, the browser's `Sec-Fetch-Site: same-origin`. This supports ordinary same-origin GET fetches without allowing cross-site requests to renew cookies. Bearer-only authentication does not use ambient cookies and does not require browser Origin validation. Protected responses use `Cache-Control: no-store`; 401 responses advertise Bearer authentication. Frontend assets and public routes never trigger automatic renewal.

## Organization switching

Switching is a separate operation. Verify the current refresh credential and target membership, change the session organization, save conditionally, and return both replacement tokens after commit. Signin starts in an owner/admin organization; switching may select a member organization. Refresh tokens for a different organization are rejected; already-issued access JWTs retain the short-expiry limitation described below.

Old-organization credentials no longer resolve the moved session. Switching to the same organization preserves existing tokens. Switching A → B → A allows unexpired tokens for A again. Conditional saves reject a switch when the stored organization or revocation state no longer matches the loaded state; a losing request is rejected without revoking the winner.

Ordinary renewal never changes organizations. It uses the current role when issuing an access JWT.

## Logout and revocation

POST `/api/auth/signout` requires a user access token, revokes its organization/user-scoped session, and clears both authentication cookies for cookie requests. Repeating signout with a still-valid access token for an already revoked session succeeds. Cookie requests require a trusted Origin. Bearer requests revoke only the selected session and leave browser cookies unchanged. Other login sessions remain independent. Persistence failures return an error rather than claiming success.

A copied access JWT can remain usable until its short expiry, including its cached role. Immediate access-token invalidation requires an additional server-side check. Failed renewal or switching does not automatically revoke a session, and invalid signatures or expired JWTs never authorize changes.

## Credential delivery

Signup/signin code requests and confirmation each accept optional `delivery` in JSON, defaulting to `cookie`. Confirmation requires the OTP access cookie or Bearer challenge. Code requests deliver the OTP access token only after email admission succeeds. Confirmation delivers user credentials only after Core authentication succeeds and the session commits. The mode is never stored in Session or JWT claims.

- `cookie`: return lifetime metadata with no token values. A code request sets a ten-minute OTP access cookie and clears the refresh cookie; confirmation replaces it with a user access cookie and sets the refresh cookie. Starting a new cookie login replaces any current browser credentials but does not revoke the previous database session.
- `token`: code requests return `{ "token": "…" }`; confirmation returns `{ "accessToken": "…", "refreshToken": "…" }`. Neither sets cookies. This does not clear an existing browser session.

Both require JSON input. Cookie delivery at either step requires an allowed Origin; token mode does not rely on browser cookies. Protected API middleware reads these cookies and renews them automatically when needed.

| Cookie | Expiry | Production attributes |
| --- | --- | --- |
| `__Host-access` | 30 minutes (configured access lifetime) | `HttpOnly; Secure; SameSite=Lax; Path=/` |
| `__Host-refresh` | 30 days, renewed after successful refresh | `HttpOnly; Secure; SameSite=Lax; Path=/` |

Production requires HTTPS, no `Domain` attribute, and matching cookie and credential expiries. `AUTH_COOKIE_SECURE` defaults to true in production and false in development/tests. Nonsecure local cookies use `compallm-access` and `compallm-refresh` names. `AUTH_PUBLIC_ORIGIN` sets the exact browser origin, including scheme and port, and is required for cookie confirmation in production. No forwarded headers determine cookie security or the allowed origin.

With an empty configured origin in nonsecure local development, only a loopback HTTP Origin matching the request Host is accepted, supporting Hero's selected port. Missing, opaque, or untrusted origins receive 403 before authentication or persistence.

State-changing browser requests, including login confirmation, refresh, and logout, must pass Origin validation. Keep GET free of mutations. `SameSite=Lax` supplements CSRF protection; OAuth callbacks require request binding, including state validation.

HttpOnly blocks cookie reads, but injected scripts can still make requests. Safe rendering and a Content Security Policy remain necessary. Keep credentials out of browser storage and logs, and prevent caching of private identity or credential-setting responses.

## Session retention

Refresh requires a signed token with an embedded session. Renewal records no activity timestamp, so cleanup requires a separate retention policy that remains unresolved.

See [Email](email.md), [GitHub](github.md), and [GitLab](gitlab.md) for login-method details.


## Token lifetime responses

Every explicit token-issuing response includes timing metadata for either
delivery mode. OTP requests return `lifetime`; confirmation returns
`accessLifetime` and `refreshLifetime`. Each contains `issuedAt` and `expiresAt`
(Unix seconds) and `expiresInSeconds` (remaining whole seconds when the response
is prepared), plus `throttleSeconds` (minimum client wait before requesting
another token). OTP responses set `throttleSeconds` to 30; access and refresh
token metadata defaults to 0. Expiry is unchanged by this throttle. Values come from the issued token's validity, not a second client
constant. Cookie delivery omits credentials only, retaining the metadata.
Automatic cookie renewal on an unrelated protected endpoint preserves that
endpoint's response contract; its browser cookies retain their expiry settings.

The portal uses `lifetime.throttleSeconds` for its resend countdown, independently
of OTP expiry.
It computes remaining time from a deadline rather than decrementing a counter,
so background-tab throttling does not extend the wait. Resend is disabled while
throttle time remains; the SMTP recipient throttle is an independent server rule.
Pending SMTP delivery or configured server cooldowns can still reject a resend;
metadata does not guarantee admission. A new
successful challenge replaces the deadline. Failed sends preserve it. Leaving
authentication or completing verification disposes the timer. Credentials and
OTP validity state remain in memory and never enter the URL.
