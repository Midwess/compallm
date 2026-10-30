---
title: LLM Gateway
available: false
---

# LLM Gateway

## Status

Decision status: The Router/Gateway boundary is an accepted direction; detailed controls remain proposed. Implementation status: Under development.

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Open questions are recorded below.

## Context and proposed scope

Give members and applications controlled access to models without exposing upstream provider credentials.

- Issue and revoke Compallm API keys independently of provider API keys.
- Attribute requests to an organization, member or application, model, and provider.
- Control which models, providers, and tools a caller can use.
- Measure input and output tokens, request counts, errors, and estimated cost.
- Configure usage allowances, spending limits, and request rate limits.
- Show usage history and remaining allowances to members and administrators.
- Provide a documented API for external clients and automation, with a clearly stated compatibility scope.

## Decisions and open questions

Gateway enforces access, allowances, and usage attribution; [Router](../llm-router/adr.md) selects providers and translates formats.

- Are allowances based on tokens, money, requests, or a combination, and what happens when a limit is reached?
- How are simultaneous requests, fallback attempts, cancellations, and failed requests charged against limits?
- Which controls and client API capabilities belong in the first release?

## Consequences

Clients use Compallm credentials without receiving upstream secrets. Usage accounting and enforcement must remain consistent across Router retries and fallbacks.

## Behavior examples

### Governed model access
The Gateway should authenticate Compallm API keys, enforce configured access policies, and attribute usage without exposing upstream credentials.

#### Example: Unauthorized model
- **WHEN** a caller requests a model excluded by its access policy
- **THEN** the Gateway rejects the request before contacting a provider

## Related pages

- [API keys](api-keys.md)
- [LLM Router](../llm-router/adr.md)
- [Usage accounting](../usage-administration/accounting.md)
- [Product overview](../overview.md)
