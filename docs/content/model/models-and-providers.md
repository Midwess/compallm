---
title: Models and providers
---

# Models and providers

## Purpose

This page explains how organization-owned models relate to provider
connections and provider routes. It describes the intended product design so
model-management and routing behavior can be agreed before those workflows are
implemented. The persistence foundation exists, but model-management services,
APIs, route selection, and request forwarding are not implemented.

## Concepts

### Model

A model is the stable identity that an organization exposes to
its members and clients. For example, an organization might publish
`general-chat`. Callers select that logical name without needing to know which
external provider or upstream model serves it.

A model owns organization-facing configuration:

- a name unique within its organization;
- input, output, and cached-token prices used for the organization's billing
  and reporting;
- `enabled`, which determines whether requests may be routed;
- `hidden`, which controls ordinary discovery without changing direct use;
- a JSON settings object for model-specific configuration validated by the
  service that owns each setting.

Settings must not contain provider credentials or other secrets.

### Provider connection

A provider connection describes how one organization connects to an external
LLM service. It owns the provider protocol, operation URL, and credential. A
provider connection is not itself a caller-facing model: one provider can host
several upstream models, and one model can use several providers.

The provider's `enabled` flag controls whether any associated route is
available. Disabling it preserves the connection and route configuration for
management while preventing routing through that provider.

Provider credentials remain server-side. Model, route, discovery, and
usage responses must not expose them.

### Provider route

A provider route joins one model to one provider connection in the
same organization. It records:

- the upstream model name understood by that provider;
- a positive priority that can express route order;
- whether the route is enabled.

One model cannot use the same provider connection twice, and two routes for the
same model cannot have the same priority. Database
constraints reject routes that join records from different organizations.

## Relationship

```text
Organization
  ├─ Model: general-chat
  │    ├─ route priority 1 ─> OpenAI provider ─> gpt-5-mini
  │    └─ route priority 2 ─> Anthropic provider ─> claude-sonnet-4-5
  └─ Provider connections
       ├─ OpenAI provider (URL, protocol, secret credential)
       └─ Anthropic provider (URL, protocol, secret credential)
```

The model is the stable contract with the caller. Provider routes are
replaceable delivery configuration. Changing an upstream model name, disabling
a route, or moving to another provider does not require changing the logical
name exposed to callers.

## Request flow

The intended flow is:

1. An authenticated caller selects a model in its organization.
2. The Router rejects a disabled model. Ordinary discovery also omits a hidden
   model, although its stable name can still be used directly.
3. The Router loads enabled routes whose provider connections are also enabled.
4. The Router selects one route according to the future route-selection rules.
5. The provider client uses that route's upstream model name, the provider's
   protocol and URL, and its protected credential.
6. Usage records identify both the model requested by the caller and
   the provider route that served the request.

Steps 3–5 describe the intended design, not current implemented behavior.
Priority exists in persistence, but whether selection is strictly
lowest-number-first and when another route may be attempted remain open.

## Configuration boundaries

| Information | Owner | Reason |
| --- | --- | --- |
| Stable caller-facing name | Model | Remains independent of provider changes |
| Organization billing prices | Model | Represents the organization's published rates |
| Model-specific non-secret settings | Model | Applies to the organization-facing model |
| Provider URL and protocol | Provider connection | Defines one external integration |
| Provider credential | Provider connection | Keeps secrets out of models and routes |
| Provider availability | Provider connection | Enables or disables every associated route |
| Upstream model name | Provider route | Can differ for each provider |
| Route priority and availability | Provider route | Configures delivery for one model-provider pair |
| Provider cost, health, quota, cooldown | Separate operational state | Must not overwrite published prices or route configuration |

Provider HTTP calls must occur outside database transactions. Management
services may change model, provider, and route configuration transactionally,
but a database transaction cannot make an external inference request atomic.

## Current status and open questions

Documented persistence decisions include organization ownership, model and
provider lifecycle flags, JSON settings storage, provider routes, route
priority, and same-organization constraints. The schema and model/provider
repositories form the persistence foundation.

The following behavior must be decided before routing is implemented:

- Which organization roles may manage models and routes?
- Is the lowest positive priority always attempted first?
- Which failures permit fallback to another route?
- How are retries, cooldowns, health, quotas, and request cancellation handled?
- Where are provider capability metadata and upstream costs maintained?
- Which keys and validation rules are introduced in model settings?

## Related decisions

- [Model management](adr.md)
- [Provider management](../provider/adr.md)
- [LLM Router](../llm-router/adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
