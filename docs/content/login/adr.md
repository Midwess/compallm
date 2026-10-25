---
title: "Login: ADR"
available: false
---

# Login: ADR

Decision status: Accepted on 2026-10-26. Implementation status: Planned.

## Context and decision

Vite and Axum share one public origin. The browser authenticates through HttpOnly cookies containing a short-lived access JWT and a rotating refresh credential backed by a database session. This keeps reusable login credentials out of frontend JavaScript and lets the server revoke refresh sessions.

This is the agreed target design, not a description of completed implementation. Authentication is passwordless; the application stores no user password and never receives a provider's password. This record applies to browser sessions; authentication for external API clients is a separate concern.

## Login methods

| Method | Flow and configuration |
| --- | --- |
| Email code | [Email login](email.md) |
| GitHub OAuth | [GitHub login](github.md) |
| GitLab OAuth | [GitLab login](gitlab.md) |

All methods establish the same browser session after identity verification. OAuth requires a verified provider email and does not send an email code. Repeated login with an already-linked provider account signs in the same user without creating a duplicate. Linking different provider identities to an existing user remains a separate decision.

First-time onboarding creates a personal organization. Access to additional organizations follows their membership policies; proving ownership of an email alone does not grant access to another workspace.

## Login and authenticated requests

1. The user completes email-code verification or an enabled OAuth login. An email challenge only permits verification; it cannot authorize normal API requests.
2. For a new user, registration creates the user and their personal organization atomically. Signin selects an active organization where the user is an owner or administrator.
3. Axum establishes a refresh session and sets access and refresh cookies through `Set-Cookie`. Responses can include user and organization information; they do not expose these credentials in JSON or URLs.
4. The browser automatically includes the cookies on same-origin API requests. Axum validates the access token before providing authenticated user and organization context to the endpoint. The frontend obtains its current identity from the API.

The access JWT contains the user ID, organization ID, and membership role. A refresh token identifies the user and refresh credential without carrying organization claims. Refresh and email-challenge tokens must never be accepted as user access tokens.

## Refresh and organization selection

When the access token expires, the frontend requests a refresh and waits for the result before retrying requests that were rejected before execution. It does not repeatedly retry authentication failures.

The server validates the refresh credential against its active database session, checks the user's status and current organization membership, and replaces both cookies. An explicit organization selection must belong to the user and be active; when omitted, refresh selects the user's oldest active membership, breaking ties by organization ID. The new access token uses the current membership role.

Signin's owner/admin rule selects the initial workspace; users may subsequently switch to an organization where they are a member. The frontend supplies the current organization during routine refresh to preserve workspace selection; omitting it requests the fallback above.

Each successful refresh atomically replaces the stored refresh credential and moves the session's expiry forward by 30 days. A user must sign in again after 30 days without a successful refresh. Reissuing a signed JWT alone does not invalidate its predecessor; persistent refresh state makes rotation enforceable. Store a hash of the refresh credential, not its reusable value.

Reusing a consumed refresh credential is treated as a possible theft: revoke the associated refresh session and require signin. Concurrent browser requests and tabs must coordinate refresh so ordinary activity does not trigger reuse detection. The server's policy for simultaneous refreshes and lost responses must be settled before implementation.

## Cookie and request protections

| Cookie | Expiry | Production attributes |
| --- | --- | --- |
| `__Host-access` | Short-lived, within 5–15 minutes | `HttpOnly; Secure; SameSite=Lax; Path=/` |
| `__Host-refresh` | 30 days, renewed on successful refresh | `HttpOnly; Secure; SameSite=Lax; Path=/` |

Production requires HTTPS. Neither cookie sets a `Domain` attribute, as required by the `__Host-` prefix. Cookie expiry must match the server's credential expiry. Local HTTP development uses explicitly separate cookie names and settings; production protections must not be relaxed for development convenience.

Because cookies are sent automatically, protect state-changing browser requests, including login confirmation, refresh, and logout, against CSRF. Validate their `Origin` against the configured public origin and reject untrusted or unverifiable origins. Keep GET requests free of mutations. `SameSite=Lax` is an additional protection, not the sole CSRF control. OAuth callbacks use their own request-binding protections, including state validation.

HttpOnly prevents JavaScript from reading cookies, but injected scripts can still make authenticated requests. Safe rendering and a Content Security Policy remain necessary. Do not store access or refresh credentials in browser storage or logs, and prevent caching of credential-setting and private identity responses. JWT validation checks signature, permitted algorithm, validity, and intended token variant before trusting claims.

## Consequences for logout and revocation

Logout revokes the current refresh session and clears both cookies. Administrative revocation also prevents further refresh for the affected session. Separate browser sessions can be revoked independently.

A copied access JWT may remain usable until its short expiry because normal requests validate it without consulting refresh-session state. Immediate invalidation would require an additional server-side check; it is not promised by clearing cookies or revoking refresh credentials. Organization or role changes are reflected when a new access token is issued, with the same bounded delay for existing tokens.

## Responsibilities and remaining choices

| Component | Responsibility |
| --- | --- |
| Axum application | Cookie transport, CSRF checks, authenticated request context, and private response handling |
| Core | Login and refresh rules, token validation, organization selection, and refresh-session lifecycle |
| PostgreSQL | Refresh-session persistence and atomic credential rotation |
| Vite frontend | Identity display, coordinated refresh, and signin navigation without reading credentials |

Before implementation, settle the exact access-token lifetime, concurrent-refresh and lost-response handling, and whether any operation requires immediate revocation. Database schema and endpoint contracts will follow those decisions. The [email-code page](email.md) records the outstanding expiry and reuse discrepancy. Cross-provider account linking and which login methods ship first remain open.

See [Development](../development/adr.md) for application boundaries and [Organization and membership](../organization/adr.md) for workspace rules.
