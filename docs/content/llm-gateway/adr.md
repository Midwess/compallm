---
title: LLM Gateway
available: false
---

# LLM Gateway

## Status

Implementation: Under development.

The following solution is planned; unresolved requirements are listed below.

## Solution

Give members and applications controlled access to models without exposing upstream provider credentials.

- Issue and revoke Compallm API keys independently of provider API keys.
- Attribute requests to an organization, member or application, model, and provider.
- Control which models, providers, and tools a caller can use.
- Measure input and output tokens, request counts, latency, errors, and estimated cost.
- Configure usage allowances, spending limits, and request rate limits.
- Show usage history and remaining allowances to members and administrators.
- Provide a documented API for external clients and automation, with a clearly stated compatibility scope.

## Responsibilities and unresolved requirements

Gateway enforces access, allowances, and usage attribution; [Router](../llm-router/adr.md) selects providers and translates formats.

[Usage and spend limits](member-limits.md) define workspace budgets, monthly USD member defaults, and individual overrides. Persistence, accounting, authorization, and enforcement remain planned.

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

- [LLM Router](../llm-router/adr.md)
- [Product overview](../overview.md)
