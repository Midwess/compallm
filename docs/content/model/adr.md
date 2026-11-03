---
title: Model management
available: true
---

# Model management

## Status

Decision status: Organization ownership, shared logical/provider-model
configuration, model-level enabling, pricing separation, and router bindings
are accepted for the proposed schema. Authorization, route selection, capability
matching, and fallback rules remain unresolved. Implementation status: The
diagram shows the proposed typed configuration. Persistence still uses scalar
token prices, logical-model settings, and direct provider reasoning fields.
Management services, discovery, capability refresh, and routing are not implemented.

## Context and decision

A model is an organization-owned logical name exposed to members and clients.
It remains stable when the organization changes upstream providers. Model names
are unique within an organization but may be reused by another organization.

For example, an organization can expose `general-chat` while routing it to the
provider models `gpt-5-mini` through one provider and `claude-sonnet-4-5`
through another. The system model identifies what the organization offers; a
provider model identifies what a provider accepts.

Logical and provider models share `id`, `organization_id`, `name`,
`service_kind`, `enabled`, `capabilities`, `limits`, `thinking`, and `pricing`.
Logical-model configuration is the caller-facing contract; provider-model
configuration describes an upstream endpoint. Logical models have no
`provider_id`, `protocol`, `connection`, or `headers`. Routes select the upstream
connection, keeping caller configuration independent of provider transport.

`capabilities` is a distinct flag array defaulting to `[]`; `limits` is an
object defaulting to `{}`. Nullable `thinking` contains independent optional
effort and budget controls, with at least one configured when non-null. Both
present means both are supported. These typed fields replace logical-model
`settings`; no generic timestamps are part of the proposed model schema.

`pricing` uses the same token, audio, image, video, or request JSONB shapes for
both records, with exact nonnegative decimal-string USD rates. Logical-model
prices are organization-facing charges; provider-model prices estimate upstream
costs. Sharing a shape does not share values or define media charging policy.

Models default to enabled. A disabled logical model cannot serve requests even
when its routes are enabled. Enabling it does not override disabled upstream
models or bindings. Visibility remains a separate Gateway policy, not a stored
model flag.

## Provider models and routes

A provider owns multiple provider models in the same organization. In addition
to shared configuration, each provider model owns its provider reference,
service-specific protocol, optional replacement connection, and headers. The
service/protocol pair selects a concrete adapter as defined in
`suggest-schema.md`. Credentials remain on the provider connection.

A provider model is independent of a system model and may be routed from more
than one system model. `model_router` binds one system model to one provider
model. A system model can bind multiple provider models, including models from
multiple providers. Each binding has a positive priority unique within its
system model and an independent enabled state. Database ownership constraints
prevent a provider model or route from crossing organization boundaries.
Priority expresses an order without deciding whether the Router must always
select the lowest value first. Selection and fallback rules remain unresolved.

## Consequences

- Clients can use one stable organization model name while routes change.
- Usage must identify the system model, provider model, and router binding
  actually used.
- Provider credentials remain on provider connections and never appear in
  model or route responses.
- Provider HTTP calls must occur outside database transactions.
- Configured provider-model prices support upstream cost estimates but are not
  provider-reported costs. They must not overwrite organization-facing model
  prices.
- Health, quota, and cooldown data must not overwrite provider-model or router
  binding configuration.
- Each provider attempt snapshots its provider-model prices. Historical upstream
  cost estimates therefore remain reproducible after provider-model prices
  change.
- A model must be enabled and have an eligible router binding to serve
  requests. Discovery rules remain Gateway access policy.
- Diagram route eligibility requires the logical model, provider model, and
  binding to be enabled. The provider draft has no provider-level enabled flag;
  provider-wide disabling remains an unresolved conflict with provider management.
- Model persistence operations are organization-scoped. Repository listing
  returns every model in the organization for management; future workflows
  apply discovery and routing policy.
- Shared shapes do not imply automatic inheritance or merging. Logical-model
  capabilities, limits, and thinking are configured independently; route
  compatibility and request validation need an explicit contract.
- Routing can use provider-model reasoning capabilities without storing
  provider request templates or coupling logical models to one provider.

## Behavior examples

### Model availability

- **WHEN** a logical model is disabled or has no eligible router binding
- **THEN** Compallm has no eligible route for that model and does not serve
  requests for it

### Organization isolation

- **WHEN** a route attempts to associate a model with another organization's
  provider
- **THEN** the association is rejected

## Open questions

- May administrators and owners both manage models and routes, or do those
  operations require different permissions?
- Must the Router always begin with the lowest-priority enabled route, and can
  policy or health change that order?
- How does an organization withdraw a model from discovery without deleting
  its router bindings?
- Which provider errors permit fallback, and how are cooldowns represented?
- How are provider-reported capabilities, Compallm registry data, and
  administrator overrides reconciled during model discovery and refresh?
- How must a route satisfy the logical model's service, declared capabilities,
  limits, and thinking controls? Shared columns do not decide fallback eligibility.
- How are existing logical-model settings migrated without losing meaningful
  configuration? Organization-facing media prices and historical charge
  snapshots also need a contract before accounting implementation.

## Related pages

- [Models and providers](models-and-providers.md)
- [LLM Router](../llm-router/adr.md)
- [Provider management](../provider/adr.md)
- [Organization and membership](../organization/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
