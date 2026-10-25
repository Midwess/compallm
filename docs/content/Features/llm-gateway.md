---
title: LLM Gateway
available: false
---

# LLM Gateway

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

Give members and applications controlled access to models without exposing upstream provider credentials.

- Issue and revoke Compallm API keys independently of provider API keys.
- Attribute requests to an organization, member or application, model, and provider.
- Control which models, providers, and tools a caller can use.
- Measure input and output tokens, request counts, latency, errors, and estimated cost.
- Configure usage allowances, spending limits, and request rate limits.
- Show usage history and remaining allowances to members and administrators.
- Provide a documented API for external clients and automation, with a clearly stated compatibility scope.

## Behavior examples

### Governed model access
The Gateway should authenticate Compallm API keys, enforce configured access policies, and attribute usage without exposing upstream credentials.

#### Example: Unauthorized model
- **WHEN** a caller requests a model excluded by its access policy
- **THEN** the Gateway rejects the request before contacting a provider

## Related pages

- [LLM Router](llm-router.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
