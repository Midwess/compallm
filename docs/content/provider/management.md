---
title: Managing LLM providers
---

# Managing LLM providers

LLM provider connections are organization-owned resources. They hold the
endpoint, protocol, and protected credentials Compallm needs to call an
upstream provider without exposing those credentials to organization members.

## Who can manage providers

An authenticated organization **owner** or **administrator** can:

- list the organization's provider connections;
- add an OpenAI, Anthropic, or OpenAI-compatible connection;
- change a connection's non-secret configuration or replace its credential;
- enable or disable a connection;
- remove a connection from the organization.

An ordinary **member** cannot add, change, or remove provider connections. A
caller must hold the required role in the organization that owns the provider;
being an owner or administrator in another organization does not grant access.

Authorization uses organization membership and role.

## Creation and connection testing

Creation accepts multiple providers for one organization in a single request.
The planned `ProviderIntegrationService` validates input, checks provider-name
conflicts within that organization and within the submitted batch, protects
credentials, and persists the providers. PostgreSQL's organization/name unique
constraint remains the final guard against concurrent conflicting requests.
An application precheck cannot replace that constraint.

Creation does not call upstream providers and never requires a successful
connection test. A saved provider therefore does not imply verified credentials
or a reachable endpoint.

A separate planned connection-test API accepts full connection configuration,
including the credential, without requiring a stored provider ID. An owner or
administrator can test the configuration before deciding to save it. Testing
does not insert or update providers, models, or routes, and does not reserve
provider names. Existing names do not prevent testing. The result applies only
to the submitted configuration at the time of the test.

Connection testing uses concrete provider clients with reused HTTP pools and
bounded timeouts. No database transaction stays open during a provider call.
Responses expose a safe test result without credentials or raw upstream bodies.
The exact probe, possible usage charges, and supported endpoint modes must be
defined before implementing the test API.

The [API contract](api-contract.md) proposes the batch and unsaved-test request
schemas, responses, atomicity, name comparison, and per-item errors. Batch
limits and the identified persistence gaps remain unresolved.

Each connection contains one or more upstream models. Each model requires
explicit input, output, and cache-read prices. Catalog presets may prefill
editable values, but missing, null, or blank rates are invalid. An explicit
zero is a configured free rate; it is never inferred. Changing one model's
price does not affect other models. Submitted rates are stored per provider
model; creation does not resolve missing prices from the catalog.

Provider discovery starts with distinct provider identities, without requiring
a model selection. Choosing a provider scopes model discovery to that
provider's known models. Callers explicitly enable the models they want to
configure and can add custom models to the same connection. Merely viewing
catalog entries does not configure or enable them. Creation submits only the
enabled model definitions; unselected catalog entries remain catalog data.
Every submitted catalog or custom model requires all three explicit prices.

## Viewing, editing, and cloning

Planned provider management opens saved configuration directly for editing.
Save changes is disabled while the values match the saved configuration and
enabled only after an actual change. Reverting every change disables saving
again. Reading never returns the credential; leaving its replacement empty
preserves the stored key. Cancel discards edits rather than updating the saved
provider.

Cloning opens a new unsaved provider definition in the same organization. It
copies non-secret connection configuration and configured models, including
explicit prices and availability. The draft name defaults to the source name
followed by ` clone` and remains editable. A new credential is required; cloning
does not expose or copy the stored secret. Saving creates an independent
provider with new identities, using normal creation validation and name-conflict
checks. It does not copy router bindings, usage records, or connection-test
results. Cancel leaves the source unchanged. Provider read/update/clone runtime
integration remains unimplemented.

## Credential boundaries

Provider credentials are write-only from the management interface. Responses
may identify a connection and show its safe configuration, but must not return
the API key or another reusable secret. Logs and client-visible errors must not
contain credentials.

Replacing a credential accepts a new protected value without revealing the
old value. Removing a provider also removes its model routes; it does not expose
the removed credential.

Credential encryption, key rotation, and provider-specific verification remain
unresolved. Neither creation nor a prior successful test guarantees that later
provider requests will succeed.

Disabling a provider preserves its configuration, credential, and model routes
for management, but makes all of those routes unavailable. Enabling the
provider does not override a disabled provider model or binding; routing
requires the provider connection, the provider model, and the binding to be
enabled.

## Expected behavior

### Authorized management

- **WHEN** an owner or administrator manages a provider owned by their current
  organization
- **THEN** Compallm performs the requested management operation without
  returning the provider credential

### Member denied

- **WHEN** an ordinary member attempts to add, change, or remove a provider
- **THEN** Compallm denies the operation before changing provider data

### Cross-organization access denied

- **WHEN** a caller attempts to manage a provider owned by another organization
- **THEN** Compallm denies the operation even if the caller is an owner or
  administrator elsewhere

### Disabled provider retained for management

- **WHEN** an owner or administrator disables a provider connection
- **THEN** Compallm retains it in management listings without returning its
  credential
- **AND THEN** routing does not use any route associated with that provider

## Related pages

- [Provider management decisions](adr.md)
- [Provider creation API contract](api-contract.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
- [LLM Router](../llm-router/adr.md)
