---
title: Model management
available: true
---

# Model management

## Status

Decision status: Organization ownership, pricing, lifecycle flags, provider
models, and route configuration are accepted. Model-management authorization,
route selection, and fallback error rules remain proposed. Implementation
status: The migration and generated PostgreSQL schema implement
`provider_models` and `model_router`. Model services, route management APIs, and
routing are not implemented.

## Context and decision

A model is an organization-owned logical name exposed to members and clients.
It remains stable when the organization changes upstream providers. Model names
are unique within an organization but may be reused by another organization.

For example, an organization can expose `general-chat` while routing it to the
provider models `gpt-5-mini` through one provider and `claude-sonnet-4-5`
through another. The system model identifies what the organization offers; a
provider model identifies what a provider accepts.

Each model records organization-facing prices for input, output, and cached
tokens. Prices are non-negative fixed-point USD amounts per one million tokens.
They are stable billing and reporting rates, not necessarily the organization's
upstream provider costs.

Each model also has two independent lifecycle flags. `enabled` determines
whether Compallm may route requests to the model. `hidden` removes the model
from ordinary discovery and selection lists without disabling requests that
identify its stable name directly. Existing and new models default to enabled
and visible so introducing the flags does not remove current availability.

Each model has a `settings` JSON object for model-specific configuration whose
shape may evolve independently of the common model fields. PostgreSQL stores
it as non-null `JSONB`; existing and new models default to an empty object.
Services that introduce a setting remain responsible for validating its
documented shape before persistence. Settings must not contain provider
credentials or other secrets.

## Provider models and routes

A provider can have multiple provider models. A provider model records:

- one provider owned by the same organization;
- the upstream model name expected by that provider;
- configured provider prices for input, output, and cached tokens in fixed-point
  USD per one million tokens;
- whether the provider model is enabled.

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
- Disabling a model prevents routing regardless of its visibility. Hiding a
  model affects discovery only and does not imply that it is disabled.
- A route is available only when its system model, provider connection, provider
  model, and router binding are all enabled. Disabling a provider therefore
  makes every associated route unavailable without deleting configuration.
- Model persistence operations are organization-scoped. Repository listing
  returns both visible and hidden, enabled and disabled models for management;
  future workflows apply discovery and routing policy.
- Model settings round-trip through persistence as JSON. The repository does
  not interpret individual keys; owning services validate settings they use.

## Behavior examples

### Model lifecycle flags

- **WHEN** a model is disabled
- **THEN** Compallm does not route requests to it
- **AND WHEN** an enabled model is hidden
- **THEN** ordinary discovery omits it while direct use by stable name remains
  available

### Organization isolation

- **WHEN** a route attempts to associate a model with another organization's
  provider
- **THEN** the association is rejected

## Open questions

- May administrators and owners both manage models and routes, or do those
  operations require different permissions?
- Must the Router always begin with the lowest-priority enabled route, and can
  policy or health change that order?
- Which provider errors permit fallback, and how are cooldowns represented?
- Where is provider capability metadata maintained?

## Related pages

- [Models and providers](models-and-providers.md)
- [LLM Router](../llm-router/adr.md)
- [Provider management](../provider/adr.md)
- [Organization and membership](../organization/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
