---
title: Session persistence implementation
available: false
---

# Session persistence implementation

This first implementation replaces user-level login recording with `user_sessions`, a dedicated Core session entity and repository, and a PostgreSQL adapter. Signup and signin create sessions transactionally; refresh JWTs carry a stable session ID and a rotating random token identifier. Full JWTs and device fingerprints are not stored.

Successful refresh locks the owned session row, validates its active lifetime and current credential, checks the user and selected membership, and atomically changes the token identifier, activity time, and 30-day deadline. Concurrent attempts serialize on that row. A valid unexpired old credential revokes the entire session and requires signin, including after a lost response or competing browser refreshes; there is no grace or automatic replay. Invalid signatures and expired JWTs never authorize database changes. Revocation must commit even when authentication is denied.

Refresh owns its transaction and rejects invocation inside an existing transaction: a caller rolling back an authentication error must not undo a committed revocation. Signup and signin still join enclosing transactions as usual. Application entry points provide the ordinary scoped connection context.

Only the database session and credential change on refresh; the original login timestamp and session ID remain stable. The session repository also supports idempotent revocation. Cookie transport, OAuth, mail delivery, and new browser endpoints are separate changes; the existing signup response remains JSON.

Add a forward migration and checked-in SeaORM schema without applying the migration. Keep `users.last_login_at` as a deprecated compatibility column, stop writing it, and do not fabricate historical sessions. Removing it and defining history retention remain later migration decisions. Existing refresh tokens without a session ID require signin after deployment; coordinate this cutover with clients.

The implementation branch starts at `main`, independently of the documentation PR. The complete login/session design is recorded in [PR #14](https://github.com/Midwess/compallm/pull/14). This page records the first implementation scope and the strict credential-reuse decision.
