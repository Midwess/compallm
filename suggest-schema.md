# Model, provider, and provider-model schema proposal

Proposed design for organization-owned API-key connections. Application code,
migrations, and generated schemas are unchanged. Model and usage feature docs
describe the corresponding proposed changes; implementation is a separate change.

## 1. Tables

One provider owns shared connection settings and one API key. Each provider
model directly declares its service kind, protocol, optional connection,
headers, capabilities, and upstream prices. The service/protocol pair selects
one concrete adapter. Routing
availability requires an enabled logical model, provider model, and router binding.
The organization-facing `models` and `model_router` remain separate resources.

### `providers`

| Column | PostgreSQL type | Rust value | Meaning |
| --- | --- | --- | --- |
| `id` | `BIGINT PRIMARY KEY` | `u64` | Positive application-generated connection ID; checked conversion to signed BIGINT. |
| `organization_id` | `BIGINT NOT NULL` | `u64` | Owning organization; foreign key to `organizations`. |
| `name` | `TEXT NOT NULL` | `String` | Nonblank connection name, unique within the organization after trimming. |
| `connection` | `JSONB NOT NULL` | `Connection` | Shared base URL, bounded timeout, non-secret query parameters, and API-key placement. |
| `headers` | `JSONB NOT NULL DEFAULT '{}'` | `Headers` | Non-secret custom headers, normalized by header name. |
| `credentials` | `JSONB NOT NULL` | `EncryptedCredentials` | Private encrypted envelope containing the API key and secret custom headers. Excluded from ordinary reads. |

Providers hold the shared connection, headers, and private credentials. Each
model's service/protocol pair selects its concrete upstream adapter. Presets prefill ordinary
configuration. The base URL lives in `connection`.

### `models`

Logical models use the same shared configuration as provider models, without
`provider_id`, `protocol`, `connection`, or `headers`:

| Column | PostgreSQL type | Rust value | Meaning |
| --- | --- | --- | --- |
| `id` | `BIGINT PRIMARY KEY` | `u64` | Positive application-generated logical-model ID. |
| `organization_id` | `BIGINT NOT NULL` | `u64` | Owning organization. |
| `name` | `TEXT NOT NULL` | `String` | Nonblank caller-facing name, unique within the organization. |
| `service_kind` | `TEXT NOT NULL` | `ServiceKind` | Caller-facing operation. |
| `enabled` | `BOOLEAN NOT NULL DEFAULT TRUE` | `bool` | Logical-model availability; disabling it blocks all routes through this model. |
| `capabilities` | `JSONB NOT NULL DEFAULT '[]'` | `Vec<Capability>` | Declared caller-facing capability flags. |
| `limits` | `JSONB NOT NULL DEFAULT '{}'` | `ModelLimits` | Caller-facing limits using the shared typed shape. |
| `thinking` | `JSONB NULL` | `Option<Thinking>` | Optional caller-facing effort and budget controls. |
| `pricing` | `JSONB NOT NULL` | `Pricing` | Organization-facing prices, not upstream costs. |

Typed configuration replaces the logical model's scalar token prices and
generic `settings`; no generic timestamps are retained. Shared JSONB fields
have the same shapes and structural validation as provider-model fields, but
values are configured independently, without automatic merging or inheritance.
Routes continue to select upstream provider models by stable IDs. Exact
service/capability/limit compatibility and request validation remain unresolved.
The shared pricing shape does not define organization-facing media charging.
Before implementation, map existing scalar prices losslessly and decide how
meaningful existing settings are preserved before removing them.

### `provider_models`

| Column | PostgreSQL type | Rust value | Meaning |
| --- | --- | --- | --- |
| `id` | `BIGINT PRIMARY KEY` | `u64` | Positive application-generated provider-model ID. |
| `organization_id` | `BIGINT NOT NULL` | `u64` | Ownership scope for composite foreign keys. |
| `provider_id` | `BIGINT NOT NULL` | `u64` | Owning provider connection in the same organization. |
| `name` | `TEXT NOT NULL` | `String` | Nonblank exact upstream model identifier; preserve case. |
| `service_kind` | `TEXT NOT NULL` | `ServiceKind` | Requested operation, such as LLM inference, image generation, or transcription. |
| `enabled` | `BOOLEAN NOT NULL DEFAULT TRUE` | `bool` | Configured model availability. |
| `protocol` | `TEXT NOT NULL` | `Protocol` | API-format or implementation family, interpreted within the selected service. |
| `connection` | `JSONB NULL` | `Option<Connection>` | Optional complete replacement for the provider connection; null uses the shared provider connection. |
| `headers` | `JSONB NOT NULL DEFAULT '{}'` | `Headers` | Non-secret model headers merged over provider headers. |
| `capabilities` | `JSONB NOT NULL DEFAULT '[]'` | `Vec<Capability>` | Explicitly declared feature flags; an empty array declares no optional capabilities. |
| `limits` | `JSONB NOT NULL DEFAULT '{}'` | `ModelLimits` | Optional positive limits with units in their field names. |
| `thinking` | `JSONB NULL` | `Option<Thinking>` | Independent optional effort and budget controls; both present means both are supported. Null means no controls are configured, not a classification of reasoning support. |
| `pricing` | `JSONB NOT NULL` | `Pricing` | One required pricing variant with exact decimal amounts and explicit units. |

Pricing is a typed enum: token, audio, image, video, or request. Each value has
one variant containing only its applicable rates. Token pricing requires all
three explicit input, output, and cache-read rates; zero is a valid configured
rate. Token prices are fixed per model. Image/video conditions describe
differing output rates.

The proposed Core values use `rust_decimal::Decimal` directly. Decimal amounts
in `pricing` JSONB are strings, preserving exact input across JavaScript clients.
Other scalar monetary columns retain exact PostgreSQL NUMERIC storage. The
current application still uses `Price(u128)` until a separately implemented
change updates its domain types and persistence conversions.

Required database integrity:

- Positive IDs, nonblank normalized names, and nonnegative prices. JSONB values
  are objects except `capabilities`, which must be an array of distinct flags.
- `providers`: unique `(organization_id, name)` and `(organization_id, id)`.
- `models`: unique `(organization_id, name)` and `(organization_id, id)`;
  `organization_id` references `organizations`. Preserve route and usage scope.
- `provider_models`: unique `(organization_id, provider_id, name, service_kind, protocol)`;
  retain unique `(organization_id, id)` and `(organization_id, provider_id, id)`
  as targets of existing route/usage foreign keys.
- Composite foreign key `(organization_id, provider_id)` references
  `providers(organization_id, id)`. Preserve organization ownership on routes
  and usage; existing usage-retention restrictions still apply.
- `pricing` must be an object with exactly one supported variant. Core validates
  the selected variant against the service and requires its rate fields.

Each provider-model record has one explicit `(service_kind, protocol)` pair.
The same upstream model identifier may have separate records for different
pairs; routing uses stable IDs and does not automatically choose among
upstream formats. Existing organization-scoped route and usage foreign keys
continue to reference those IDs.

`service_kind` answers what the caller wants to do. `protocol` identifies the
API-format or native implementation family used for that service. The pair
selects a concrete adapter with defined URL construction, request encoding,
response decoding, streaming behavior, and supported suboperations. There is
no separate stored adapter identifier or provider integration key.

The initial compatibility contract is explicit:

| Service kind | Protocol values | Concrete behavior |
| --- | --- | --- |
| `llm` | `openai`, `openai_responses`, `anthropic`, `gemini` | Chat/text inference and supported vision/tool features. |
| `image` | `openai`, `gemini` | Image generation and adapter-supported editing. |
| `embedding` | `openai` | OpenAI-compatible embeddings. |
| `tts` | `openai`, `minimax`, `openrouter` | Native speech generation for the selected family. |
| `stt` | `openai` | OpenAI-compatible transcription. |
| `video` | `xai` | Video-job submission, polling, and adapter-supported operations. |

For example, `llm + openai` selects Chat Completions, `tts + openai` selects
OpenAI speech, and `stt + openai` selects transcription. `tts + openrouter`
uses audio through Chat Completions and SSE extraction, rather than the OpenAI
speech endpoint. Provider-native differences belong in the selected concrete
client; a shared format label never promises compatibility with every vendor.

These are proposed adapter contracts; the corresponding clients are not yet
implemented. Add a pair alongside a required concrete implementation, its
configuration contract, and representative fixtures. Validation and dispatch
use the same supported-pair definition; unsupported combinations fail before
calling upstream. Enum membership alone does not establish runtime support or
parity with every 9Router integration.

A compatible endpoint reuses its protocol regardless of provider name. Groq,
Mistral, and a custom server can use `llm + openai` when their API satisfies
that adapter's contract. MiniMax chat through its Anthropic-compatible API
uses `llm + anthropic`; its native speech API uses `tts + minimax`.

Service kind is stored and exposed directly. Image understanding is the `llm`
service with `vision`, so accepting image content does not require another
operation enum. Search, fetching, decision APIs, and other native integrations
get service/protocol pairs when their concrete clients are introduced. Shared
code is factored between real clients when behavior matches, without a generic
request-transformation language or arbitrary provider-quirk map.

A model with null `connection` uses the provider's complete `Connection`.
A supplied model connection is the same complete structure and replaces the
provider connection in full. It must include the base URL, timeout, and API-key
placement; there is no partial merge. Model headers merge over provider
headers using case-insensitive names. Secret headers are applied separately;
public custom headers cannot replace authentication or transport-managed
headers. Every model uses the owning provider's API key.

The selected adapter constructs operation-specific URLs and handles streaming
or job polling. Suboperations such as image editing and video polling come
from the request and the adapter's supported operation set. Their existence
does not require another endpoint-mode column. Unsupported client formats or
capabilities fail explicitly instead of silently losing content.

API-key placement is a header or query parameter and never contains the key
value. Other query parameters remain non-secret; the credential is injected
only when sending the request. OAuth, cookies, local-device integrations, and
multi-part cloud credentials are outside this proposal.

Runtime retries, connection selection, health, and cooldowns remain separate
from the protocol and saved model configuration. One provider-model record
always selects one upstream adapter; incoming client formats can be translated
into it within the declared translation coverage.

References: [9Router chat dispatch](https://github.com/decolua/9router/blob/1fe586bb1cf35e1ed0cb2150a8ef7d1db040b21d/open-sse/handlers/chatCore.js),
[native service adapters](https://github.com/decolua/9router/tree/1fe586bb1cf35e1ed0cb2150a8ef7d1db040b21d/open-sse/handlers),
and [OpenRouter speech handling](https://github.com/decolua/9router/blob/1fe586bb1cf35e1ed0cb2150a8ef7d1db040b21d/open-sse/handlers/ttsProviders/openrouter.js).

Capabilities are a flat list of declared flags. The following meanings cover
9Router's common capability flags and its forced-tool-choice override while
retaining the existing text and structured-output flags:

| Capability | Meaning | 9Router correspondence |
| --- | --- | --- |
| `text` | Supports text content. | Text is implicit for LLM models. |
| `vision` | Understands images. | `vision` |
| `pdf` | Understands PDF/document content. | `pdf` |
| `audio` | Understands audio content. | `audioInput` |
| `video` | Understands video content. | `videoInput` |
| `image_generation` | Generates image content. | `imageOutput` |
| `audio_generation` | Generates audio content. | `audioOutput` |
| `search` | Supports provider-integrated web search or grounding. | `search` |
| `reasoning` | Supports reasoning/thinking. | `reasoning` |
| `tool_call` | Can request tool/function execution. | `tools` |
| `forced_tool_choice` | Accepts a forced tool selection; also requires `tool_call`. | `forcedToolChoice` overrides |
| `structured_output` | Supports schema-constrained responses. | Adapter/model parameter support, not a common capability boolean. |

Only declared capabilities are available; absence does not enable a feature.
The service identifies the requested operation, while capability flags identify
the model's additional abilities. A flag alone does not implement an API or
enable another service route. No separate input/output lists or capability
boolean fields are stored. The `thinking` object owns reasoning controls;
reasoning support does not imply particular effort levels. Internal 9Router
thinking-format flags and numeric limits are not general capabilities.

Reference: [9Router capability definitions](https://github.com/decolua/9router/blob/1fe586bb1cf35e1ed0cb2150a8ef7d1db040b21d/open-sse/providers/capabilities.js).

## 2. Rust structures

These are proposed Core values and private persistence values, with no SeaORM
or PostgreSQL types. Provider aggregates use `Decimal` monetary values;
Serde applies to the focused JSONB values. SQL names use snake_case, JSON
property names use camelCase, and enum strings use snake_case.

Connection authentication is a nested object. Its untagged enum accepts exactly
one header or query placement, and Serde rejects unknown fields directly.
Thinking is a strict object with independent optional effort and budget fields.
At least one must be present; both present means the model supports both controls.

`ProviderModel` owns `service_kind: ServiceKind`, `protocol: Protocol`,
`connection: Option<Connection>`, and `headers: Headers` directly. Service kind
and protocol serialize as strings. Their supported pair selects the concrete
client implementation. Unknown service/protocol
values and unknown JSONB object fields are rejected. Authentication retains its
nested untagged enum; thinking uses a nested struct.

`Pricing` uses a named outer variant so the billed quantity is explicit.
Token rates apply to token-reported usage, including a token-billed media model
when that shape covers its actual rates. Audio supports duration or character
pricing. Image and video support conditional output rates; request pricing
counts billable operations. Combined token/output/request fees require a typed
combined variant before serving those models; one variant must never silently
discard another billable component.

The proposed dependency is `rust_decimal` with `serde-with-str` enabled.
Per-field `rust_decimal::serde::str` annotations require monetary JSON strings
and prevent binary-float serialization. Domain validation still rejects
negative, out-of-range, or over-precision amounts.

```rust
use std::{
    collections::BTreeMap,
    num::{u32, u32},
};

use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};

pub type Headers = BTreeMap<String, String>;
pub type QueryParameters = BTreeMap<String, String>;

// Public domain read models deliberately exclude credentials.
#[derive(Clone, Debug)]
pub struct Provider {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub connection: Connection,
    pub headers: Headers,
    pub models: Vec<ProviderModel>,
}

#[derive(Clone, Debug)]
pub struct ProviderModel {
    pub id: u64,
    pub organization_id: u64,
    pub provider_id: u64,
    pub name: String,
    pub service_kind: ServiceKind,
    pub enabled: bool,
    pub protocol: Protocol,
    pub connection: Option<Connection>,
    pub headers: Headers,
    pub capabilities: Vec<Capability>,
    pub limits: ModelLimits,
    pub thinking: Vec<Thinking>,
    pub pricing: Pricing,
}

// Organization-facing configuration; upstream transport belongs to routed models.
#[derive(Clone, Debug)]
pub struct Model {
    pub id: u64,
    pub organization_id: u64,
    pub name: String,
    pub service_kind: ServiceKind,
    pub enabled: bool,
    pub capabilities: Vec<Capability>,
    pub limits: ModelLimits,
    pub pricing: Pricing,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ServiceKind {
    Llm,
    Image,
    Video,
    Embedding,
    Tts,
    Stt,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct Connection {
    pub base_url: String,
    pub timeout_ms: u32,
    #[serde(default)]
    pub query: QueryParameters,
    pub authentication: ApiKeyAuthentication,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged, deny_unknown_fields, rename_all_fields = "camelCase")]
pub enum ApiKeyAuthentication {
    Header { api_key_header: ApiKeyHeader },
    Query { api_key_query: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ApiKeyHeader {
    pub name: String,
    pub prefix: String,
}

// Vocabulary for planned adapters; enum membership does not implement support.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Protocol {
    Openai,
    OpenaiResponses,
    Anthropic,
    Gemini,
    Minimax,
    Openrouter,
    Xai,
}

impl Protocol {
    pub const fn supports(self, service: ServiceKind) -> bool {
        matches!(
            (service, self),
            (
                ServiceKind::Llm,
                Self::Openai | Self::OpenaiResponses | Self::Anthropic | Self::Gemini
            ) | (ServiceKind::Image, Self::Openai | Self::Gemini)
                | (ServiceKind::Embedding, Self::Openai)
                | (
                    ServiceKind::Tts,
                    Self::Openai | Self::Minimax | Self::Openrouter
                )
                | (ServiceKind::Stt, Self::Openai)
                | (ServiceKind::Video, Self::Xai)
        )
    }
}

// Private storage value: serialize only inside persistence/encryption code.
// The selected encryption format defines nonce/tag encoding and key rotation.
#[derive(Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct EncryptedCredentials {
    pub version: u16,
    pub key_id: String,
    pub nonce: String,
    pub ciphertext: String,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Capability {
    Text,
    Vision,
    Pdf,
    Audio,
    Video,
    ImageGeneration,
    AudioGeneration,
    Search,
    Reasoning,
    ToolCall,
    ForcedToolChoice,
    StructuredOutput,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ModelLimits {
    pub context_tokens: Option<u32>,
    pub max_output_tokens: Option<u32>,
    pub max_images_per_request: Option<u32>,
    pub max_duration_ms: Option<u32>,
    pub max_audio_bytes: Option<u32>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub enum Thinking {
    Low(String),
    Medium(String),
    High(String),
    Xhigh(String),
    Max(String),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Pricing {
    Token(TokenPricing),
    Audio(AudioPricing),
    Image(ImagePricing),
    Video(VideoPricing),
    Request(RequestPricing),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct TokenPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub input_usd_per_million_tokens: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub output_usd_per_million_tokens: Decimal,
    #[serde(with = "rust_decimal::serde::str")]
    pub cached_usd_per_million_tokens: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum AudioPricing {
    Duration(AudioDurationPricing),
    Character(AudioCharacterPricing),
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioDurationPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_second: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct AudioCharacterPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_million_characters: Decimal,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImagePricing {
    pub rates: Vec<ImageRate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct ImageRate {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_image: Decimal,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoPricing {
    pub rates: Vec<VideoRate>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct VideoRate {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_second: Decimal,
    #[serde(default)]
    pub conditions: RateConditions,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RequestPricing {
    #[serde(with = "rust_decimal::serde::str")]
    pub usd_per_request: Decimal,
}

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[serde(deny_unknown_fields, rename_all = "camelCase")]
pub struct RateConditions {
    pub resolution: Option<String>,
    pub quality: Option<String>,
    pub audio: Option<bool>,
    pub service_tier: Option<String>,
}
```

Example column values are grouped by provider and provider model. JSONB fields
use the actual Serde field names:

```json
{
  "provider": {
    "connection": {
      "baseUrl": "https://inference.example.com/v1",
      "timeoutMs": 30000,
      "query": {},
      "authentication": {
        "apiKeyHeader": { "name": "Authorization", "prefix": "Bearer " }
      }
    },
    "headers": {}
  },
  "providerModel": {
    "serviceKind": "llm",
    "protocol": "openai",
    "connection": null,
    "headers": {},
    "capabilities": ["text", "vision", "pdf", "reasoning", "tool_call", "structured_output"],
    "thinking": {
      "effort": ["low", "medium", "high"],
      "budget": {
        "minTokens": 1024,
        "maxTokens": 32768,
        "defaultTokens": 8192
      }
    },
    "pricing": {
      "token": {
        "inputUsdPerMillionTokens": "2.00",
        "outputUsdPerMillionTokens": "8.00",
        "cachedUsdPerMillionTokens": "0.50"
      }
    }
  }
}
```

The example groups independent columns for readability. The connection's API
key placement is nested under `authentication`. The model's required `protocol`
is a direct column; its null `connection` uses the provider connection.
Thinking has no additional `kind` or state tag. Its independent optional fields
allow `{ "effort": ... }`, `{ "budget": ... }`, or
`{ "effort": ..., "budget": ... }`. Both present means both controls are
supported, without a separate combined variant. These are capabilities, not
selected request values; the concrete adapter validates request-level combinations.
The example's levels and token bounds are illustrative, not an upstream model contract.
`effort` is a nonempty array of distinct supported `EffortLevel` values with no
wrapper or configured default effort. Budget requires positive `minTokens` and
`maxTokens`; optional `defaultTokens` must lie within those bounds.
SQL `NULL` and Rust `None` mean no reasoning controls are configured, without
classifying support as unknown or unsupported. An empty object or an object
with both fields absent or null is invalid.

Core uses Validify for domain validation beyond Serde structure checks:

- IDs fit BIGINT; connection names and model IDs are normalized; the selected
  service/protocol pair has a supported concrete adapter. A supplied model connection is
  validated with the same complete rules as the provider connection.
- `Protocol::supports(service_kind)` validates the supported combinations from
  the table. Invalid pairs fail before HTTP dispatch. Image understanding
  requires `vision`; provider-integrated search requires `search` and explicit
  support in the selected adapter.
- HTTP(S) base URLs contain no credential, query, or fragment. Query settings
  are explicit. Timeout values have a selected upper bound. Header names and
  values are valid and cannot contain CR/LF or override managed/auth headers.
- Capability lists contain distinct declared flags. `forced_tool_choice`
  requires `tool_call`. A non-null `thinking` requires `reasoning` and at least
  one non-null control field. A present effort list is nonempty and distinct;
  budget min/default/max are ordered. A `reasoning` capability alone does not
  invent controls or effort levels and may coexist with null `thinking`.
- Requests use declared capabilities and valid parameters supported by the
  selected protocol adapter. Capability flags do not authorize arbitrary keys.
- Decimal prices are nonnegative, within the selected NUMERIC(20,8) range,
  with at most eight decimal places. JSON amounts must be strings; no blank,
  binary-float, inferred zero, or automatic price lookup. Decimal is signed,
  so nonnegativity remains a domain rule. Use checked decimal arithmetic and
  define final rounding before accounting is implemented.
- LLM models require token pricing with all three rates. Embedding and
  token-priced media may use token pricing when it represents their actual
  billing. Duration-priced audio uses billable input duration for STT and
  output duration for TTS; character-priced audio uses billable input text.
- Image/video rate lists are nonempty and their conditions do not overlap.
  Request pricing uses billable operation count. Missing applicable media rates
  are errors, never inferred free.
- Provider adapters define billable measurements. Do not double-count cached
  input or reasoning tokens included in reported totals. Compound fees,
  modality-specific token rates, duplex audio, and rounding/minimum charges
  require additional typed support before
  those pricing plans become usable.

Presets may prefill editable values. Persist only submitted model configuration
and explicit prices; later preset updates never overwrite saved values.

Owner/admin authorization, atomic provider/model writes, optional unsaved
connection tests, and write-only credentials remain the management boundaries.
Creation makes no upstream request. Test outcomes, health, rate limits, and
cooldowns live outside these configuration tables. Add no generic timestamps.

Before implementation, define the shipped service/protocol adapters and their configuration,
credential encryption/key rotation, numeric field/request limits, translation
coverage for the selected protocol, and exact pricing calculations. The proposed
independent typed provider-usage tables, direct client-request links, and historical pricing snapshots are
defined in [usage accounting](docs/content/usage-administration/accounting.md)
and shown in `dbdiagram.dbml`; implemented usage persistence remains token-only.
Organization-facing media billing still needs its own contract.
The present enum vocabulary describes storage shapes, not implemented clients.

When implementing this proposal, backfill each provider model's three existing
token-rate values into `pricing.token` without rounding, then remove the
duplicated columns in a reviewed migration. Provider usage must snapshot the
selected pricing variant and measured units. Update logical-model monetary
types and NUMERIC conversions consistently when replacing the shared `Price`
wrapper with `Decimal`; JSONB prices are decimal strings, not NUMERIC columns.

Implementation must update the provider/model/Router ADRs and provider contract
before application changes, preserve scoped foreign keys, and correct the
generated composite relations while regenerating affected PostgreSQL entities.
No migration or application startup is authorized by this proposal.
