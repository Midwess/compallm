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

Implementation status: The migrations and generated PostgreSQL schema implement
typed logical models, `provider_models`, and `model_router` through one migration
per table. Migration execution is separate and requires a fresh database.
Model and route management services, management APIs, provider discovery,
capability refresh, route selection, and request forwarding are not implemented.

## Concepts

### Model

A model is an organization-owned logical identity exposed to members and API
clients. Its name is unique within that organization. For example, an
organization can publish `general-chat` without exposing which external
provider serves each request.

A model owns caller-facing configuration:

- the stable model name;
- `service_kind` and `enabled`, defaulting to enabled;
- declared `capabilities`, `limits`, and optional effort/budget `thinking`;
- typed `pricing` used for organization billing and reporting.

These columns share the provider-model shapes, but represent a caller-facing
contract, not automatically inherited upstream configuration. Logical models
exclude `provider_id`, `protocol`, `connection`, and `headers`. The proposed
typed columns replace generic `settings` and scalar price columns; no generic
timestamps are retained. Names remain unique within an organization.

A disabled logical model cannot serve requests. Enabling it does not override
disabled provider models or bindings. Discovery visibility remains Gateway
policy, with no separate stored visibility flag. Models contain no credentials.

### Provider connection

A provider connection is an organization-owned configuration for reaching an
external service. In the proposal it owns a management name, shared connection,
headers, and protected credential. Service kind and protocol belong to the
provider model. Provider connections have no provider-wide enabled flag.

A provider connection is not a model. A connection can have multiple provider
models, each representing an upstream model the connection can call. The same
provider model can be bound to multiple logical models, and a logical model can
use provider models from multiple connections.

Credentials are write-only at the management boundary. They must not appear
in model, route, discovery, or usage responses, logs, or client-visible errors.

### Provider model

A provider model represents one upstream model available through one provider
connection in the same organization. It stores:

- the upstream model name accepted by that provider;
- `service_kind`, a service-specific `protocol`, a required complete connection,
  and headers;
- capabilities, limits, optional thinking controls, and upstream pricing using
  the same types as logical models;
- an independent `enabled` state.

Provider models are unique by organization, provider, name, service kind, and
protocol. Database constraints reject cross-organization ownership.

Capabilities are distinct flags defaulting to `[]`; limits are an object
defaulting to `{}`. Null thinking means no configured controls. A non-null
thinking object has optional effort and budget fields, at least one non-null;
both present means both are supported. Concrete adapters validate request-level
combinations. Pricing has one typed token, audio, image, video, or request
variant, with exact decimal-string USD rates. Detailed contracts are in the
Core `provider::configuration` module and the diagram.

The database stores `thinking` as nullable JSONB. Its concrete Rust structure and
validation do not require a PostgreSQL composite type or reasoning-level enum.

### Provider-model storage

The provider-model migration creates the diagram's `service_kind`, `protocol`,
required `connection`, public `headers`, declared `capabilities`, `limits`,
nullable `thinking`, and required `pricing`. JSONB object/array checks guard
column shapes; Rust deserialization owns typed fields. Adapters and full
management validation remain planned.

The migration targets a fresh database; it has no data backfill, compatibility
structures, or inferred configuration. Required `service_kind`, `protocol`, and
`pricing` columns have no defaults. Provider models are unique across
organization, provider, name, service kind, and protocol. Each provider model
requires its own complete connection; no null-based inheritance applies.
The upstream base URL lives only in `connection.baseUrl`. Prices use explicit
decimal strings in typed `pricing`; reasoning controls use `thinking`. Down
migrations drop tables in reverse dependency order. No migration execution is
authorized here.

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
   └─ Provider connections (connection, headers, protected credential) own
       provider models and can serve multiple logical models
```

The model is the stable caller contract. A router binding is replaceable
delivery configuration, while a provider model is reusable upstream-provider
configuration. Changing an upstream model name, disabling a binding, or moving
traffic to another provider does not require clients to select a new logical
model name.

A router binding is eligible in the diagram only when these records are enabled:

```text
logical model enabled AND provider model enabled AND binding enabled
```

Management listings retain disabled logical models, provider models, and
bindings. Discovery and routing workflows apply availability and visibility
policy rather than relying on the repository to filter management data.

## Intended request flow

1. The Gateway authenticates the caller, establishes its organization, and
   checks whether it may use the requested logical model.
2. The Gateway rejects disabled models and applies discovery and access rules, which decide which
   models the caller may see and use.
3. The Router loads `model_router` bindings whose provider models and provider
   connections are in the same organization and whose provider model and binding
   are enabled. Service/capability matching rules remain to be defined.
4. The Router selects a binding according to rules that still need a decision.
5. A concrete provider client sends the provider model's upstream name using the
    model's service/protocol adapter, effective connection, and protected credential.
6. Usage records identify the requested model, provider model, and binding that
   served the request.

Provider HTTP calls occur outside database transactions. A transaction can
protect configuration changes, but it cannot make an external inference call
atomic with a database update.

## Configuration boundaries

| Information | Owner |
| --- | --- |
| Stable caller-facing name and published prices | Model |
| Caller-facing service, enabled state, capabilities, limits, and thinking | Model |
| Shared connection, headers, and credential | Provider connection |
| Upstream name, service/protocol, connection override, headers, typed configuration, and upstream prices | Provider model |
| Route priority and availability | `model_router` binding |
| Applicable rates used to estimate one provider attempt's cost | Each typed provider-usage record |
| Health, quota, cooldown, and observed operational data | Separate operational state |

Published model prices remain stable across router bindings and do not
necessarily equal configured provider-model prices. Provider cost is estimated
from typed measurements and provider-model prices. Each typed provider-usage
record snapshots applicable provider-model rates in dedicated decimal columns,
so historical upstream cost estimates remain reproducible after later price
changes. Operational observations must not overwrite model prices,
provider-model configuration, or router bindings.

## Open questions

- Which organization roles may create, update, and remove models, provider
  models, and router bindings?
- Must selection always begin with the lowest priority, and can policy or
  health temporarily change that order?
- Which provider outcomes permit fallback, and how do retries, cooldowns,
  cancellation, and charging interact?
- How are provider-reported capabilities, Compallm registry data, and
  administrator overrides reconciled during model discovery and refresh?
- How do logical-model service, capabilities, limits, and thinking constrain
  eligible provider models? No automatic merge or inheritance is defined.
- How are meaningful existing settings preserved when replacing generic JSON
  settings with typed columns? Media member-charging rules remain unresolved.
- What happens when no eligible binding remains for a model?

## Related decisions

- [Model management](adr.md)
- [Provider management](../provider/adr.md)
- [Managing providers](../provider/management.md)
- [LLM Router](../llm-router/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
