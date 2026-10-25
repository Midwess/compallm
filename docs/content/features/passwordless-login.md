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
- The application sends a one-time code to that address.
- A code expires after 15 minutes and can only be used once.
- Successful verification establishes the user's authenticated session.

The proposed supporting controls are resend limits, verification-attempt limits, session expiration, logout, and administrative session revocation. Authentication proves control of the email address; whether that grants workspace access or creates membership remains an open business decision.

## Behavior examples

### Organization email verification
The system should authenticate eligible organization emails through emailed single-use codes without passwords; the allowed domain should be configured through an environment variable.

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

#### Example: Domain mismatch
- **WHEN** an email domain does not exactly match the configured allowed domain, ignoring case
- **THEN** the system rejects the login request without sending a code

## Related pages

- [Organization and membership](organization-membership.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
