# Data and privacy

## ADDED Requirements

### Requirement: Credential confidentiality
The system SHALL exclude upstream provider credentials and plugin secrets from browser responses and exported content.

#### Scenario: Export content
- **WHEN** a member exports a conversation that used a provider or plugin
- **THEN** the export contains no upstream credentials or plugin secrets
