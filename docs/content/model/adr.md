---
title: Model management
available: true
---

# Model management

## Status

Decision status: Organization ownership, pricing, and provider routes are
accepted. Model-management authorization, route selection, and fallback error
rules remain proposed. Implementation status: Persistence schema and repository
implemented; model services, APIs, and routing are not implemented.

## Context and decision

A model is an organization-owned logical name exposed to members and clients.
It remains stable when the organization changes upstream providers. Model names
are unique within an organization but may be reused by another organization.

For example, an organization can expose `general-chat` while routing it to
`gpt-5-mini` through one provider and `claude-sonnet-4-5` through another. The
logical name identifies what the organization offers; the upstream model name
identifies what a provider accepts.

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

## Provider routes

A model can have multiple provider routes. Each route records:

- one provider owned by the same organization;
- the model name expected by that provider;
- whether the route is enabled.

A provider may appear only once for one model. Database ownership
constraints prevent a route from joining a model and provider from different
organizations. The rule for selecting among multiple enabled routes and the
errors that permit fallback remain unresolved.

## Consequences

- Clients can use one stable organization model name while routes change.
- Usage must identify both the model and the provider route actually
  used.
- Provider credentials remain on provider connections and never appear in
  model or route responses.
- Provider HTTP calls must occur outside database transactions.
- Upstream cost, health, quota, and cooldown data must not overwrite the
  organization-facing model price or route configuration.
- Disabling a model prevents routing regardless of its visibility. Hiding a
  model affects discovery only and does not imply that it is disabled.
- A route is available only when its model, provider connection, and route are
  all enabled. Disabling a provider therefore makes every associated route
  unavailable without deleting configuration.
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
- How does the Router select among enabled routes?
- Which provider errors permit fallback, and how are cooldowns represented?
- Where are upstream provider costs and capability metadata maintained?

## Related pages

- [Models and providers](models-and-providers.md)
- [LLM Router](../llm-router/adr.md)
- [Provider management](../provider/adr.md)
- [Organization and membership](../organization/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
