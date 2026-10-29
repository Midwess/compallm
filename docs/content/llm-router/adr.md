---
title: LLM Router
available: false
---

# LLM Router

## Status

Decision status: Unified format translation and the Router/Gateway boundary are accepted directions; remaining scope is proposed. Implementation status: Under development.

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Open questions are recorded below.

## Context and proposed scope

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

## Agreed direction and open questions

The client API format is independent of the provider. Translate both directions through a unified internal representation and explicitly reject unsupported capabilities. Router selects providers and translates formats; [Gateway](../llm-gateway/adr.md) enforces access, allowances, and usage attribution.

Before implementation, decide which API compatibility standard, providers, and model capabilities ship first, and whether credentials are organization-owned only or members may bring personal credentials.

The 9Router comparison also surfaced routing profiles, credential pools, quota awareness, OAuth refresh, self-hosted policies, compression and prompt presets, client setup helpers, and configuration sync. These remain candidates, not accepted scope.

### Custom model fallback

Decision status: Ordered fallback and observed serving status are accepted for the custom-model design. Implementation status: Penpot/local design work; runtime routing, health detection, and telemetry remain pending. Dragging changes provider priority. Percentages are small read-only measurements of observed serving responsibility, not configurable traffic weights: a first-priority provider that was down throughout the period can show 0%. Down/inactive providers are excluded. The earlier editable-weight interpretation is superseded. Measurement window/aggregation, health recovery, eligible failures, retry/timeout budgets, partial streams, exhaustion, compatibility, and charging remain open. See [Custom model design](custom-model.md).

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
