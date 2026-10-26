---
title: GitHub login
available: false
---

# GitHub login

Planned behavior. This page owns the GitHub OAuth flow; the [login ADR](adr.md) defines shared browser sessions, cookies, and security boundaries.

## Flow

1. The user selects GitHub login. The application redirects them to GitHub's authorization endpoint with a request-bound state value.
2. GitHub handles authentication and consent, then redirects to the application with an authorization code.
3. The server validates the callback and exchanges the code with GitHub. It obtains the provider account identity and a verified email address without requesting the user's password.
4. An already-linked GitHub account signs in the same user. First-time onboarding follows the shared registration rules; automatic linking to another existing identity is not decided here.
5. The server establishes the browser session described in the [login ADR](adr.md). No email code is sent, and provider credentials are not returned to the browser.

## Planned configuration

| Environment variable | Required when enabled | Default | Purpose |
| --- | --- | --- | --- |
| `GITHUB_CLIENT_ID` | Yes | None | GitHub OAuth application identifier |
| `GITHUB_CLIENT_SECRET` | Yes | None | Server-side OAuth token exchange |
| `GITHUB_API_URL` | No | `https://api.github.com` | API base URL; override for GitHub Enterprise Server |

Read credentials from the environment; do not expose them in logs or browser responses. These configuration names and defaults describe planned integration.

## Expected outcomes

- Valid authorization and a verified provider email establish a session.
- Denied consent, an invalid callback, or an unverified email does not establish a session.
- Failed token exchange or provider errors report login failure without exposing provider details.
- Repeated login with the same linked GitHub account does not create a duplicate user.

See [Email login](email.md), [GitLab login](gitlab.md), and the [login ADR](adr.md).
