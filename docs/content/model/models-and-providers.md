---
title: Models and model providers
---

# Models and model providers

## Purpose and status

This page defines how an organization-facing model relates to an upstream
provider connection. It is the intended design for future management and
routing workflows, not a description of an available API.

Decision status: The ownership boundaries, stable model identity, provider
models, and router bindings are accepted. Route selection, fallback,
model-management authorization, and model discovery remain unresolved.

Implementation status: The migration and generated PostgreSQL schema implement
`provider_models` and `model_router`. Model and route management services,
management APIs, route selection, and request forwarding are not implemented.

## Concepts

### Model

A model is an organization-owned logical identity exposed to members and API
clients. Its name is unique within that organization. For example, an
organization can publish `general-chat` without exposing which external
provider serves each request.

A model owns caller-facing configuration:

- the stable model name;
- input, output, and cached-token prices used for organization billing and
  reporting;
- non-secret model settings whose shape is validated by the service that
  introduces each setting.

A model owns no availability or visibility flag. Whether Compallm serves it
depends on its router bindings, and whether it appears in a discovery list
depends on Gateway access rules that are still to be decided. Model settings
must not contain provider credentials.

### Provider connection

A provider connection is an organization-owned configuration for reaching an
external LLM service. It owns a management name, API protocol, endpoint URL,
protected credential, and `enabled` state. OpenAI, Anthropic, and
OpenAI-compatible connections are planned initially.

A provider connection is not a model. A connection can have multiple provider
models, each representing an upstream model the connection can call. The same
provider model can be bound to multiple logical models, and a logical model can
use provider models from multiple connections. Disabling a connection preserves
it for management but makes all routes through it unavailable.

Credentials are write-only at the management boundary. They must not appear
in model, route, discovery, or usage responses, logs, or client-visible errors.

### Provider model

A provider model represents one upstream model available through one provider
connection in the same organization. It stores:

- the upstream model name accepted by that provider;
- configured provider input, output, and cached-token prices in fixed-point USD
  per one million tokens;
- an independent `enabled` state.

Provider-model names are unique per provider connection. Database constraints
reject provider models that cross organization boundaries.

### Model router binding

A `model_router` binding associates one logical model with one provider model
in the same organization. A logical model may have multiple bindings, and the
same provider model may be bound to multiple logical models. The binding owns:

- a positive priority unique within the logical model;
- an independent `enabled` state.

Priority records an explicit order, but it does not yet decide whether routing
must always use ascending priority, when a later binding may be attempted, or
whether future health and policy information may affect selection.

## Relationship and availability

```text
Organization
  ├─ Model: general-chat
  │    ├─ model_router: priority 1 ─> provider_model: gpt-5-mini
  │    │                                └─ OpenAI connection
  │    └─ model_router: priority 2 ─> provider_model: claude-sonnet-4-5
  │                                     └─ Anthropic connection
  └─ Provider connections (endpoint, protocol, protected credential) own
       provider models and can serve multiple logical models
```

The model is the stable caller contract. A router binding is replaceable
delivery configuration, while a provider model is reusable upstream-provider
configuration. Changing an upstream model name, disabling a binding, or moving
traffic to another provider does not require clients to select a new logical
model name.

A router binding is available only when all three records are enabled:

```text
provider enabled AND provider model enabled AND binding enabled
```

Management listings retain disabled provider connections, provider models, and
bindings. Discovery and routing workflows apply availability and visibility
policy rather than relying on the repository to filter management data.

## Intended request flow

1. The Gateway authenticates the caller, establishes its organization, and
   checks whether it may use the requested logical model.
2. The Gateway applies its discovery and access rules, which decide which
   models the caller may see and use.
3. The Router loads `model_router` bindings whose provider models and provider
   connections are in the same organization and whose provider, provider model,
   and binding are enabled.
4. The Router selects a binding according to rules that still need a decision.
5. A concrete provider client sends the provider model's upstream name using the
   connection's protocol, endpoint, and protected credential.
6. Usage records identify the requested model, provider model, and binding that
   served the request.

Provider HTTP calls occur outside database transactions. A transaction can
protect configuration changes, but it cannot make an external inference call
atomic with a database update.

## Configuration boundaries

| Information | Owner |
| --- | --- |
| Stable caller-facing name and published prices | Model |
| Non-secret model behavior settings | Model |
| Endpoint, protocol, credential, and connection availability | Provider connection |
| Upstream model name, configured provider prices, and availability | Provider model |
| Route priority and availability | `model_router` binding |
| Price snapshot used to estimate one provider attempt's cost | `provider_usage` |
| Health, quota, cooldown, and observed operational data | Separate operational state |

Published model prices remain stable across router bindings and do not
necessarily equal configured provider-model prices. Providers return token usage
rather than a USD cost, so provider cost is estimated from provider-model
prices. `provider_usage` snapshots the three provider-model rates when an
attempt is created, so historical upstream cost estimates remain reproducible
after later price changes. Operational observations must not overwrite model
prices, provider-model configuration, or router bindings.

## Open questions

- Which organization roles may create, update, and remove models, provider
  models, and router bindings?
- Must selection always begin with the lowest priority, and can policy or
  health temporarily change that order?
- Which provider outcomes permit fallback, and how do retries, cooldowns,
  cancellation, and charging interact?
- Where is provider capability metadata maintained?
- Which model setting keys and validation rules are introduced first?
- What happens when no eligible binding remains for a model?

## Related decisions

- [Model management](adr.md)
- [Provider management](../provider/adr.md)
- [Managing providers](../provider/management.md)
- [LLM Router](../llm-router/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
