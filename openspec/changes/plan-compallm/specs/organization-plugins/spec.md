# Organization plugins

## ADDED Requirements

### Requirement: Controlled plugin access
The system SHALL restrict plugin invocation according to organization configuration, member access, and plugin permissions.

#### Scenario: Disabled plugin
- **WHEN** a conversation attempts to invoke a plugin disabled by its organization
- **THEN** the system rejects the invocation without executing the external tool
