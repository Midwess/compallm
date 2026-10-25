---
title: Authentication
available: false
---

# Authentication

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

Authentication is passwordless. The user proves control of an identity without providing a password to the application. Three login methods are supported:

| Method | Provider | Credential sent by the user |
| --- | --- | --- |
| Email code | SMTP mail server | One-time code emailed to the address |
| GitHub OAuth | GitHub | OAuth authorization code from GitHub's consent screen |
| GitLab OAuth | GitLab | OAuth authorization code from GitLab's consent screen |

Shared behavior across methods:

- A successful authentication establishes the user's authenticated session.
- Sessions expire, can be ended by logout, and can be revoked administratively.
- The application stores no user password and never receives the provider's password.

Method-specific behavior:

- **Email code**: the user enters a valid email address from any domain; there is no password. The application sends a one-time code to that address through the SMTP server. A code expires after 15 minutes and can only be used once. Supporting controls are resend limits and verification-attempt limits.
- **GitHub OAuth**: the application redirects the user to GitHub's authorization endpoint and handles the redirect callback with an authorization code. The client ID and secret are configured through environment variables.
- **GitLab OAuth**: the application redirects the user to GitLab's authorization endpoint and handles the redirect callback with an authorization code. The client ID and secret are configured through environment variables.

The OAuth providers supply the user's verified email address, so an OAuth login does not send an email code. There is no organization email-domain allowlist.

Authentication proves control of the identity only. Whether that grants workspace access or creates membership remains an open business decision.

## Configuration

### Email code

| Environment variable | Required | Development default | Purpose |
| --- | --- | --- | --- |
| `SMTP_HOST` | Yes | `127.0.0.1` | Hostname of the SMTP server |
| `SMTP_PORT` | Yes | `587` | Port used to connect to the SMTP server |
| `SMTP_USERNAME` | No | None | Username for SMTP authentication |
| `SMTP_PASSWORD` | No | None | Password for SMTP authentication |
| `SMTP_FROM_ADDRESS` | Yes | None | Sender address used for login-code messages |
| `SMTP_TLS` | No | `false` | Whether the SMTP connection requires TLS; production should enable the mode required by its mail server |

SMTP credentials are optional only when the configured server permits unauthenticated delivery.

### GitHub OAuth

| Environment variable | Required | Default | Purpose |
| --- | --- | --- | --- |
| `GITHUB_CLIENT_ID` | Yes | None | Identifies the GitHub OAuth application |
| `GITHUB_CLIENT_SECRET` | Yes | None | Authenticates the application during the OAuth token exchange |
| `GITHUB_API_URL` | No | `https://api.github.com` | Base URL for GitHub API requests; override it for GitHub Enterprise Server |

### GitLab OAuth

| Environment variable | Required | Default | Purpose |
| --- | --- | --- | --- |
| `GITLAB_CLIENT_ID` | Yes | None | Identifies the GitLab OAuth application |
| `GITLAB_CLIENT_SECRET` | Yes | None | Authenticates the application during the OAuth token exchange |
| `GITLAB_API_URL` | No | `https://gitlab.com/api/v4` | Base URL for GitLab API requests; override it for a self-managed GitLab instance |

The client ID and secret are required only when the corresponding login method is enabled. The API URLs above are planned configuration names and defaults; they are not implemented yet.

Credentials are read from the environment, are never exposed in logs or browser responses, and are absent from development defaults.

## Behavior examples

### Email code

The system should authenticate valid email addresses from any domain through emailed single-use codes without passwords.

#### Example: Expired code
- **WHEN** a code is submitted at or after 15 minutes from issuance
- **THEN** the system rejects the code without establishing a new session

#### Example: Unexpired code
- **WHEN** a user submits a matching unused code before expiration
- **THEN** the system consumes the code and establishes an authenticated session
- **AND** workspace access remains subject to the membership policy to be decided

#### Example: Consumed code
- **WHEN** a user resubmits a previously consumed code
- **THEN** the system rejects it without establishing another session

#### Example: Any email domain
- **WHEN** a user submits a valid email address
- **THEN** the system can send a verification code regardless of its domain

#### Example: Delivery failure
- **WHEN** the SMTP server rejects or fails to accept the message
- **THEN** the system reports that the code could not be sent
- **AND** does not establish a session

### GitHub OAuth

The system should authenticate a user through GitHub's authorization code flow without requesting a password.

#### Example: Authorization granted
- **WHEN** the user completes GitHub's consent screen and the callback carries a valid authorization code
- **THEN** the system exchanges the code for the user's identity
- **AND** establishes an authenticated session

#### Example: Authorization denied
- **WHEN** the user declines GitHub's consent screen or the callback reports an error
- **THEN** the system reports the failed authorization
- **AND** does not establish a session

#### Example: Provider error
- **WHEN** the token exchange fails or GitHub returns a non-success response
- **THEN** the system reports the failure without exposing provider details to the browser
- **AND** does not establish a session

### GitLab OAuth

The system should authenticate a user through GitLab's authorization code flow without requesting a password.

#### Example: Authorization granted
- **WHEN** the user completes GitLab's consent screen and the callback carries a valid authorization code
- **THEN** the system exchanges the code for the user's identity
- **AND** establishes an authenticated session

#### Example: Authorization denied
- **WHEN** the user declines GitLab's consent screen or the callback reports an error
- **THEN** the system reports the failed authorization
- **AND** does not establish a session

#### Example: Provider error
- **WHEN** the token exchange fails or GitLab returns a non-success response
- **THEN** the system reports the failure without exposing provider details to the browser
- **AND** does not establish a session

### Identity rules

#### Example: Unverified provider email
- **WHEN** the provider reports an email address that is not verified
- **THEN** the system rejects the login without establishing a session

#### Example: Repeated login with the same provider account
- **WHEN** a user signs in again with a provider account already linked to them
- **THEN** the system establishes a new session for the same user
- **AND** does not create a second user

#### Example: Session ended
- **WHEN** the user logs out or an administrator revokes the session
- **THEN** the system rejects further use of that session

## Related pages

- [Organization and membership](organization/organization-membership.md)
- [Data and privacy](data-privacy.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
