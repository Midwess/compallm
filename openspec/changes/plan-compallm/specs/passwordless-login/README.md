# Passwordless email login

## Planning status

Draft scope for discussion, not a claim of implementation. Detailed policies and first-release scope require review before implementation.

## Feature scope

The agreed login direction is:

- A user enters an organization email address; there is no password.
- The allowed organization email domain is configured through an environment variable.
- The application sends a one-time code to that address.
- A code expires after 15 minutes and can only be used once.
- Successful verification establishes the user's authenticated session.

The proposed supporting controls are resend limits, verification-attempt limits, session expiration, logout, and administrative session revocation. Authentication proves control of the email address; whether that grants workspace access or creates membership remains an open business decision.

## Acceptance requirements

See [spec.md](spec.md) for proposed requirements and scenarios. This feature overview remains part of the proposal; preserve enduring scope in baseline documentation when the feature is implemented and archived.
