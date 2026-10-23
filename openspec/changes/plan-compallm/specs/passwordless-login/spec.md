# Passwordless email login

## ADDED Requirements

### Requirement: Organization email verification
The system SHALL authenticate eligible organization emails through emailed single-use codes without passwords; the allowed domain SHALL be configured through an environment variable.

#### Scenario: Expired code
- **WHEN** a code is submitted at or after 15 minutes from issuance
- **THEN** the system rejects the code without establishing a new session

#### Scenario: Unexpired code
- **WHEN** a user submits a matching unused code before expiration
- **THEN** the system consumes the code and establishes an authenticated session
- **AND** workspace access remains subject to the membership policy to be decided

#### Scenario: Consumed code
- **WHEN** a user resubmits a previously consumed code
- **THEN** the system rejects it without establishing another session

#### Scenario: Domain mismatch
- **WHEN** an email domain does not exactly match the configured allowed domain, ignoring case
- **THEN** the system rejects the login request without sending a code
