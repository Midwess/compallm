---
title: Email login
available: false
---

# Email login

Queued SMTP delivery and signup/signin confirmation with cookie or token delivery are implemented; automatic cookie authentication and renewal remain planned. The [login ADR](adr.md) defines the shared authentication decisions.

## Flow

1. The user enters a valid email address from any domain. There is no organization email-domain allowlist and no password.
2. The server enqueues a login code for SMTP delivery and returns a verification challenge after admission. Queue acceptance does not guarantee delivery. The challenge cannot authorize normal API requests.
3. The user submits the challenge and received code. The server verifies them before establishing a session. Signup derives the display name from the email; the user can edit it later.
4. Signup confirmation (`POST /api/auth/signup/email/confirmation/otp`) creates the user; signin confirmation (`POST /api/auth/signin/email/confirmation/otp`) requires an existing active user. Both accept `delivery: "cookie" | "token"`, defaulting to cookies. Code requests use the corresponding `/signup/email` or `/signin/email` endpoint. Only confirmation chooses credential transport; code requests return the encrypted challenge.
5. Cookie confirmation requires the browser Origin and sets both HttpOnly cookies, returning `{}`. Token mode returns both credentials in JSON without setting cookies. See [credential delivery](session.md#credential-delivery) for security settings and compatibility.

The current stateless challenge allows reuse until its 10-minute expiry. The older product plan's 15-minute single-use policy remains unresolved; mailing does not change it. See the [mailing ADR](../mailing/adr.md) for queue limits. Duplicate recipients are throttled while pending and for the configured cooldown after SMTP acceptance (30 seconds by default); the API returns 429 without issuing a replacement challenge.

## Input normalization and validation

Signup and signin requests use Validify during extraction. Email addresses are trimmed, ASCII-lowercased, and validated using the shared User email rule. Registration prepares the User through the same domain rules; repositories persist that canonical email unchanged. Confirmation trims surrounding code whitespace and requires exactly six ASCII digits. The challenge must be nonempty and at most 16,384 characters; its contents are not modified. Delivery remains an enum.

Malformed JSON or invalid field formats return 400 before mail or persistence is called. A well-formed but incorrect code, expired challenge, or invalid signature still returns 401 through Core authentication. Validation errors do not echo emails, codes, or token values. Request rules are also described in the generated API guide and TypeScript hover documentation.

## SMTP configuration

| Environment variable | Required | Development default | Purpose |
| --- | --- | --- | --- |
| `SMTP_HOST` | Yes | `127.0.0.1` | SMTP server hostname |
| `SMTP_PORT` | Yes | `1025` | SMTP server port; local MailDev listens on 1025 |
| `SMTP_USERNAME` | No | None | SMTP authentication username |
| `SMTP_PASSWORD` | No | None | SMTP authentication password |
| `SMTP_FROM_ADDRESS` | Yes | `noreply@compallm.local` | Sender address for login-code messages |
| `SMTP_TLS` | Yes | `none` | `none`, `starttls`, or `tls`; production defaults to `starttls` |
| `SMTP_ENABLED` | No | `true` | Enable the queued SMTP service; disabled by default in production and tests |

Queue and delivery limits are part of the same SMTP settings passed to MailingService:

| Environment variable | Default | Purpose |
| --- | --- | --- |
| `SMTP_CONCURRENCY` | `10` | Joined receive/send loops in one worker |
| `SMTP_QUEUE_CAPACITY` | `1024` | Waiting emails, excluding active sends |
| `SMTP_ADMISSION_TIMEOUT_SECONDS` | `30` | Maximum wait for queue capacity |
| `SMTP_SEND_TIMEOUT_SECONDS` | `60` | Whole SMTP attempt timeout |
| `SMTP_RECIPIENT_COOLDOWN_SECONDS` | `30` | Throttle after successful SMTP acceptance |
| `SMTP_SHUTDOWN_TIMEOUT_SECONDS` | `60` | Maximum shutdown drain time |

SMTP settings are passed directly to the queue and transport libraries without a separate numeric-validation layer. Use positive concurrency and queue capacity; cooldown may be zero to disable the post-send delay. Duplicate recipients are still suppressed while pending.

SMTP credentials are optional only when the server permits unauthenticated delivery. Read secrets from the environment and keep them out of logs and browser responses. Production needs an explicit host and sender before enabling mailing. Local messages appear in MailDev at `http://127.0.0.1:1080` after `docker compose up`.

## Expected outcomes

- A matching, unexpired code establishes a session only after verification succeeds.
- An expired or incorrect code does not establish a session.
- Under the planned single-use policy, a consumed code cannot establish another session; enforcement remains unresolved above.
- A valid email is eligible regardless of its domain.
- Admission failure returns no challenge. SMTP failure after admission does not establish a session or automatically retry; the user can request another code after the pending recipient reservation clears.

## API reference

- [Request a signup code](../api/auth/signup/email/post.md).
- [Confirm signup with the code](../api/auth/signup/email/confirmation/otp/post.md).
- [Request a signin code](../api/auth/signin/email/post.md).
- [Confirm signin with the code](../api/auth/signin/email/confirmation/otp/post.md).

See [GitHub login](github.md) and [GitLab login](gitlab.md) for the other login methods, and the [login ADR](adr.md) for shared decisions.
