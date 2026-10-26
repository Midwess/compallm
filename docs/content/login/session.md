---
title: Browser sessions
available: false
---

# Browser sessions

This page expands the accepted [login ADR](adr.md). Cookie delivery and persistent refresh sessions remain planned; the current signup API returns credentials in JSON.

## Credential transport

Axum sets access and refresh cookies after authentication. Responses may include identity and organization information but must not expose reusable credentials in JSON or URLs once the cookie design is implemented. The frontend obtains its current identity from the API; it never reads the cookies.

| Cookie | Expiry | Production attributes |
| --- | --- | --- |
| `__Host-access` | 5–15 minutes; exact lifetime undecided | `HttpOnly; Secure; SameSite=Lax; Path=/` |
| `__Host-refresh` | 30 days, renewed after successful refresh | `HttpOnly; Secure; SameSite=Lax; Path=/` |

Production uses HTTPS, no `Domain` attribute, and matching cookie and credential expiries. Local HTTP development uses separate cookie names and settings without weakening production protections.

An access JWT carries the user ID, organization ID, and membership role. A refresh JWT carries `user_id`, `session_id`, the rotating `token_id`, and validity timestamps without organization claims. The session ID stays unchanged during refresh. Email challenges and refresh tokens cannot authorize normal API operations.

## Session record replaces login recording

Use one `user_sessions` row for each successful signup or signin. A row represents a login session, not a verified physical device. A second login creates another session; refreshing an existing session does not create another row.

| Field | Meaning |
| --- | --- |
| `id` | Stable session ID, using the project's single-instance Snowflake generator; Core uses `u64`, stored within PostgreSQL BIGINT's positive range |
| `user_id` | Owning user, with a foreign key to `users.id` |
| `refresh_token_id` | Current random credential identifier matching the signed JWT's `token_id`; changes on every successful refresh |
| `created_at` | Time this login succeeded; never advanced by refresh |
| `last_used_at` | Time of login or most recent successful refresh |
| `expires_at` | Sliding refresh-session deadline, initially 30 days after login |
| `revoked_at` | Nullable timestamp; a value means the session cannot refresh |

The primary key supports refresh and revocation by session ID. Index `(user_id, created_at)` to list a user's logins and find their latest successful login. Do not store the reusable refresh JWT. Its signature must be verified before treating a matching identifier as authorized.

Device information is not required. A user-agent description or user-supplied device label may be added later to help users recognize sessions. Browser metadata is untrusted and must not establish identity; no device fingerprint is required.

Replace `UserRepository::record_login` with `SessionRepository::create` and session lifecycle operations. Core owns the session entity and repository contract; PostgreSQL owns its schema, mapper, and repository implementation. Operations use the caller's database context.

For signin, validate identity, user status, and initial membership before inserting the session. For signup, create the user, personal organization, membership, and first session within one transaction. Generate credentials for that session and return them only after commit; any failure rolls back the session together with the account changes.

Once readers use session records, retire `users.last_login_at` rather than maintaining a second login record. A latest-login query uses `created_at`, including retained revoked sessions; `last_used_at` measures refresh activity instead. Session-history retention and the transition for existing timestamps and stateless refresh tokens must be defined before migration. Do not fabricate historical sessions from the old timestamp.

## Refresh and organization selection

When an access token expires, the frontend coordinates one refresh before retrying requests rejected before execution. Repeated authentication failures must not cause a retry loop. Cross-tab concurrency and lost-response handling need an explicit policy before implementation.

Refresh first validates the JWT signature, variant, and expiry, then loads its `session_id`. Require the stored `user_id` to match the claims, no revocation, an unexpired session deadline, and a matching `refresh_token_id`. Also check the user's status and current membership. An explicit organization must be active and accessible to the user. With no selection, choose the oldest active membership, breaking ties by organization ID. The frontend supplies the current organization during routine refresh to preserve workspace selection.

Signin initially selects an owner/admin organization. Later refreshes may switch to an organization where the user is a member. The new access token carries the current role.

Atomically replace `refresh_token_id`, advance `last_used_at`, and extend `expires_at` to 30 days from successful refresh. The update must match the previous credential identifier and an active session so concurrent requests cannot both rotate it, and logout cannot be undone by a concurrent refresh. Reissuing a JWT alone does not invalidate the predecessor. Treat a validly signed, unexpired credential that no longer matches its session as possible reuse: revoke that session and require signin, subject to the concurrency policy still to be settled.

## Request protections

Validate state-changing browser requests against the configured public `Origin`, including login confirmation, refresh, and logout. Reject untrusted or unverifiable origins. Keep GET free of mutations. `SameSite=Lax` supplements CSRF protection; OAuth callbacks use their own request binding, including state validation.

HttpOnly prevents script access to cookie values, but injected scripts can still send authenticated requests. Safe rendering and a Content Security Policy remain necessary. Keep credentials out of browser storage and logs; prevent caching of credential-setting and private identity responses.

Validate JWT signatures, permitted algorithms, validity, and intended token variants before trusting claims.

## Logout and revocation

Logout revokes the current refresh session and clears both cookies. Administrators can revoke refresh sessions independently. Expired, revoked, or reused refresh credentials require signin; a disabled user or inaccessible organization cannot receive replacement credentials.

Existing access JWTs remain usable until expiry because normal requests do not consult refresh-session state. Role changes have the same bounded delay. Immediate invalidation would require an additional server-side check and is not promised by clearing cookies.

See [Email](email.md), [GitHub](github.md), and [GitLab](gitlab.md) for method-specific flows.
