---
title: Usage accounting
available: false
---

# Usage accounting

## Purpose and status

This page defines the proposed records and calculations for logical-model
usage, provider attempts, and balances. The migration and generated PostgreSQL
schema implement these records; services and public APIs remain unimplemented.

The source diagram is `dbdiagram.dbml`. The diagram and this page must change
together when the design changes.

## Record responsibilities

### Logical-model usage

`model_usage` records one client request and its organization-facing attribution. It
owns:

- its Compallm-generated ID, which identifies that client request;
- organization, member, API-key, and logical-model attribution.

Each client request creates one `model_usage` row. `organization_id` is stored directly
so composite foreign keys can reject a model, member, or API key from another
organization.

The logical model owns the published input, cached-input, and output prices.
`model_usage` does not copy token totals, provider prices, or monetary amounts.

### Provider usage

`provider_usage` records one upstream call and is the authoritative source of
token usage. Several rows may share one `model_usage_id` when Compallm retries or
selects a fallback binding. Each row owns:

- its own application-generated ID and the parent `model_usage_id`;
- the organization-scoped `model_router` binding, provider model, and provider;
- an optional request ID returned by the provider;
- the attempt result status;
- provider input, cached-input, and output token counts when known.
- snapshots of the provider model's input, output, and cached-input prices when
  the attempt begins.

The provider model identifies the upstream model and current provider prices,
while the binding records how the system model selected it. The three non-negative
price snapshots preserve the rates selected at attempt creation. The row does
not duplicate the upstream model name or provider-reported USD cost. Token
columns are nullable because an attempt can end without usable provider usage.
When token counts are present, the snapshots support a reproducible historical
upstream-cost estimate even after the provider model's current prices change.
Reports sum the selected `provider_usage` rows and join through `model_usage` for
member, API-key, and logical-model attribution. Which attempt outcomes contribute
to a report or member charge remains a Gateway policy decision.

### Balances

`balances` is a signed fixed-point ledger scoped to
`(organization_id, user_id)`. It has two intended forms:

| Row | `model_usage_id` | `amount` |
| --- | --- | --- |
| Funds added | `NULL` | Positive |
| Logical-model charge | Model-usage ID | Negative |

One model-usage ID can appear at most once, preventing duplicate debits for the
same model-usage row. The composite model-usage foreign key ensures the debit
belongs to the same organization member as the model usage. Zero-value rows are
invalid.

The current member balance is derived rather than stored:

```sql
SELECT COALESCE(SUM(amount), 0)
FROM balances
WHERE organization_id = :organization_id
  AND user_id = :user_id;
```

## Intended write flow

After the Gateway selects the provider attempts that contribute to a member
charge, one database transaction should:

1. insert `model_usage`, whose ID identifies the client request;
2. calculate the member charge from the logical model's prices and selected
   `provider_usage` token totals;
3. insert one linked negative `balances` row with the calculated charge;
4. return the current balance calculated from the ledger.

Provider calls must occur outside this transaction. Provider-attempt rows may
be written as attempts progress, but they do not independently debit the
member. The unresolved Gateway policy decides which outcome supplies the
logical usage and whether failed attempts contribute to the charge.

## Deletion behavior

Deleting a `model_usage` row cascades to its linked balance debit and associated
`provider_usage` attempts. Subsequent token totals and member-balance
calculations both exclude that request. Deleting `provider_usage` only removes
upstream attempt evidence.

This design has no reversal or adjustment record. A deletion therefore removes
history rather than preserving a correction trail. Until authorization,
retention, and audit requirements are decided, application APIs should not
offer unrestricted usage deletion.

## Query boundaries

- Member token usage sums `provider_usage` after joining `model_usage` by organization
  and user.
- API-key usage sums `provider_usage` after joining `model_usage` by `api_key_id`.
- Logical-model reports group `provider_usage` through `model_usage.model_id`.
- Retry and provider reports group `provider_usage` by `model_usage_id`, route,
  provider, and status.
- Current member balance sums `balances.amount`.

Daily summaries may be introduced later as rebuildable query accelerators.
They must not become a second authoritative balance or usage source.

## Unresolved rules

The amount unit, funding workflow, insufficient-amount behavior, concurrent
admission policy, request status semantics, retry charging, deletion
authorization, retention, and auditing remain undecided. These rules must be
accepted before implementing migrations or accounting services.

## Related decisions

- [Usage and administration](adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
- [Models and model providers](../model/models-and-providers.md)
- [Organization and membership](../organization/adr.md)
