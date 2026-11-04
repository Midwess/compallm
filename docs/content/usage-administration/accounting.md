---
title: Usage accounting
available: false
---

# Usage accounting

## Purpose and status

This page defines records and planned calculations for logical-model usage,
provider attempts, typed measurements, and usage limits. One migration per table
creates the current schema directly, including the four typed provider tables.
Migration execution requires a separate explicit instruction.
Services and public APIs remain unimplemented.

The source diagram is `dbdiagram.dbml`. The diagram and this page must change
together when the design changes.

## Record responsibilities

### Logical-model usage

`model_usage` records one client request and its organization-facing
attribution. It owns:

- its Compallm-generated ID, which identifies that client request;
- organization, member, API-key, and logical-model attribution.

Each client request creates one `model_usage` row. `organization_id` is stored
directly so composite foreign keys can reject a model, member, or API key from
another organization.

The logical model owns the published typed organization-facing prices.
`model_usage` does not copy token totals, provider prices, or monetary amounts.

### Independent provider usage

There are five request/usage tables: `model_usage` and four provider-specific
tables. There is no shared provider-attempt parent. Each provider-specific row owns:

- its own application-generated `id` and a direct parent `model_usage_id`;
- an application-generated `attempt_id` shared by records for one upstream call;
- the organization-scoped `model_router` binding, provider model, and provider;
- an optional request ID returned by the provider;
- the attempt result status;
- an immutable `pricing_snapshot` copied from the provider model when the
  attempt begins, using the typed `Pricing` shape in the Core
  `provider::configuration` module.

Retries and fallback use fresh attempt IDs even for the same route. The optional
provider-issued request ID is for correlation, not identity or deduplication.
The provider model identifies the upstream model and current provider prices,
while the binding records how the system model selected it. The snapshot
preserves exact decimal-string rates, units, and rate conditions.
The row does not duplicate the upstream model name or provider-reported USD
cost. An attempt can end without usable measurements.
When measurements are present, the snapshot supports a reproducible historical
upstream-cost estimate even after the provider model's current prices change.
Reports aggregate the selected attempts' measurements and join through
`model_usage` for member, API-key, and logical-model attribution. Which attempt outcomes
contribute to a report or member charge remains a Gateway policy decision.

### Typed usage records

All four tables reference `(organization_id, model_usage_id)` directly with an
organization-scoped foreign key and cascade when the client-request record is
deleted. Each also has scoped foreign keys to its route, provider model, and
provider. There are no foreign keys between typed usage tables:

| Table | Cardinality per attempt | Measurements |
| --- | --- | --- |
| `provider_token_usage` | Zero or one | Input tokens including cached input, cached-input tokens, output tokens, and optional reasoning-output subset |
| `provider_image_usage` | Zero or more | Nullable provider-reported generated-image count and billing conditions |
| `provider_audio_usage` | Zero or one per input/output direction | Duration in seconds and/or character count |
| `provider_video_usage` | Zero or more | Nullable provider-reported generated-video duration in seconds and billing conditions |

Each row repeats route/provider attribution, status, request ID, and snapshot.
Within an organization, `attempt_id` must identify one client request and one
upstream call, with consistent metadata and immutable pricing across tables.
The application owns this invariant; separate foreign keys cannot enforce it.
Image/video rows use their own primary keys and record provider-reported
measurements and conditions without an application-assigned group number.
There is no uniqueness constraint per media attempt. Deduplication of repeated
provider updates remains unresolved. Audio is unique per attempt and direction;
tokens are unique per attempt.

The tables describe measurements, not mutually exclusive services. A single
image-generation attempt may have token and image rows. Only the applicable
pricing-snapshot variant contributes to upstream cost; do not bill every
reported quantity automatically. Combined fees need an explicit pricing
contract before use.

All measured quantities are nullable, including image count and video duration,
so typed rows can preserve in-progress or failed attempts without usage reports.
Cached-input tokens are a subset of total input, and reasoning-output tokens
are a subset of total output, not additional tokens. Known subsets require
known totals and cannot exceed them. Durations use exact decimal seconds,
not binary floats. All measured
quantities are nonnegative. Missing rows or null quantities mean unknown;
explicit zero means a known zero.

Image/video `conditions` use the non-secret `RateConditions` shape from the Core
`provider::configuration` module: optional resolution, quality, audio, and
service tier. Measurements must not double-count the same provider-reported
usage. A missing condition
is not a wildcard for selecting a priced rate. Ambiguous or unmatched rates
are errors, and required unknown measurements cannot produce a zero estimate.

### Model usage limits

`model_usage_limit` stores one mutable usage-limit record for each
`(organization_id, user_id)` membership:

- `limit_usd` is the member's non-negative configured USD limit;
- `current_usage_usd` is the member's non-negative accumulated USD usage for
  the active accounting period.

The membership foreign key prevents limits for users outside the organization.
Request-level attribution remains in `model_usage` and typed provider usage; the
limit record does not reference an individual request.

## Intended write flow

Create `model_usage`, allocate an `attempt_id`, and snapshot pricing before
dispatch. Insert a row in the table corresponding to the selected billing
measurement, leaving quantities null until reported. Audio records use the
known input/output direction. Image/video rows retain provider-reported
measurements and conditions. Add
other measurement types only when applicable, reusing the attempt ID and snapshot.
Update statuses consistently across the attempt's records, grouping dependent
updates in a short transaction. Provider calls occur outside all transactions.
Request-billed plans need a measurement contract before dispatch; no fifth
provider-specific table or shared parent is introduced.

The Gateway selects which attempts contribute to member usage. Read the needed
records and calculate charges outside a write transaction. Group multiple
dependent updates, such as final measurement writes and a member-usage update,
in the shortest transaction that preserves correctness. Measurement writes do
not independently debit the member's current usage. Atomic charging,
idempotency, concurrent admission, and which outcomes are charged remain
unresolved; the diagram does not implement them. Member charges use the
organization-facing contract, not upstream pricing snapshots.

## Deletion behavior

Deleting a `model_usage` row cascades directly to its four types of usage rows.
It does not reduce `current_usage_usd`; adjustment and reconciliation rules
remain unresolved. Deleting one typed row does not delete correlated rows in
other tables and does not change the member's accumulated charge.

This design has no reversal or adjustment record. Until authorization,
retention, reconciliation, and audit requirements are decided, application APIs
should not offer unrestricted usage deletion.

## Query boundaries

- Member token usage sums `provider_token_usage` through `model_usage` by
  organization and user.
- API-key usage aggregates typed measurements through `model_usage.api_key_id`.
- Logical-model reports group measurements through `model_usage.model_id`.
- Retry and provider reports union typed records and deduplicate by
  `(organization_id, attempt_id)` before grouping by client request, route,
  provider, or status. Conflicting metadata is an integrity error, not a reason
  to silently select one row.
- Limit checks read the member's `model_usage_limit` row.

Aggregate each measurement table per attempt before joining different types;
joining raw image and video rows can multiply quantities. Never sum unlike
units. Reproducible upstream-cost estimates use each attempt's own snapshot.

Daily summaries may be introduced later as rebuildable query accelerators.
They must not become a second authoritative usage source.

## Schema setup

The table migrations target a fresh database. They do not upgrade a previous
migration history, copy old usage, or provide compatibility tables. The four
typed provider tables reference `model_usage` directly from creation. Down
migrations drop tables in reverse dependency order. Migration execution
requires a separate explicit instruction.

## Unresolved rules

The accounting period, reset behavior, limit-management authorization,
over-limit behavior, concurrent admission policy, request status semantics,
retry charging, reconciliation, deletion authorization, retention, and auditing
remain undecided. These rules must be accepted before implementing accounting
services. Request-billed measurements, compound pricing, detailed modality
token breakdowns, and organization-facing media prices also need explicit
contracts before serving those billing plans.

## Related decisions

- [Usage and administration](adr.md)
- [LLM Gateway](../llm-gateway/adr.md)
- [Models and model providers](../model/models-and-providers.md)
- [Organization and membership](../organization/adr.md)
