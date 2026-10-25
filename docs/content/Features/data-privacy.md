---
title: Data and privacy
available: false
---

# Data and privacy

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

- Keep provider credentials and plugin secrets out of browser responses and exported content.
- Define access, retention, export, and deletion behavior for chats, files, notebooks, and activity records.
- Make it clear which providers and plugins receive information for each interaction.
- Allow organizations to control whether request content is stored in gateway logs.
- Apply the same resource permissions when content is shared, retrieved for context, or accessed by a plugin.

## Behavior examples

### Credential confidentiality
The system should exclude upstream provider credentials and plugin secrets from browser responses and exported content.

#### Example: Export content
- **WHEN** a member exports a conversation that used a provider or plugin
- **THEN** the export contains no upstream credentials or plugin secrets

## Related pages

- [Organization and membership](organization/organization-membership.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
