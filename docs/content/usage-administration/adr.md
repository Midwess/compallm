---
title: Usage and administration
available: false
---

# Usage and administration

The PostgreSQL schema implements the records below. Reporting, charging,
retention, administrative workflows, and public APIs remain planned.

## Solution

Separate logical-model attribution, provider attempts, and member usage limits:

- `model_usage` records one client request and its organization, member, API
  key, and logical-model attribution;
- `provider_usage` records each upstream provider attempt and is the
  authoritative token-usage source;
- `model_usage_limit` stores the member's USD limit and current USD usage in an
  organization.

```text
organization member
  ├─ API key ─> model_usage ─> provider_usage attempt(s)
  └─ model_usage_limit ─> limit_usd and current_usage_usd
```

Logical-model prices belong to `models`. Provider-model prices belong to
`provider_models`; `model_router` identifies the binding used for an attempt.
Each provider attempt snapshots its provider model's three rates so historical
upstream-cost estimates remain reproducible after current prices change.
`model_usage` does not duplicate tokens, provider prices, or money, and
`model_usage_limit` does not duplicate request-level token facts.

## Integrity and lifecycle

Every record retains `organization_id`. Composite foreign keys ensure the
member, API key, logical model, router binding, provider model, and provider
belong to that organization.

Each client request creates one `model_usage` row. Multiple `provider_usage`
rows may reference it when routing retries or falls back. The provider-issued
request ID is optional and supports correlation with provider logs.

At most one usage-limit row exists for an organization member. Both monetary
values are non-negative fixed-point USD. Usage records do not own or delete the
member's limit; deleting model usage cascades only to its provider attempts.

Model usage does not store prompts, responses, API-key secrets, provider
credentials, provider-returned USD cost, or request timing.

## Consequences

Logical-model reporting remains independent of route selection and fallback.
Token reports sum selected provider attempts through their parent model usage.
Admission compares `current_usage_usd` with `limit_usd`. A transaction must
group the accepted usage write and current-usage update when those operations
are implemented.

Hard deletion removes accounting evidence. Do not expose unrestricted deletion
until authorization, retention, and audit requirements are decided.

## Open questions

- When does `current_usage_usd` reset, and who may change `limit_usd`?
- Which provider outcomes contribute to reports and member charges?
- Who may adjust current usage or delete usage, and what audit record is required?
- Which reports, limits, notifications, and retention periods ship first?

## Related pages

- [Usage accounting](accounting.md)
- [LLM Gateway](../llm-gateway/adr.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
