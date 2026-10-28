---
title: Models and model providers
---

# Models and model providers

## Purpose and status

This page defines how an organization-facing model relates to an upstream
provider connection. It is the intended design for future management and
routing workflows, not a description of an available API.

Decision status: The ownership boundaries, stable model identity, lifecycle
flags, and model-provider relationship are accepted. Route selection,
fallback, and model-management authorization remain unresolved.

Implementation status: The model and provider persistence foundation exists.
Model and route management services, management APIs, route selection, and
request forwarding are not implemented.

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
- `enabled`, which determines whether requests may be routed;
- `hidden`, which omits the model from ordinary discovery without preventing
  direct use of its stable name;
- non-secret model settings whose shape is validated by the service that
  introduces each setting.

Disabling a model always prevents routing. Hiding it changes discovery only.
Model settings must not contain provider credentials.

### Provider connection

A provider connection is an organization-owned configuration for reaching an
external LLM service. It owns a management name, API protocol, endpoint URL,
protected credential, and `enabled` state. OpenAI, Anthropic, and
OpenAI-compatible connections are planned initially.

A provider connection is not a model. One connection can serve several
upstream models, and one logical model can use connections from several
providers. Disabling a connection preserves it for management but makes all
of its routes unavailable.

Credentials are write-only at the management boundary. They must not appear
in model, route, discovery, or usage responses, logs, or client-visible errors.

### Model-provider route

A model-provider route associates one model with one provider connection in
the same organization. It stores:

- the upstream model name accepted by that provider;
- a positive priority unique within the model;
- an independent `enabled` state.

A model can have several routes, but it can use a particular provider
connection only once. Database constraints reject cross-organization routes.
Priority records an explicit order, but it does not yet decide whether routing
must always use ascending priority, when a later route may be attempted, or
whether future health and policy information may affect selection.

## Relationship and availability

```text
Organization
  ├─ Model: general-chat
  │    ├─ route 1 ─> OpenAI connection ─> gpt-5-mini
  │    └─ route 2 ─> Anthropic connection ─> claude-sonnet-4-5
  └─ Provider connections (endpoint, protocol, protected credential)
```

The model is the stable caller contract. A route is replaceable delivery
configuration. Changing an upstream model name, disabling a route, or moving
traffic to another provider does not require clients to select a new logical
model name.

A route is available only when all three records are enabled:

```text
model enabled AND provider enabled AND route enabled
```

Management listings retain disabled and hidden records. Discovery and routing
workflows apply availability and visibility policy rather than relying on the
repository to filter management data.

## Intended request flow

1. The Gateway authenticates the caller, establishes its organization, and
   checks whether it may use the requested logical model.
2. The Router rejects a disabled model. Ordinary discovery also omits a hidden
   model, although an authorized caller can still use its stable name directly.
3. The Router loads routes whose provider connections are in the same
   organization and whose model, provider, and route are enabled.
4. The Router selects a route according to rules that still need a decision.
5. A concrete provider client sends the upstream model name using the
   connection's protocol, endpoint, and protected credential.
6. Usage records identify the requested model and the route that served the
   request.

Provider HTTP calls occur outside database transactions. A transaction can
protect configuration changes, but it cannot make an external inference call
atomic with a database update.

## Configuration boundaries

| Information | Owner |
| --- | --- |
| Stable caller-facing name and published prices | Model |
| Non-secret model behavior settings | Model |
| Endpoint, protocol, credential, and connection availability | Provider connection |
| Upstream model name, route priority, and route availability | Model-provider route |
| Health, quota, cooldown, and actual upstream cost | Separate operational state |

Published model prices remain stable across routes and do not necessarily
equal provider costs. Operational observations must not overwrite model prices
or route configuration.

## Open questions

- Which organization roles may create, update, and remove models and routes?
- Must selection always begin with the lowest priority, and can policy or
  health temporarily change that order?
- Which provider outcomes permit fallback, and how do retries, cooldowns,
  cancellation, and charging interact?
- Where are provider capability metadata and actual upstream costs maintained?
- Which model setting keys and validation rules are introduced first?
- What happens when no eligible route remains for an otherwise enabled model?

## Related decisions

- [Model management](adr.md)
- [Provider management](../provider/adr.md)
- [Managing providers](../provider/management.md)
- [LLM Router](../llm-router/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
