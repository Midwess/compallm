---
title: Passwordless login
tags:
  - features
---

# Passwordless login

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

The agreed login direction is:

- A user enters an organization email address; there is no password.
- The allowed organization email domain is configured through an environment variable.
- The application emails a verification code and returns a signed access token containing an encrypted email-OTP payload.
- The token and its matching code expire after 10 minutes. Requesting another code does not invalidate previous pairs, and a matching pair may be reused until it expires.
- Successful verification establishes the user's authenticated session.

The proposed supporting controls are resend limits, verification-attempt limits, session expiration, logout, and administrative session revocation. Authentication proves control of the email address; whether that grants workspace access or creates membership remains an open business decision.

## Behavior examples

### Organization email verification
The system should authenticate eligible organization emails through emailed verification codes without passwords; startup supplies the configured allowed domain. OTP payload encryption and JWT signing share one configured secret passed through constructors; the cipher key is derived from that secret. Token modules do not read environment variables.

#### Example: Expired code
- **WHEN** a token and code are submitted at or after 10 minutes from issuance
- **THEN** the system rejects the code without establishing a new session

#### Example: Unexpired code
- **WHEN** a user submits an authentic email access token and its matching code before expiration
- **THEN** the system verifies email control so the login workflow can establish an authenticated session
- **AND** workspace access remains subject to the membership policy to be decided

#### Example: Previous code remains valid
- **WHEN** a user requests another code and later submits an older authentic token with its original matching code before expiration
- **THEN** the system accepts that pair
- **AND** successfully verifying a pair does not prevent its reuse before expiration

#### Example: Domain mismatch
- **WHEN** an email domain does not exactly match the configured allowed domain, ignoring case
- **THEN** the system rejects the login request without sending a code

## Related pages

- [Organization and membership](organization-membership.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
