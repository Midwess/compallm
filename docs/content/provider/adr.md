---
title: Provider management
available: true
---

# Provider management

## Status

Decision status: Organization ownership, management authorization, and provider
availability, batch creation, and optional connection testing are accepted;
credential protection and provider-specific verification remain unresolved.
Each provider model requires explicit input, output, and cache-read prices.
Catalog presets may prefill editable prices; creation never infers missing
rates or substitutes zero.
Provider discovery lists provider identities independently of their models.
After choosing a provider, callers choose enabled models from that provider's
catalog and may add custom model definitions with explicit prices.
Implementation status: Persistence schema and repository implemented. The Core
provider entity represents its associated provider models, but repository
relation loading, the Core management service, API, and provider integrations
are not implemented. The library provides a consumer-controlled provider form
with creation, saved-configuration, and clone previews; see [component integration](../portal/add-provider.md#component-integration).

Provider connections belong to one organization. An authenticated owner or
administrator may manage the LLM providers in that organization. Ordinary
members cannot add, change, or remove provider connections, and a role in one
organization grants no access to providers in another organization.

The planned Core `ProviderIntegrationService` in
`provider/integration_service.rs` coordinates organization-scoped provider
creation, input validation, name conflicts, and credential protection. Creation
accepts multiple providers in one request and does not contact upstream
providers or require a successful connection test.

A separate planned API accepts complete unsaved connection configuration and
tests it without inserting or updating any provider. Testing is optional and
can happen before creation. It validates connection fields and organization
access, but does not check provider-name uniqueness. Concrete provider clients
perform upstream requests using reused pools and bounded timeouts, outside
database transactions. Subscription integration remains separate. See
[Managing providers](management.md) for the authorized behavior and credential
boundaries.

Each provider connection has an independent `enabled` flag and defaults to
enabled. Disabling a provider makes every associated provider model and router
binding unavailable without deleting the provider, its credential, or its
configuration. Management listings retain disabled providers; future routing
and discovery workflows must exclude them from availability.

Saved-provider viewing and editing keep credentials write-only. Cloning prepares
an independent unsaved definition from non-secret configuration and model rates,
requires a new key, and does not copy routes, usage, or test results. See
[viewing, editing, and cloning](management.md#viewing-editing-and-cloning).
These management flows remain planned for runtime integration.

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

The [API contract](api-contract.md) defines proposed request/response schemas
for `POST /api/providers` and `POST /api/providers/test-connection`. Atomicity,
name comparison, limits, and error extensions remain proposed as specified
there. A successful connection test is a result for the submitted configuration
at that moment, not a requirement or guarantee for later creation or inference.

## Related pages

- [Managing providers](management.md)
- [Provider creation API contract](api-contract.md)
- [Models and providers](../model/models-and-providers.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
