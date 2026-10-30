---
title: Usage and administration
available: false
---

# Usage and administration

## Status

Decision status: The separation of logical-model usage, provider-attempt usage,
and organization-member amounts is proposed. Reporting, charging, retention,
and administrative permissions remain unresolved. Implementation status: The
diagram is a design draft; application persistence and workflows are not
implemented.

## Context and proposed decision

Members need their own usage and remaining amount. Administrators need the same
information across an organization, with breakdowns by member, API key, logical
model, provider route, and time period. One record cannot represent all three
concerns without confusing organization-facing model usage with upstream
provider attempts or duplicating a mutable balance.

Use three records with distinct responsibilities:

- `usage` records one client request's logical-model token usage;
- `provider_usage` records each upstream provider attempt associated with that
  request;
- `organization_member_amounts` is a signed ledger whose sum is the member's
  current amount within one organization.

```text
organization member
  ├─ API key ─> usage ─> one member-amount debit
  │              └─ request_id ─> provider_usage attempt(s)
  └─ organization_member_amounts ─> current amount = SUM(amount)
```

Logical-model prices belong to `models`. Provider-route prices belong to
`model_providers`. Provider responses supply token counts rather than a USD
cost. Usage records therefore retain token facts, while the member ledger
retains the calculated signed amount.

## Integrity and lifecycle

Every record retains `organization_id` even when another reference can imply
it. Composite foreign keys enforce that the member, API key, model, route, and
provider belong to the same organization. This also supports organization
reports without deriving tenant scope through mutable configuration.

`usage.request_id` is unique, so repeated delivery cannot create two logical
usage records for one client request. Several `provider_usage` rows may share a
request ID when routing retries or falls back. The provider-issued request ID
is optional and supports correlation with provider logs.

One usage-linked member-amount row is allowed. Positive standalone amount rows
add funds; a negative row linked to usage subtracts the calculated charge.
There is no separate balance column. Deleting usage cascades to its linked
member-amount row, so both the reported usage and derived current amount change
together. Deleting provider usage does not change either value.

Usage records do not store prompts, responses, API-key secrets, provider
credentials, provider-returned USD cost, or request timing.

## Consequences

Logical-model reporting remains independent of route selection and fallback.
Provider attempts can be inspected without silently changing a member's
amount. Balance checks must sum the signed member ledger, and token reports
must sum logical usage. A transaction must write a usage record and its debit
together so a successful charge cannot leave only one side persisted.

Hard deletion is simple and internally consistent but removes accounting
evidence. Authorization and retention rules must therefore be decided before
deletion is exposed through an API.

## Open questions

- What unit does `organization_member_amounts.amount` represent, and how is it
  displayed and funded?
- Which provider outcomes contribute to logical usage and member charges?
- Do retries remain separate `provider_usage` rows, or will the schema adopt a
  merged per-attempt usage record before implementation?
- Who may add funds or delete usage, and what audit record is required?
- Which reports, limits, notifications, and retention periods ship first?

## Related pages

- [Usage accounting](accounting.md)
- [LLM Gateway](../llm-gateway/adr.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
- [Data and privacy](../data-privacy/adr.md)
