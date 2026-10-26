---
title: Provider management
available: true
---

# Provider management

## Status

Decision status: Proposed. Implementation status: Under development.

Provider connections belong to one organization. Management workflows create,
list, update, and delete only connections in that organization. Provider API
integration behavior belongs in separate integration services rather than one
general provider manager.

## Consequences

Provider persistence and services require organization scope. Credentials stay
outside ordinary provider read models and must not appear in API responses or
logs.

## Open questions

Credential encryption, rotation, and provider-specific integration behavior
remain unresolved.

## Related pages

- [Implementation](implementation.md)
- [Organization and membership](../organization/adr.md)
