---
title: Provider management
available: true
---

# Provider management

## Status

Decision status: Proposed. Implementation status: Under development.

Provider connections belong to one organization. The organization service lists
and removes its connections and provides explicit operations for adding OpenAI,
Anthropic, and OpenAI-compatible APIs. It does not expose generic create or
update operations. Provider API calls and subscription integration belong in
separate integration services rather than one general provider manager.

## Consequences

Provider persistence and services require organization scope. Each add operation
selects its protocol internally, so callers cannot persist a mismatched protocol.
Credentials stay outside ordinary provider read models and must not appear in
API responses or logs.

## Open questions

Credential encryption, rotation, and provider-specific integration behavior
remain unresolved.

## Related pages

- [Implementation](implementation.md)
- [Organization and membership](../organization/adr.md)
