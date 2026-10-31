---
title: LLM Router
available: false
---

# LLM Router

## Status

Implementation: Model, provider-model, and route persistence are implemented;
runtime routing and protocol translation remain planned.

The following solution is planned; unresolved requirements are listed below.

## Solution

Connect multiple providers and offer a consistent way to select and call their models.

- Let clients choose a supported API format independently of the upstream provider, so switching providers does not require changing the client's request or response handling.
- Normalize incoming requests into a unified internal format, then translate them into the selected provider's native API format.
- Translate provider responses back into the client's requested format, including streamed events, tool calls and results, supported multimodal content, usage information, completion reasons, and errors.
- Configure provider connections, endpoints, and multiple upstream API credentials.
- Discover available models and their supported capabilities, such as text, images, tools, and structured output.
- Select a provider and model explicitly, or use organization-defined routing preferences.
- Support routing by model availability, capability, latency, and cost where provider information permits it.
- Handle provider failures and rate limits with controlled retry and fallback behavior.
- Preserve supported provider capabilities during translation and explicitly report unsupported features or options instead of silently discarding them.

For example, a client using an OpenAI-compatible format could select an Anthropic or Gemini model. Compallm translates the request for that provider and returns the result in the same OpenAI-compatible format the client expects. The exact supported API formats and translation coverage remain to be defined; format conversion does not give a model capabilities it does not support.

## Responsibilities and unresolved requirements

The client API format is independent of the provider. Translate both directions through a unified internal representation and explicitly reject unsupported capabilities. Router selects providers and translates formats; [Gateway](../llm-gateway/adr.md) enforces access, allowances, and usage attribution.

Each organization owns models whose names are unique within that organization.
A model gives callers a stable name independent of an upstream provider. Each
provider has provider models that store the upstream model name, configured
provider token prices, and whether that provider model is enabled. A
`model_router` binding connects a logical model to a provider model and stores a
positive priority unique within the logical model and whether the binding is
enabled. One provider model can serve multiple logical models, and each logical
model can bind multiple provider models. Database constraints prevent provider
models and bindings from crossing organization boundaries. Priority expresses
binding order without yet deciding whether selection must always begin with the
lowest value or when a later binding may be attempted.

A binding is eligible only when the provider connection, provider model, and
binding are all enabled. A disabled provider remains available to management
but cannot serve requests through any associated binding. A model with no
eligible binding cannot serve requests.

Each model stores the organization-facing input, output, and cached
token prices in USD per one million tokens. These are non-negative fixed-point
rates used for stable billing and reporting regardless of the selected route.
Configured upstream provider prices can differ and therefore belong to each
provider model rather than the logical model. Providers report token usage
rather than USD cost; `provider_usage` snapshots those provider-model prices at
attempt creation, so any historical upstream-cost estimate uses the rates in
effect for that attempt.

Before implementation, decide which API compatibility standard, providers, and model capabilities ship first, and whether credentials are organization-owned only or members may bring personal credentials.

## Custom models

Custom models define ordered provider/model routes. Down or inactive providers are excluded. Observed serving shares are telemetry and do not configure traffic weights. Optional token prices use USD per million tokens. Runtime routing, telemetry, and pricing integration remain planned; failure policy, reporting periods, price precedence, and charging rules remain unresolved. See [custom model routing](custom-model.md).

## Consequences

Provider adapters must preserve supported capabilities and report translation limits explicitly. A unified format does not make unsupported provider capabilities available.

## Behavior examples

### Client-selected format translation
The Router should translate supported client requests through a unified internal format to the selected provider, and return responses in the client's requested format.

#### Example: Different provider protocol
- **WHEN** a client sends a supported OpenAI-compatible request targeting a supported Anthropic or Gemini model
- **THEN** the Router translates the request and returns the response using the client's protocol, including streamed events when requested

### Explicit translation limitations
The Router should explicitly report unsupported requested features or options rather than silently discard them during conversion.

#### Example: Unsupported capability
- **WHEN** a request requires a capability unavailable from the selected model or adapter
- **THEN** the Router returns an explicit compatibility error rather than silently omitting the capability

## Related pages

- [LLM Gateway](../llm-gateway/adr.md)
- [Product overview](../overview.md)
