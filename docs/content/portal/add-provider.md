---
title: "Provider configuration"
available: false
---

# Provider configuration

Provider configuration defines a connection name, base URL, endpoint mode, API protocol, credentials, and upstream models. Each model has a required identifier and optional context limit and supported thinking levels. Runtime integration and persistence remain planned.

## Catalog defaults

Selecting a provider identity immediately opens its configuration; there is
no separate continue action. Select a provider identity before choosing models. Provider discovery returns
distinct providers; model discovery is scoped to the selected provider.
Callers explicitly enable the known models they want to configure and may add
custom model definitions to the same connection. Unselected catalog entries
are not included in the creation request. Both catalog and custom models
require explicit input, output, and cache-read prices before enabling them.

The LiteLLM model cost map supplies provider slugs, model modes, context limits, capabilities, and prices. Obtain base URLs from verified provider documentation; the cost map generally does not supply them. Use `max_input_tokens` for input context. Only configure thinking levels explicitly supported by the model; a reasoning-capability flag does not establish particular levels.

Catalog lookup matches provider names and model identifiers. Provider, mode, and capability constraints use known metadata. Unknown metadata stays unspecified. A preset supplies editable defaults but never supplies an API key. Custom configuration starts without inferred provider defaults. Changing a preset must preserve explicit user edits.

## Endpoint construction

Keep host/version in the base URL and the relative route in the endpoint mode. `https://api.openai.com/v1` plus `/chat/completions` produces `https://api.openai.com/v1/chat/completions`. Avoid duplicated version segments.

Provider identity, API protocol, and endpoint mode are separate. Modes include Chat Completions, Responses, Messages, Video, and Realtime where supported. RPC/model-specific paths and WebSockets require protocol-aware handling; not every mode is an interchangeable HTTP suffix.

## Pricing and unresolved requirements

Token prices use USD per million tokens; multiply per-token cost-map values by one million. Cached-token price means cache-read price; cache writes and tiers are distinct. Image, video, and audio retain their actual billing units. All three token rates are required for each model; explicit zero is a valid free rate.

Each upstream model requires explicit input, output, and cache-read prices.
Catalog presets may prefill known prices as editable values. Missing, null,
or blank rates are invalid; the server does not resolve them automatically.
Changing one model's price does not change other models. The planned
[provider API contract](../provider/api-contract.md) accepts multiple
connections, each with multiple models, and defines a separate test operation
for unsaved configuration. Saving never requires a successful connection test.

Catalog refresh/pinning, retired-model handling, and supported adapters remain
unresolved. Connection tests may incur usage; define their probes and charging
rules before implementation. The API contract records outstanding persistence,
batch, and validation decisions.

See the [Router architecture](../llm-router/adr.md).


## Component integration

The library exports `CreateProviderPage`, `createCreateProviderStore`, and the
`CreateProviderController` interface through its create-provider page entry.
Consumers provide one stable controller through `CreateProviderControllerContext`
from the public context module. The page composes inside the organization shell;
it does not create a provider, call an API, or register routes by itself.

The controller owns provider discovery, selection, connection fields, enabled
models, model drafts, and operation state. UI events report intents to its
methods; field subscriptions remain scoped to the components that display them.
Model editing preserves explicit decimal prices and treats an empty price as
missing, including cache-read prices. Custom model drafts use the same contract
as catalog entries. Incompatible modes remain unavailable rather than becoming
silently selectable. Test results apply only to the tested configuration and
must be cleared when that configuration changes. A successful test is optional
for creation. The consumer supplies safe error messages and owns asynchronous
cancellation, API operations, and completion navigation.

Model dialogs ignore outside clicks. Cancel, the close action, and Escape
request dismissal; a modified draft requires explicit discard confirmation.
Keeping changes returns to the draft, while confirming discard invokes the
consumer's cancel action without updating the configured model. Unchanged
drafts close directly.

Storybook previews use local sample catalogs and consumer controllers to
exercise choosing providers, configuring preset/custom connections, editing
models, connection-test results, and pending operations. They make no upstream
requests, persist no credentials, and do not implement the planned provider API.


The same library form supports saved providers and clone drafts. Its controller
supplies a non-secret saved snapshot and page intent. Save is available only
when validated configuration differs from that snapshot; search, test results,
and unsaved model-dialog edits do not count. Reverting changes disables saving.
Blank replacement credentials preserve an existing key. Clone actions delegate
to the consumer and open a separate draft with copied configuration and prices,
a ` clone` name suffix, an empty credential, and no prior test result. Cancel
restores the saved configuration or returns from a clone draft to its source.
The portal lazy-loads the creation page at `/:organizationName/providers/new`
inside the authenticated organization shell. Its controller supplies known
provider identities and unsaved configuration/model editing. Provider model
catalog discovery, persistence, and connection-test APIs remain unavailable;
those actions report that limitation through notifications. The portal never
imports preview controllers, fictional model catalogs, or simulated successes.
The sidebar Add provider action soft-navigates to this route.

Provider creation uses child routes `new/choose-provider` and
`new/configure-provider`. The injected root URL controller owns navigation and
query state; the provider controller projects non-secret fields and model drafts
from it, including reload and Back/Forward. API keys stay in memory. See
[URL state](url-state.md) for query ownership, history, and discard semantics.
