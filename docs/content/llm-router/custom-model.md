---
title: Custom model fallback design
available: false
---

# Custom model fallback design

Decision status: Ordered fallback with observed, read-only serving status and shared design controls is accepted for this design. Implementation status: Penpot design work; routing, persistence, health detection, and serving telemetry remain pending.

A custom model has a caller-facing name and an ordered list of configured provider/model pairs. Dragging changes priority. The small percentage beside each provider reports its observed share of serving responsibility over a reporting period; it is not editable and does not configure traffic weights. A first-priority provider can show 0% if it was down for the whole period. Exclude down or inactive providers from serving; the displayed percentages reflect actual responsibility among providers that served. The earlier interpretation as user-configured weights is superseded. The precise measurement window and aggregation remain unresolved.

## View and edit modes

Use the existing Create model design page for model details and editing; do not add a new application page for switching modes. Default model view shows fixed provider/model values, health/exhaustion icons and labels, semantic borders/text, and observed serving percentages. It has no dropdowns, drag handles, add/remove controls, or editable percentage fields. **Edit** switches the same screen into editing. Edit mode reveals the model-name/prompt fields, provider dropdowns, drag handles, inline addition, trash controls, and **Cancel** / **Save changes**. Cancel returns to the read-only view without applying edits; save commits valid edits and returns to view in an implemented application. New-model creation uses the editing layout with **Create model**. Penpot frames illustrate these states on the existing design page and do not implement persistence.

Healthy uses a green check, Exhausted an amber warning, Down a red flag, and Inactive muted gray. Include a text label as well as color; keep inner icons 12 px. Exhausted indicates currently unavailable serving capacity, with detection thresholds/recovery still unresolved. Observed percentages describe the reporting period, so an exhausted provider may still have a nonzero historical share.

## Create model

Use the minimal Compallm shell with a centered content column capped at 800 px. Use a plain page title without a breadcrumb. Keep the model-name field, ordered provider list, existing optional system-prompt field, and a compact action footer. Use shared labels, fields, dropdowns, buttons, status icons, and helper/error controls. Rows have transparent backgrounds. Replace Primary/Fallback badges with small, muted, read-only percentage text. A down provider has a red flag on the far right; unavailable or disabled content is muted while its flag remains red. Observed shares can total 100% when serving occurred; do not infer activity from priority alone. Every route identifies both the configured connection and its upstream model. Do not expose credentials or treat a provider name alone as a complete route.

The supplied `linear_app_devlogs_reviews.h2d` contains sidebar customization rather than a code-review pane. It shows compact 36 px reorderable rows, a 6 × 10 px six-dot grip, and a trailing 30 px dropdown. Adapt that anatomy into a compact fallback list. The capture does not show a drag in progress; lifted rows and insertion markers are Compallm states.

Keep the route assembly on the dedicated Create model page. Design foundations owns only atomic bases, including the drag handle and insertion indicator. Do not introduce a page-pattern component or place the full fallback editor in Foundations.

## Editing order

- Show a small observed serving percentage without a field, tag, or ordinal badge. Drag the grip to reorder priority; preserve the provider/model pair.
- Show the lifted entry, a stable placeholder, and a visible insertion line. Escape cancels without changing order.
- Do not show arrow buttons. Support keyboard reordering through the drag handle, restore focus to the moved entry, and announce its position in an implemented interface. Removal uses a Huge Icons trash control.
- Add a configured provider/model pair through a full-width dropdown row matching the existing row's dropdown width. Its empty state has no drag handle. Choosing a pair appends a draggable entry and leaves a fresh empty dropdown row for another addition. Do not use a separate Add provider button. Remove an entry with its explicit action. The design reference prevents duplicate pairs and requires a name plus at least one complete route before enabling Create model.
- Preserve the existing name and system prompt while editing routes. Empty, populated, picker-open, dragging, reordered, and validation states illustrate the design. Penpot uses fixed prototype transitions; a local reference can demonstrate actual drag/keyboard behavior.

## Rules still to decide

Which failures allow fallback, per-attempt and total timeouts, retry limits, backoff, exhaustion responses, cancellation, and partial-stream behavior remain unresolved. Do not imply that every error is retryable or restart a partially delivered response silently. Model capabilities and endpoint compatibility need validation across every route; protocol conversion does not supply missing capabilities. System-prompt merging, name uniqueness, model lifecycle, permissions, and accounting across attempts also remain separate decisions. See the [Router ADR](adr.md) and [Gateway](../llm-gateway/adr.md).
