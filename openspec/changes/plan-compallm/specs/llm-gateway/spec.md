# LLM Gateway

## ADDED Requirements

### Requirement: Governed model access
The Gateway SHALL authenticate Compallm API keys, enforce configured access policies, and attribute usage without exposing upstream credentials.

#### Scenario: Unauthorized model
- **WHEN** a caller requests a model excluded by its access policy
- **THEN** the Gateway rejects the request before contacting a provider
