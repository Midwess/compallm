# LLM Router

## Planning status

Draft scope for discussion, not a claim of implementation. Detailed policies and first-release scope require review before implementation.

## Feature scope

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

## Acceptance requirements

See [spec.md](spec.md) for proposed requirements and scenarios. This feature overview remains part of the proposal; preserve enduring scope in baseline documentation when the feature is implemented and archived.
