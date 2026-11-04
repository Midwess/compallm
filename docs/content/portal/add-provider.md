---
title: "Provider configuration"
available: false
---

# Provider configuration

The library exports `CreateProviderPage`, `createCreateProviderStore`, and the
`CreateProviderController` interface. Consumers create one stable controller
and supply it through the public `CreateProviderControllerContext`. The page
reports intents; consumers own discovery, navigation, credentials, and API
operations. Storybook controllers use local sample data without upstream calls.
Provider persistence and connection-test integration remain planned.

## Connection configuration

A connection owns its name, base URL, credential, avatar, and ordered custom
headers. Choosing a provider scopes its catalog and supplies editable defaults;
provider identity does not determine every model's protocol. No provider-level
mode, protocol, or user-configurable timeout is required by the library flow.
Concrete upstream clients still require bounded request timeouts.

Credentials and custom header values remain in memory. Headers can contain
secrets and must never enter URL parameters, logs, or ordinary read responses.
An empty new header entry is not saved. Entering its key creates an editable
entry, and reordering preserves its key/value pair and stable identity.
Header names are case-insensitive and unique; invalid names and line breaks
are rejected. Avatar selection reports a file to the consumer; the library
provides a local preview rather than uploading it. Remote avatar persistence
and accepted upload policy remain unresolved.

## Text models and endpoint paths

Adding models supports text generation through three explicit operations:
`openai_chat_completions`, `openai_responses`, and `anthropic_messages`.
The chosen operation initializes the draft protocol; the model editor does
not offer an independent protocol selector. Media and arbitrary protocol
configuration are outside this library flow.

Every model has an editable endpoint path. Defaults are `/chat/completions`,
`/responses`, and `/messages`, respectively. Only a path is accepted: no scheme,
host, query, fragment, or traversal segments. Append it to the connection's base
URL while retaining the base path prefix and normalizing the joining slash.
For example, `https://api.openai.com/v1` and `/responses` produce
`https://api.openai.com/v1/responses`. Never resolve a leading slash by replacing
`/v1`. Invalid input remains editable but cannot be saved.

The draft also owns the upstream model name, optional positive context/output
limits, effort-name mappings, and three explicit token prices. Effort maps
associate Low, Medium, High, Xhigh, and Max with model-specific request values.
The form calls this configuration Thinking efforts. Contextual help explains
that increasing reasoning effort can use more time and tokens, that mappings
must match values accepted by the upstream model, and that empty levels remain
unconfigured.
An empty mapping means unsupported or unconfigured; `None` is only an editing
hint and is never submitted as a mapped value. Levels are configured sequentially: users may edit configured levels or the
first unconfigured level, but cannot skip ahead. Clearing a level also clears
later mappings. Navigating between available levels retains entered mappings. Prices are nonnegative decimal strings in USD per million
input, output, and cache-read tokens; explicit zero is valid, while missing,
blank, or over-precision rates cannot enable a model.

Each model may declare capability flags: text, vision, PDF, audio input, video
input, image generation, audio generation, web search, reasoning, tool calling,
forced tool choice, and structured output. Missing declarations enable no
capabilities. The selected operation and declared capabilities are independent;
capability flags do not establish a supported endpoint or permission.
Capability filters combine with enabled-status, operation, and search filters.
Multiple capability selections require every selected flag. Clearing filters
preserves configuration and model selection. Filtering never invents capability
metadata or changes configured models.

Catalog models are configured only when explicitly enabled. Search and enabled
or protocol filters do not remove configured models. Select-all changes only
matching selectable rows; its state derives from those rows. Clearing filters
preserves searches, configuration, and model selection. Catalog defaults never
overwrite explicit edits.

## Controller integration

The controller owns connection fields, headers, avatar selection, model drafts,
filters, and operation state. UI events call controller actions; components do
not mutate the controller store or invoke APIs. Optional capability callbacks
allow existing consumers to adopt new connection controls without changing
unrelated authentication or organization scopes. Preview controllers implement
all supported actions. Legacy mode/protocol members remain compatibility data
until consumers adopt model-specific operation and path fields.

Model dialogs ignore outside clicks. Cancel, Close, and Escape request dismissal;
a changed draft requires `Dialog.Discard` confirmation. Dismissal leaves the
configured model unchanged. The selected operation's default path alone does
not make a new draft dirty.

Saved-provider management compares connection/model values with a non-secret
snapshot. Searches, filters, and test results do not make a saved provider dirty.
A blank replacement credential preserves the existing key. Cloning copies
configuration and prices into a separate draft, adds a ` clone` suffix, clears
the credential, and does not copy prior test results or router bindings.

Connection tests are optional, invalidate on configuration changes, and never
establish authorization or guarantee later inference success. Consumers supply
safe errors and own cancellation. The existing portal reports unavailable
backend operations rather than simulating production success. Its URL controller
owns workflow addresses and non-secret draft restoration; API keys, header
values, and avatar files remain memory-only. See [URL state](url-state.md) and
[the planned API contract](../provider/api-contract.md).
