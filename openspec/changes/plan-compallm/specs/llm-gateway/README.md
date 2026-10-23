# LLM Gateway

## Planning status

Draft scope for discussion, not a claim of implementation. Detailed policies and first-release scope require review before implementation.

## Feature scope

Give members and applications controlled access to models without exposing upstream provider credentials.

- Issue and revoke Compallm API keys independently of provider API keys.
- Attribute requests to an organization, member or application, model, and provider.
- Control which models, providers, and tools a caller can use.
- Measure input and output tokens, request counts, latency, errors, and estimated cost.
- Configure usage allowances, spending limits, and request rate limits.
- Show usage history and remaining allowances to members and administrators.
- Provide a documented API for external clients and automation, with a clearly stated compatibility scope.

## Acceptance requirements

See [spec.md](spec.md) for proposed requirements and scenarios. This feature overview remains part of the proposal; preserve enduring scope in baseline documentation when the feature is implemented and archived.
