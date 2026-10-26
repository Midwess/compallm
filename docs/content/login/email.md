---
title: Email login
available: false
---

# Email login

Planned behavior. This page owns the email-code flow; the [login ADR](adr.md) defines shared browser sessions, cookies, and security boundaries.

## Flow

1. The user enters a valid email address from any domain. There is no organization email-domain allowlist and no password.
2. The server requests delivery of a login code through the configured SMTP server and returns a verification challenge. The challenge cannot authorize normal API requests.
3. The user submits the challenge and received code. The server verifies them before establishing a session. Signup derives the display name from the email; the user can edit it later.
4. Successful confirmation follows the registration or signin flow and cookie delivery described in the [login ADR](adr.md).

The existing product plan specifies a single-use code expiring after 15 minutes, with resend and verification-attempt limits. The current stateless challenge design allows reuse until expiry and uses a 10-minute lifetime. Resolve this discrepancy before implementing delivery or code-consumption behavior; this documentation refactor does not change either rule.

## Planned configuration

| Environment variable | Required | Development default | Purpose |
| --- | --- | --- | --- |
| `SMTP_HOST` | Yes | `127.0.0.1` | SMTP server hostname |
| `SMTP_PORT` | Yes | `587` | SMTP server port |
| `SMTP_USERNAME` | No | None | SMTP authentication username |
| `SMTP_PASSWORD` | No | None | SMTP authentication password |
| `SMTP_FROM_ADDRESS` | Yes | None | Sender address for login-code messages |
| `SMTP_TLS` | No | `false` | Production enables the TLS mode required by its mail server |

SMTP credentials are optional only when the server permits unauthenticated delivery. Read secrets from the environment and keep them out of logs and browser responses. These settings describe planned integration, not implemented email delivery.

## Expected outcomes

- A matching, unexpired code establishes a session only after verification succeeds.
- An expired or incorrect code does not establish a session.
- Under the planned single-use policy, a consumed code cannot establish another session; enforcement remains unresolved above.
- A valid email is eligible regardless of its domain.
- Delivery failure reports that the code could not be sent and does not establish a session.

## API reference

- [Request a signup code](../api/auth/signup/email/post.md).
- [Confirm signup with the code](../api/auth/signup/email/confirmation/otp/post.md).

See [GitHub login](github.md) and [GitLab login](gitlab.md) for the other login methods, and the [login ADR](adr.md) for shared decisions.
