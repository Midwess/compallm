---
title: Usage and administration
available: false
---

# Usage and administration

The PostgreSQL schema implements the records below. Reporting, charging,
retention, administrative workflows, and public APIs remain planned.

## Solution

Separate logical-model attribution, provider attempts, and member balances:

- `model_usage` records one client request and its organization, member, API
  key, and logical-model attribution;
- `provider_usage` records each upstream provider attempt and is the
  authoritative token-usage source;
- `balances` is a signed ledger whose sum is the member's current balance in an
  organization.

```text
organization member
  ├─ API key ─> model_usage ─> one balance debit
  │              └─ model_usage_id ─> provider_usage attempt(s)
  └─ balances ─> current balance = SUM(amount)
```

Logical-model prices belong to `models`. Provider-model prices belong to
`provider_models`; `model_router` identifies the binding used for an attempt.
Each provider attempt snapshots its provider model's three rates so historical
upstream-cost estimates remain reproducible after current prices change.
`model_usage` does not duplicate tokens, provider prices, or money, and
`balances` does not duplicate token facts.

## Integrity and lifecycle

Every record retains `organization_id`. Composite foreign keys ensure the
member, API key, logical model, router binding, provider model, and provider
belong to that organization.

Each client request creates one `model_usage` row. Multiple `provider_usage`
rows may reference it when routing retries or falls back. The provider-issued
request ID is optional and supports correlation with provider logs.

One balance row may link to a model-usage row. Positive standalone rows add
funds; a negative linked row subtracts the calculated charge. Current balance
is derived by summing the ledger. Deleting model usage cascades to its linked
balance row and provider attempts; deleting a provider attempt does not change
the member balance.

Model usage does not store prompts, responses, API-key secrets, provider
credentials, provider-returned USD cost, or request timing.

## Consequences

Logical-model reporting remains independent of route selection and fallback.
Token reports sum selected provider attempts through their parent model usage,
while balance checks sum the member ledger. A transaction must group creation
of model usage and its balance debit so neither can persist alone.

Hard deletion removes accounting evidence. Do not expose unrestricted deletion
until authorization, retention, and audit requirements are decided.

## Open questions

- What unit does `balances.amount` represent, and how is it funded and shown?
- Which provider outcomes contribute to reports and member charges?
- Who may add funds or delete usage, and what audit record is required?
- Which reports, limits, notifications, and retention periods ship first?

## Related pages

- [Usage accounting](accounting.md)
- [LLM Gateway](../llm-gateway/adr.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
