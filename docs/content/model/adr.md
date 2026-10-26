---
title: Model management
available: true
---

# Model management

## Status

Decision status: Organization ownership, pricing, access levels, and ordered
provider routes are accepted. Model-management authorization and fallback error
rules remain proposed. Implementation status: Persistence schema under
development; model services, APIs, and routing are not implemented.

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

Each model also records a non-negative minimum access level. A member may use a
model when their level in that organization is greater than or equal to the
model's minimum level. Existing and new models default to level `0`. Membership
roles govern administration separately and do not replace model access levels.

## Provider routes

A model can have multiple provider routes. Each route records:

- one provider owned by the same organization;
- the model name expected by that provider;
- whether the route is enabled;
- a positive priority unique within the logical model.

Lower priority values are attempted first. A provider may appear only once for
one logical model. Database ownership constraints prevent a route from joining
a model and provider from different organizations.

Priority defines preference, not permission to retry every failure. The Router
may continue to the next enabled route only for an explicitly fallback-eligible
outcome, such as exhausted upstream quota or temporary unavailability. Reaching
the requested output-token limit is a successful completion, not a fallback
condition. Detailed fallback categories remain unresolved.

## Consequences

- Clients can use one stable organization model name while routes change.
- Usage must identify both the logical model and the provider route actually
  used.
- Provider credentials remain on provider connections and never appear in
  model or route responses.
- Provider HTTP calls must occur outside database transactions.
- Upstream cost, health, quota, and cooldown data must not overwrite the
  organization-facing model price or route priority.

## Behavior examples

### Access level

- **WHEN** a member at level `2` requests a model with minimum level `1`
- **THEN** the model passes the level check
- **AND WHEN** the member requests a model with minimum level `3`
- **THEN** access is denied before contacting a provider

### Ordered fallback

- **WHEN** the first enabled route reports an explicitly fallback-eligible
  failure
- **THEN** the Router attempts the next enabled route by ascending priority

### Organization isolation

- **WHEN** a route attempts to associate a model with another organization's
  provider
- **THEN** the association is rejected

## Open questions

- May administrators and owners both manage models and route priorities, or do
  those operations require different permissions?
- Which provider errors permit fallback, and how are cooldowns represented?
- Where are upstream provider costs and capability metadata maintained?

## Related pages

- [LLM Router](../llm-router/adr.md)
- [Provider management](../provider/adr.md)
- [Organization and membership](../organization/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
