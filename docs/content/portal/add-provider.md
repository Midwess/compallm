---
title: Add provider design
available: false
---

# Add provider design

Decision status: A two-page catalog/custom setup flow and shared component reuse are accepted for this desktop design. Implementation status: Penpot contains minimal all, selected, filtered, and no-results catalog states with shared filter popovers plus preset/custom configuration, as editable compositions of shared base components. The local interactive reference searches and paginates all 4,370 captured catalog entries, combines filters, and demonstrates default filling. Penpot uses representative rows and fixed transitions; live data filtering is demonstrated in the local reference. No React application, LiteLLM runtime, provider requests, or credential storage is integrated.

## Choose a provider and model

Opening Add provider shows a minimal, full-width model catalog. The table, header, group labels, and default rows have transparent backgrounds so they sit directly on the page surface. Keep the borderless 50 px rows, two-line identity, 13/12 px text, and 8 px row corners from `linear_app_devlogs_settings_members.h2d`. Hover, focus, and selection remain explicit states.

Remove featured-model cards and the permanent left filter rail from the chooser. Search sits above the table alongside compact 28 px filter and display actions, adapting the toolbar in `linear_app_devlogs_team_DEV_all (1).h2d`. Filters open on demand; Provider, Mode, and Capability use pill tabs adapted from the capture. The supplied view shows a details panel, so the model-filter popover and its interactions are Compallm adaptations. Active filters appear as removable pills next to search; clearing them restores the full catalog.

Search matches provider names and model identifiers. Each row identifies one provider/model pair, its mode, input context limit, and pricing with explicit units. Include all, filtered, selected, no-results, and open-filter states. Paginate the complete catalog and show the result count and current range. Unknown metadata remains unknown; only offer capability filters supported by the captured metadata. Use shared components, Linear control surfaces, and the spacing policy throughout.

Use the user-edited **Add provider · 1 Model selected** frame as the visual reference. Keep the compact page progress above the content, a full-width search field, centered pagination, and the right-aligned **Use custom provider** / **Continue** footer. Remove the separate title/description, result count, selection summary, Cancel action, and repeated pagination range. Match its left inset and control sizes in the filtered and no-results states. Configuration pages use the same progress placement and left alignment, without a large repeated introduction. Keep their required fields and connected numbered section line; retain helpers only where they clarify credentials, units, or unknown values.

Place the secondary **Use custom provider** action in the catalog footer. **Continue** moves a selected preset to page 2. Continue is disabled until a valid selection exists. The custom action goes directly to the same configuration page with blank editable fields. Back preserves the filters, selection, and any edits; changing presets must not silently overwrite user edits.

The catalog design can represent known LiteLLM entries without declaring every entry implemented in Compallm. Production availability, supported adapter coverage, retired-model filtering, catalog refresh/pinning, and updates to edited presets remain separate decisions.

## Configure the connection

Page 2 retains GitHub's numbered sections and connected step line:

1. **Name**: editable connection name, initially derived from the selected preset.
2. **Connection details**: **Base URL** at left and a **Mode** dropdown at right, followed by required API key and API protocol. Show the combined request URL for review. The API key is always empty initially; provider-specific credentials or deployment fields remain required when no reliable default exists.
3. **Models**: add multiple models to the provider. An empty-state prompt and centered **Add model** action open a popup with a required model name, optional maximum context window in tokens, and supported thinking levels. Added entries show their model names; an omitted context limit remains unspecified rather than zero. The popup can be opened again to add another model. Duplicate handling and persistence remain unresolved.
4. **Pricing**: a separate optional section with input, output, and cached-token prices. Each field defaults to **Auto** and accepts an explicit custom rate. Auto remains distinct from zero; how automatic prices are resolved and used for charging is unresolved.
5. **Test connection**: shared test action before the Add provider footer. Testing requirements, result states, and whether a test can incur usage are unresolved; the prototype makes no real request.

Mode identifies an API surface, separately from provider identity and wire protocol. Use friendly names with endpoint paths: Chat completions (`/chat/completions`), Responses (`/responses`), Messages (`/messages`), Video (`/videos`), and Live / Realtime (`/realtime`), alongside other supported task types. The Chat Completions path is plural. Filter known-preset modes to verified compatible endpoints. Custom configuration can enter or choose a supported path. Some providers use RPC/model-specific paths or WebSockets, so do not assume every mode is a simple HTTP suffix or interchangeable across providers.

Keep host/version in Base URL and the relative route in Mode, avoiding duplicated `/v1`. For example, `https://api.openai.com/v1` plus `/chat/completions` yields `https://api.openai.com/v1/chat/completions`.

## Defaults and pricing

[LiteLLM's model cost map](https://docs.litellm.ai/docs/proxy/model_cost_map) supplies provider slugs, model modes, context limits, capabilities, and prices. It does not generally supply base URLs; obtain those from verified provider configuration/docs. Use `max_input_tokens` for context, not the ambiguous legacy `max_tokens`. Only preselect thinking levels when explicitly known; `supports_reasoning` alone does not establish Low/Medium/High/Xhigh/Max support.

Token prices remain **USD per 1 million tokens**, converting map values by multiplying per-token costs by one million. Label Cached price as a cache-read rate; cache writes and tiered rates are distinct. Video, image, and realtime presets retain their actual billing units or show review-required fields; do not relabel per-second/image/audio prices as token prices. See [supported endpoints](https://docs.litellm.ai/docs/supported_endpoints) and the [Router ADR](../llm-router/adr.md). A local source snapshot records the sample defaults used in the canvas.

## Composition

Both pages reuse the shared layout/sidebar and Design foundations fields, dropdowns, helpers, buttons, and feedback. Extend the shared bases with provider/model result rows, catalog selection states, page progress, and mode selection. Use the [half-density spacing policy](spacing.md), with a 2 px nonzero minimum. Preserve control sizes, typography, border thickness, and corners. The Add provider page owns its screens and feature-specific assemblies. Only the underlying base controls belong on Design foundations; do not add full-page patterns there. Save a named version, verify links and navigation, and inspect fresh exports.

The user-edited selected frame is the visual reference: progress begins at the 276 px content inset and 37 px top offset, catalog search is 38 px high, pagination is centered without a repeated range, and footer actions are 28 px high. The current **Use custom provider** footer action is 142 × 28 px. The older 166 × 28 px Content button remains a reusable base, not the current provider-footer layout. Preset and custom configuration use the same 1120 px content width as the catalog table, beginning at the same left inset. Expand the fields and row groups across that width: name uses the full body width, URL/mode and key/protocol retain their left/right proportions, model/context split evenly, and the three prices use equal columns. Preserve the number rail, compact vertical spacing, control heights, and right-aligned footer. Shared controls retain the [Linear surface rule](controls.md#button-surfaces).
