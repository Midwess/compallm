---
title: Data and privacy
available: false
---

# Data and privacy

## Status

Decision status: Proposed. Implementation status: Under development.

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Open questions are recorded below.

## Context and proposed scope

- Keep provider credentials and plugin secrets out of browser responses and exported content.
- Define access, retention, export, and deletion behavior for chats, files, notebooks, and activity records.
- Make it clear which providers and plugins receive information for each interaction.
- Allow organizations to control whether request content is stored in gateway logs.
- Apply the same resource permissions when content is shared, retrieved for context, or accessed by a plugin.

## Consequences

Exports, provider calls, plugin access, and source retrieval must respect the same resource permissions and secret-handling rules.

## Behavior examples

### Credential confidentiality
The system should exclude upstream provider credentials and plugin secrets from browser responses and exported content.

#### Example: Export content
- **WHEN** a member exports a conversation that used a provider or plugin
- **THEN** the export contains no upstream credentials or plugin secrets

## Open questions

- Are chats and notebooks private by default, and who may share or inspect them?
- What are the retention and deletion rules, including files sent to external providers?
- Which privacy controls are required for the first release?

## Related pages

- [Organization and membership](../organization/adr.md)
- [Product overview](../overview.md)
