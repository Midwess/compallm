---
title: Usage and administration
available: false
---

# Usage and administration

The independent typed-usage schema below follows `dbdiagram.dbml`. One migration
per table creates the typed provider tables directly. They target a fresh
database, without historical data conversion. Migration execution is separate;
reporting, charging, retention, administrative workflows, and public APIs remain
planned.

## Solution

Separate logical-model attribution, provider attempts, and member usage limits:

- `model_usage` records one client request and its organization, member, API
  key, and logical-model attribution;
- `provider_token_usage`, `provider_image_usage`, `provider_audio_usage`, and
  `provider_video_usage` reference `model_usage` directly and each own attempt
  metadata, immutable applicable rates, and authoritative measured quantities;
- `model_usage_limit` stores the member's USD limit and current USD usage in an
  organization.

```text
organization member
  ├─ API key ─> model_usage ─> token / image / audio / video usage
  └─ model_usage_limit ─> limit_usd and current_usage_usd
```

Logical-model prices belong to `models`. Provider-model prices belong to
`provider_models`; `model_router` identifies the binding used for an attempt.
There is no shared provider-attempt table. These five request/usage tables are
separate from the member's `model_usage_limit` record.
Each typed usage row snapshots applicable provider-model rates in dedicated
nullable NUMERIC(20,8) columns, so historical upstream-cost estimates remain
reproducible after current prices change. Token rows store three token rates;
image rows store USD per image; audio rows store USD per second or per million
characters; video rows store USD per second. These are configured rates, not
provider-returned charges. Coexisting measurement rows do not automatically add
charges: null rates mean the measurement is not priced by that billing plan,
while explicit zero means a configured free rate.
`model_usage` does not duplicate measurements, provider prices, or money, and
`model_usage_limit` does not duplicate request-level token facts.

## Integrity and lifecycle

Every record retains `organization_id`. Composite foreign keys ensure the
member, API key, logical model, router binding, provider model, and provider
belong to that organization.

Each client request creates one `model_usage` row. Each upstream call gets an
application-generated `attempt_id`; retries and fallback get fresh IDs. The ID
correlates typed records without referencing a parent table. Token totals have
at most one row per attempt; audio has at most one per direction. Image/video
rows store provider-reported measurements and conditions, identified by their
own row IDs. Multiple rows may reference one attempt; media-update deduplication
still needs a provider-specific contract.

Before dispatch, record an attempt in the table for its billing measurement,
even if quantities are unknown. All quantity columns permit null, preserving
failed attempts with no provider usage. Unknown is not zero. Coexisting typed
rows repeat metadata; the application must keep attribution consistent and
populate rates only for the selected billing unit. Token rates are all present
or all null; audio permits at most one rate. Cross-table consistency is not
enforced by a parent foreign key.

At most one usage-limit row exists for an organization member. Both monetary
values are non-negative fixed-point USD. Usage records do not own or delete the
member's limit; deleting model usage cascades directly to all its typed usage rows.

Model usage does not store prompts, responses, API-key secrets, provider
credentials, provider-returned USD cost, or request timing.

## Consequences

Logical-model reporting remains independent of route selection and fallback.
Reports aggregate selected attempts' typed measurements through their parent
model usage, aggregating each detail table before combining results to avoid
multiplying totals across one-to-many joins.
Attempt reports union the typed tables and deduplicate by organization and
`attempt_id`; a multimodal attempt must not count as multiple attempts.
Admission compares `current_usage_usd` with `limit_usd`. A transaction must
group the accepted usage write and current-usage update when those operations
are implemented.

Hard deletion removes accounting evidence. Do not expose unrestricted deletion
until authorization, retention, and audit requirements are decided.

Schema constraints reject invalid cached-token subsets rather than inventing
missing totals. Down migrations drop tables in reverse dependency order.

## Open questions

- When does `current_usage_usd` reset, and who may change `limit_usd`?
- Which provider outcomes contribute to reports and member charges?
- Who may adjust current usage or delete usage, and what audit record is required?
- Which reports, limits, notifications, and retention periods ship first?
- How are compound fees, modality-specific token rates, request-billed
  measurements, rounding, and minimum charges represented? Unsupported billing
  contracts must not be served or inferred free.
- What is the organization-facing media pricing contract? Upstream provider
  rates do not define member charges.

## Related pages

- [Usage accounting](accounting.md)
- [LLM Gateway](../llm-gateway/adr.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
