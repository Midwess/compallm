---
title: "Provider creation API contract"
available: false
---

# Provider creation API contract

## Status and scope

This is the planned contract for batch provider creation and optional testing
of unsaved connections. Neither endpoint nor `ProviderIntegrationService` is
implemented. The schema below defines application behavior; it is not a
generated public API reference. Once implemented, Rust/OpenAPI descriptions
must generate the public guide and TypeScript client.

One provider represents one connection and contains one or more upstream
models. One creation request can contain multiple independent connections.
Creating provider models does not create organization-facing logical models
or router bindings.

Discovery selects a provider identity first, then loads that provider's known
model definitions. Callers explicitly enable catalog models or add custom
models. The creation `models` array contains only those enabled definitions;
unselected catalog entries are not submitted or persisted. Catalog discovery
does not imply that an upstream account can access every listed model.

Both operations require user authentication and owner/admin access to the
current organization. Organization scope comes from the authenticated session,
not a request-body organization ID. Cookie requests follow the existing
same-origin policy; Bearer requests use the existing authentication middleware.
JSON property names use camelCase, and unknown fields are rejected.

## Batch creation

`POST /api/providers`, operation ID `createProviders`.

```json
{
  "providers": [
    {
      "name": "OpenAI production",
      "baseUrl": "https://api.openai.com/v1",
      "apiProtocol": "openai_compatible",
      "endpointMode": "chat_completions",
      "apiKey": "synthetic-openai-key",
      "models": [
        {
          "name": "gpt-4.1",
          "maxContextTokens": 1047576,
          "supportedThinkingLevels": null,
          "catalogRef": {
            "provider": "openai",
            "model": "gpt-4.1"
          },
          "pricing": {
            "inputUsdPerMillionTokens": "2.00",
            "outputUsdPerMillionTokens": "8.00",
            "cachedUsdPerMillionTokens": "0.50"
          }
        },
        {
          "name": "gpt-4.1-mini",
          "catalogRef": {
            "provider": "openai",
            "model": "gpt-4.1-mini"
          },
          "pricing": {
            "inputUsdPerMillionTokens": "0.40",
            "outputUsdPerMillionTokens": "1.60",
            "cachedUsdPerMillionTokens": "0.10"
          }
        }
      ]
    },
    {
      "name": "Internal inference",
      "baseUrl": "https://inference.example.com/v1",
      "apiProtocol": "openai_compatible",
      "endpointMode": "chat_completions",
      "apiKey": "synthetic-internal-key",
      "models": [
        {
          "name": "support-model",
          "maxContextTokens": 128000,
          "supportedThinkingLevels": [
            "low",
            "medium",
            "high"
          ],
          "pricing": {
            "inputUsdPerMillionTokens": "0",
            "outputUsdPerMillionTokens": "0",
            "cachedUsdPerMillionTokens": "0"
          }
        }
      ]
    }
  ]
}
```

### Provider definition

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `name` | string | Yes | Organization-scoped connection name; trim surrounding whitespace and reject blank values. |
| `baseUrl` | string | Yes | Absolute HTTP(S) base URL containing host and optional API version; no credential, query, or fragment. |
| `apiProtocol` | enum | Yes | `openai_compatible` or `anthropic`; describes the wire format independently of provider identity. |
| `endpointMode` | enum | Yes | `chat_completions`, `responses`, `messages`, `video`, or `realtime`; only supported protocol/mode combinations may be accepted. |
| `apiKey` | string | Yes | Nonempty write-only credential. Preserve its value; never return it or log the request body. |
| `models` | model array | Yes | Enabled catalog/custom model definitions only; at least one, with distinct names within this connection. Unselected catalog models are omitted. |

Provider identity, protocol, and endpoint mode are separate. A catalog provider
slug is not a protocol. Presets and custom connections submit the same final
configuration; no separate preset-creation operation is needed. The existing
database `openai` protocol variant is not a second public spelling for
`openai_compatible`; any distinction required for it remains to be specified.

Endpoint routes are derived from the selected mode, not supplied as arbitrary
URLs. For example, the base URL `https://api.openai.com/v1` and
`chat_completions` produce `https://api.openai.com/v1/chat/completions`.
Do not duplicate the API-version segment. Realtime and model-specific
protocols need their own endpoint construction; they cannot be handled by
blindly appending an HTTP suffix.

### Model definition

| Field | Type | Required | Meaning |
| --- | --- | --- | --- |
| `name` | string | Yes | Upstream model identifier sent to this connection; trim whitespace and reject blank values. Preserve case. |
| `maxContextTokens` | positive integer or null | No | Configured input-context token limit; missing/null means unspecified. |
| `supportedThinkingLevels` | enum array or null | No | Distinct values from `low`, `medium`, `high`, `xhigh`, `max`. Missing/null means unspecified; an empty array explicitly declares none. |
| `catalogRef` | object or null | No | Exact catalog identity: required string fields `provider` and `model`. Missing/null means custom configuration with no catalog source. |
| `pricing` | pricing object | Yes | All three explicit model rates are required; neither the object nor any rate may be null. |

Catalog selection supplies defaults; explicit model fields are the submitted
configuration and are not overwritten during creation. Unknown metadata must
stay unspecified. A model's general reasoning capability does not imply
support for particular thinking levels. Catalog identity remains independent
of the upstream name so explicitly edited identifiers are not silently used
to infer another catalog entry.

### Pricing rules

Each model's required `pricing` object contains three required decimal-string
fields: `inputUsdPerMillionTokens`, `outputUsdPerMillionTokens`, and
`cachedUsdPerMillionTokens`. Values are non-negative USD per million tokens;
cached means cache-read tokens. Decimal strings preserve exact monetary input
across JSON clients without specifying a new Rust price representation.
The current storage precision is eight decimal places. Missing, null, blank,
negative, or over-precision prices are invalid. Do not accept `Auto`, currency
symbols, or formatted separators as monetary values. `"0"` is a valid explicit
free rate and must never be inferred from an empty value.

Catalog presets may prefill known rates as editable decimal values. Every
submitted model must still contain all three explicit prices; the server does
not fill missing prices from the catalog. Custom models follow the same rule.
For example, `gpt-4.1` can have rates `"2.00"`, `"8.00"`, and `"0.50"`, while
`gpt-4.1-mini` has `"0.40"`, `"1.60"`, and `"0.10"`. Changing one model's rate
does not affect other models. Provider-level pricing is not accepted. Store
the submitted prices on each provider model so later catalog changes do not
silently change configured rates.

Non-token billing rates cannot be represented as token prices. Video and
realtime configuration must not reinterpret audio, image, or video units as
USD per million tokens; supported billing coverage remains an open question.

### Validation, conflicts, and persistence

`providers` must be nonempty. The proposed batch semantics are atomic: reject
the whole request if any connection or model fails validation
or persistence. No partial-success response is returned.

Check provider names against both the request and existing records in the
current organization. Proposed comparison preserves case after trimming,
matching the existing database text uniqueness: `Production` and `production`
are distinct. Duplicate model names within one provider are invalid; the same
upstream model name on another provider is allowed.

Core owns shared domain validation using Validify. HTTP normalization reuses
those rules. `ProviderIntegrationService` coordinates validation, name checks,
credential protection and ID generation. PostgreSQL owns
queries, models, conversions, and uniqueness guards. The current repository
requires additional scoped lookup and provider-model write operations.

Perform reads before the write transaction. Protect
credentials before persistence. Use a short transaction only to group the
provider and provider-model inserts; roll it back on any error and end it as
soon as those writes complete. Database unique constraints remain authoritative
when concurrent requests pass the preliminary name checks. Creation never
calls upstream provider APIs, and never requires a test result or test token.

### Responses

`201 Created` returns `{ "providers": [...] }` in request order. Each result
contains `id`, `name`, `baseUrl`, `apiProtocol`, `endpointMode`, `enabled`, and
`models`. Each model contains its `id`, `name`, optional context/thinking
metadata, and submitted `pricing` with three non-null decimal strings using
the same rate property names. New connections and models default to enabled.
IDs in this planned contract are decimal strings to preserve full database
ID precision in JavaScript. No credential or credential-derived value is
included. The ordering makes a separate client correlation ID unnecessary.

| Status | Meaning |
| --- | --- |
| `400` | Malformed JSON, invalid fields, duplicate model names, unsupported combinations, or missing/invalid prices; nothing saved. |
| `401` | Missing or invalid user authentication. |
| `403` | Caller cannot manage the current organization, or cookie-origin checks fail. |
| `409` | Provider-name conflict, including duplicates in the batch or a concurrent unique-constraint violation; nothing saved. |
| `500` | Internal failure; no partial batch commit. |

Errors retain the existing `{ "message": "..." }` shape, with a proposed
optional `issues` array for this API. Each issue has `path`, `code`, and
`message`; paths use JSON Pointer notation such as `/providers/1/name` or
`/providers/0/models/1/catalogRef`. A code such as `name_conflict` or
`price_required` supports client handling. Issues never echo credentials or
raw upstream content. If a concurrent conflict cannot be attributed safely to
a specific item, return the operation-level message without fabricated issues.
Numeric batch/model limits, field lengths, and request-body limits still need
to be selected before implementation.

## Test unsaved connection

`POST /api/providers/test-connection`, operation ID `testProviderConnection`.

The body is one provider definition from the `providers` array above, without
the batch wrapper. It accepts the full unsaved configuration and credential;
no saved ID or earlier create request is needed. Unknown fields remain invalid.
This endpoint is planned only and must not be implemented as part of creation.

Authorize the caller and validate connection/protocol/model fields before
making a bounded upstream request. The connection name has no uniqueness
check. The submitted model prices follow the same required-field validation
as creation; testing never resolves catalog prices. The operation does not save provider configuration, reserve names,
create router bindings, or update existing records. Provider calls occur
outside database transactions.

For a completed test, `200 OK` returns a top-level `success` boolean and model
results in submitted order:

```json
{
  "success": false,
  "models": [
    { "index": 0, "name": "gpt-4.1", "success": true },
    {
      "index": 1,
      "name": "gpt-4.1-mini",
      "success": false,
      "code": "model_unavailable",
      "message": "The model is unavailable with this connection."
    }
  ]
}
```

Top-level success requires every submitted model to pass. Planned failure
codes are `invalid_credentials`, `connection_failed`, `timeout`,
`model_unavailable`, and `rate_limited`. Upstream authentication failure is a
test outcome, not an API `401`: that status is reserved for Compallm caller
authentication. Malformed/unsupported requests use `400`, caller authentication
and access use `401`/`403`, and internal failures use `500`, with the shared
error envelope. Never return raw upstream response bodies.

## Requirements still to settle

- Exact supported protocol/mode combinations and whether unsupported media
  modes should be excluded from the initial API enum.
- Catalog refresh/version policy and storage for catalog source metadata.
  Preset rates are submitted explicitly; creation does not resolve them.
- Persistence for endpoint mode, context limits, thinking levels, and catalog
  references; the current schema does not store all these fields.
- Whether to accept the proposed atomic batch, case-sensitive name comparison,
  decimal-string IDs, and optional structured error issues as shared API policy.
- Credential encryption and rotation configuration.
- Test probes per protocol/mode, concurrency/time budgets, admission limits,
  cancellation behavior, and any usage charges. A successful test only confirms
  the submitted configuration at that moment.

## Related pages

- [Provider decisions](adr.md)
- [Managing providers](management.md)
- [Provider configuration](../portal/add-provider.md)
- [Models and providers](../model/models-and-providers.md)
