---
title: "Provider creation API contract"
available: false
---

# Provider creation API contract

## Scope and implementation

The planned endpoints create organization-owned connections and optionally test
unsaved configuration. Neither endpoint nor `ProviderIntegrationService` is
implemented. Storybook previews use consumer controllers without these APIs.
Rust/OpenAPI must generate the consumer API guide and client when implemented.

Provider identity supplies editable connection defaults in the frontend. Each
configured text-model draft owns its operation, endpoint path, limits, effort
mappings, and explicit prices. Creating provider models does not create
organization-facing logical models or router bindings.

Provider-model persistence already uses direct service/protocol fields and
typed JSONB configuration. Each provider model requires a complete connection,
with its upstream base URL only in `connection.baseUrl`; provider connections
have no enabled flag. The request fields below describe the planned frontend
submission contract, not the persistence representation. Server integration
must define their translation to Core service/protocol, connection, limits,
thinking, and pricing types before implementing these endpoints. See
[provider-model storage](../model/models-and-providers.md#provider-model-storage).

Both operations require authentication and owner/admin membership in the
current organization. Organization scope comes from the verified session,
never a request-body organization ID. Cookie requests use the existing
same-origin policy. JSON uses camelCase; reject unknown fields.

## Batch creation

`POST /api/providers`, operation ID `createProviders`:

```json
{
  "providers": [{
    "name": "OpenAI production",
    "baseUrl": "https://api.openai.com/v1",
    "apiKey": "synthetic-write-only-key",
    "models": [{
      "name": "gpt-4.1",
      "protocol": "openai_chat_completions",
      "path": "/chat/completions",
      "maxContextTokens": 1048576,
      "effortMapping": { "low": "low", "medium": "medium" },
      "catalogRef": { "provider": "openai", "model": "gpt-4.1" },
      "pricing": {
        "inputUsdPerMillionTokens": "2.00",
        "outputUsdPerMillionTokens": "8.00",
        "cachedUsdPerMillionTokens": "0.50"
      }
    }]
  }]
}
```

A provider requires a trimmed nonblank name, HTTP(S) base URL, write-only
credential, and at least one enabled model. Reject credentials, query strings,
and fragments in base URLs. Provider names are organization-scoped; proposed
comparison preserves case after trimming. PostgreSQL uniqueness remains the
concurrency guard when simultaneous requests pass preliminary name checks.

Discovery lists provider identities before model selection. Models are scoped
to the selected provider catalog. Submit only explicitly enabled definitions;
merely viewing a catalog entry does not configure it. A preset and a custom
connection use the same creation contract. Explicit edits take precedence over
later catalog changes, including edits to model identifiers and prices.

## Model configuration

| Field | Contract |
| --- | --- |
| `name` | Required nonblank upstream identifier; trim whitespace, preserve case, unique within the connection. |
| `protocol` | Required `openai_chat_completions`, `openai_responses`, or `anthropic_messages`. |
| `path` | Required editable endpoint path with no host, scheme, query, fragment, or traversal segments. |
| `maxContextTokens`, `maxOutputTokens` | Optional positive integer limits; null/missing means unspecified. |
| `effortMapping` | Optional object mapping `low`, `medium`, `high`, `xhigh`, and `max` to nonblank model-specific strings. Missing keys are unmapped; `None` is an editing hint, never a submitted mapping. |
| `catalogRef` | Optional exact catalog provider/model identity, independent of an edited upstream name. |
| `pricing` | Three required nonnegative decimal-string token prices. |

The model-creation operation supplies a protocol-specific default path:
`/chat/completions`, `/responses`, or `/messages`. Append the submitted path to
the connection base URL, normalizing the joining slash while preserving the
base path prefix. `https://api.openai.com/v1` plus `/responses` becomes
`https://api.openai.com/v1/responses`. A model cannot replace the connection's
host by submitting an absolute or protocol-relative URL. The selected operation
still determines the request/response adapter; editing its route does not
change its wire format. Media and arbitrary protocols are outside this flow.

Prices are USD per million input, output, and cache-read tokens. All three
values must be present, nonblank, nonnegative decimal strings with at most
eight decimal places. Explicit `"0"` is free; missing values never imply zero
or trigger catalog lookup. Do not accept currency symbols, separators, `Auto`,
or binary floating-point JSON amounts. Cache writes and non-token billing are
separate contracts. Each submitted model stores its own prices; changing one
model must not modify another model or historical usage snapshots.

Custom header values may contain secrets. The library keeps them in memory and
preserves entry order while editing; they must not be copied into URL state or
logs. Header names are case-insensitive and unique and values cannot contain
line breaks. The API representation, secret-header protection, and persisted
ordering must be settled before server integration. Avatar upload and storage
also require an explicit server contract; selecting an image in the library
only provides a local preview and consumer callback.

## Validation and persistence

Core owns shared invariants through Validify. HTTP normalization reuses those
rules. The planned service coordinates authorization, name checks, credential
protection, ID generation, and persistence. PostgreSQL owns database models,
queries, checked conversions, and concurrency guards.

The proposed batch is atomic: reject every item if any connection or model
fails. Reads and credential protection precede the write transaction. Use one
short transaction to group provider and model inserts, ending it immediately
after the writes. Creation makes no upstream calls and never requires a
successful connection test. No database transaction spans email or HTTP calls.

`201 Created` returns providers in request order with decimal-string IDs, safe
connection configuration and model configuration/prices, including model enabled
states. Never return credentials or credential-derived values. Submitted models
default to enabled; connections have no enabled flag. Reads, updates, and cloning retain the write-only
credential boundary described in [management](management.md).

| Status | Meaning |
| --- | --- |
| `400` | Malformed or unsupported configuration, duplicate model names, or invalid prices; nothing saved. |
| `401` | Missing/invalid Compallm authentication. |
| `403` | Organization role or cookie-origin checks deny access. |
| `409` | Provider-name conflict within the batch, stored data, or concurrent insert; nothing saved. |
| `500` | Internal failure with no partial batch commit. |

Errors retain `{ "message": "..." }`; an optional proposed `issues` array has
JSON Pointer `path`, `code`, and safe `message`. Never echo credentials or raw
upstream responses. If a conflict cannot be safely assigned to one item, use an
operation-level message instead of inventing a field issue.

## Test unsaved configuration

`POST /api/providers/test-connection`, operation ID `testProviderConnection`,
accepts one complete provider definition without the batch wrapper. It does not
save providers, reserve names, create model routes, or require a stored ID.
Provider-name uniqueness is not checked. Authorization and configuration/price
validation precede the bounded upstream probe.

A completed test returns `200` with `success` and per-model results in submitted
order. Success requires every submitted model to pass. Failure codes include
`invalid_credentials`, `connection_failed`, `timeout`, `model_unavailable`, and
`rate_limited`. Upstream authentication failure is a test outcome rather than
Compallm HTTP `401`. Malformed requests and Compallm authorization failures use
the normal error envelope. A test result applies only to that configuration at
that moment and does not guarantee later availability.

## Unresolved server requirements

- Translating frontend operation/path, limits, effort-name mappings, and prices
  into the existing typed model configuration, including complete connections.
  Catalog identity storage and effort-name mapping persistence remain unresolved.
- Header representation, secret storage, and avatar upload policy.
- Credential encryption, rotation, and provider adapter coverage.
- Batch/model limits, request sizes, exact probes, concurrency budgets,
  cancellation, admission policy, and probe usage charges.
- Acceptance of proposed atomic batching, case-sensitive name comparison,
  decimal-string IDs, and structured error issues.
