---
title: Provider management
available: true
---

# Provider management

## Status

Decision status: Organization ownership, management authorization, and provider
availability are accepted; credential protection and integration behavior
remain proposed.
Implementation status: Persistence schema and repository implemented. The Core
provider entity represents its associated provider models, but repository
relation loading, the Core management service, API, and provider integrations
are not implemented.

Provider connections belong to one organization. An authenticated owner or
administrator may manage the LLM providers in that organization. Ordinary
members cannot add, change, or remove provider connections, and a role in one
organization grants no access to providers in another organization.

Management provides explicit operations for adding OpenAI, Anthropic, and
OpenAI-compatible APIs. Provider API calls and subscription integration belong
in separate integration services rather than one general provider manager. See
[Managing providers](management.md) for the authorized behavior and credential
boundaries.

Each provider connection has an independent `enabled` flag and defaults to
enabled. Disabling a provider makes every associated provider model and router
binding unavailable without deleting the provider, its credential, or its
configuration. Management listings retain disabled providers; future routing
and discovery workflows must exclude them from availability.

## Consequences

Provider persistence and services require organization scope. The repository
only executes requested data operations and maps database outcomes; provider
counts, query limits, and other business validation belong to the future
workflow that invokes it. Each add operation selects its protocol internally,
so callers cannot persist a mismatched protocol. Credentials stay outside
ordinary provider read models and must not appear in API responses or logs.
Provider persistence round-trips availability without applying routing policy.

## Open questions

Credential encryption, rotation, and provider-specific integration behavior
remain unresolved.

## Related pages

- [Managing providers](management.md)
- [Models and providers](../model/models-and-providers.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
