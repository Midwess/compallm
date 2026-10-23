# LLM Router

## ADDED Requirements

### Requirement: Client-selected format translation
The Router SHALL translate supported client requests through a unified internal format to the selected provider, and return responses in the client's requested format.

#### Scenario: Different provider protocol
- **WHEN** a client sends a supported OpenAI-compatible request targeting a supported Anthropic or Gemini model
- **THEN** the Router translates the request and returns the response using the client's protocol, including streamed events when requested

### Requirement: Explicit translation limitations
The Router SHALL explicitly report unsupported requested features or options rather than silently discard them during conversion.

#### Scenario: Unsupported capability
- **WHEN** a request requires a capability unavailable from the selected model or adapter
- **THEN** the Router returns an explicit compatibility error rather than silently omitting the capability
