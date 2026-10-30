---
title: Usage accounting
available: false
---

# Usage accounting

## Purpose and status

This page defines the proposed records and calculations for logical-model
usage, provider attempts, and organization-member amounts. It is a design for
future persistence and services, not implemented behavior or a public API.

The source diagram is `dbdiagram.dbml`. The diagram and this page must change
together when the design changes.

## Record responsibilities

### Logical-model usage

`usage` records the organization-facing result for one client request. It owns:

- the Compallm-generated request ID;
- organization, member, API-key, and logical-model attribution;
- input, cached-input, and output token counts;
- the time the usage record was created.

Token values are non-negative, and cached tokens are a subset of input tokens.
The request ID is unique. `organization_id` is stored directly so composite
foreign keys can reject a model, member, or API key from another organization.

The logical model owns the published input, cached-input, and output prices.
Those prices can differ from the selected provider route's prices. The usage
record does not copy prices or store a monetary amount.

### Provider usage

`provider_usage` records one upstream call. Several rows may share a request ID
when Compallm retries or selects a fallback route. Each row owns:

- the Compallm request ID and its own application-generated ID;
- the organization-scoped model-provider route and provider;
- an optional request ID returned by the provider;
- the attempt result status;
- provider input, cached-input, and output token counts when known.

The route identifies the upstream model and provider prices. The row does not
duplicate the upstream model name or calculate a provider USD cost. Token
columns are nullable because an attempt can end without usable provider usage.
Current route prices can estimate provider cost, but the schema does not retain
the price used at attempt time. It therefore cannot reconstruct exact historical
provider cost after a route price changes.

### Organization-member amounts

`organization_member_amounts` is a signed fixed-point ledger scoped to
`(organization_id, user_id)`. It has two intended forms:

| Row | `usage_id` | `amount` |
| --- | --- | --- |
| Funds added | `NULL` | Positive |
| Logical-model charge | Usage ID | Negative |

One usage ID can appear at most once, preventing duplicate debits for the same
usage row. The composite usage foreign key ensures the debit belongs to the
same organization member as the usage. Zero-value rows are invalid.

The current member amount is derived rather than stored:

```sql
SELECT COALESCE(SUM(amount), 0)
FROM organization_member_amounts
WHERE organization_id = :organization_id
  AND user_id = :user_id;
```

## Intended write flow

After a provider call produces the logical usage needed for charging, one
database transaction should:

1. insert `usage` with its unique request ID;
2. calculate the member charge from the logical model's prices;
3. insert one negative `organization_member_amounts` row linked to that usage;
4. return the current amount calculated from the ledger.

Provider calls must occur outside this transaction. Provider-attempt rows may
be written as attempts progress, but they do not independently debit the
member. The unresolved Gateway policy decides which outcome supplies the
logical usage and whether failed attempts contribute to the charge.

## Deletion behavior

Deleting a `usage` row cascades to its linked member-amount debit. Subsequent
token totals and member amount calculations both exclude that request. Deleting
`provider_usage` only removes upstream attempt evidence.

This design has no reversal or adjustment record. A deletion therefore removes
history rather than preserving a correction trail. Until authorization,
retention, and audit requirements are decided, application APIs should not
offer unrestricted usage deletion.

## Query boundaries

- Member token usage sums `usage` by organization and user.
- API-key usage sums `usage` by `api_key_id`.
- Logical-model reports group `usage` by `model_id`.
- Retry and provider reports group `provider_usage` by request, route, provider,
  and status.
- Current member amount sums `organization_member_amounts.amount`.

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
