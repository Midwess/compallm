---
title: "Provider configuration"
available: false
---

# Provider configuration

Provider configuration defines a connection name, base URL, endpoint mode, API protocol, credentials, and upstream models. Each model has a required identifier and optional context limit and supported thinking levels. Runtime integration and persistence remain planned.

## Catalog defaults

The LiteLLM model cost map supplies provider slugs, model modes, context limits, capabilities, and prices. Obtain base URLs from verified provider documentation; the cost map generally does not supply them. Use `max_input_tokens` for input context. Only configure thinking levels explicitly supported by the model; a reasoning-capability flag does not establish particular levels.

Catalog lookup matches provider names and model identifiers. Provider, mode, and capability constraints use known metadata. Unknown metadata stays unspecified. A preset supplies editable defaults but never supplies an API key. Custom configuration starts without inferred provider defaults. Changing a preset must preserve explicit user edits.

## Endpoint construction

Keep host/version in the base URL and the relative route in the endpoint mode. `https://api.openai.com/v1` plus `/chat/completions` produces `https://api.openai.com/v1/chat/completions`. Avoid duplicated version segments.

Provider identity, API protocol, and endpoint mode are separate. Modes include Chat Completions, Responses, Messages, Video, and Realtime where supported. RPC/model-specific paths and WebSockets require protocol-aware handling; not every mode is an interchangeable HTTP suffix.

## Pricing and unresolved requirements

Token prices use USD per million tokens; multiply per-token cost-map values by one million. Cached-token price means cache-read price; cache writes and tiers are distinct. Image, video, and audio retain their actual billing units. Unset automatic pricing differs from an explicit zero.

Catalog refresh/pinning, retired-model handling, supported adapters, duplicate models, pricing precedence, and connection-test behavior remain unresolved. Connection tests may incur usage; define their request and charging rules before implementation.

See the [Router architecture](../llm-router/adr.md).
