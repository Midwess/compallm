---
title: Provider management
available: true
---

# Provider management

## Status

Decision status: Organization ownership and management authorization are
accepted; credential protection and integration behavior remain proposed.
Implementation status: Persistence foundation under development; the Core
management service and API are deferred until their workflows are implemented.

Provider connections belong to one organization. An authenticated owner or
administrator may manage the LLM providers in that organization. Ordinary
members cannot add, change, or remove provider connections, and a role in one
organization grants no access to providers in another organization.

Management provides explicit operations for adding OpenAI, Anthropic, and
OpenAI-compatible APIs. Provider API calls and subscription integration belong
in separate integration services rather than one general provider manager. See
[Managing providers](management.md) for the authorized behavior and credential
boundaries.

## Consequences

Provider persistence and services require organization scope. The repository
only executes requested data operations and maps database outcomes; provider
counts, query limits, and other business validation belong to the future
workflow that invokes it. Each add operation selects its protocol internally,
so callers cannot persist a mismatched protocol. Credentials stay outside
ordinary provider read models and must not appear in API responses or logs.

## Open questions

Credential encryption, rotation, and provider-specific integration behavior
remain unresolved.

## Related pages

- [Managing providers](management.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
