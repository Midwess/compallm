---
title: "Portal: ADR"
available: false
---

# Portal: ADR

Decision status: Desktop sidebar design and component-family cleanup are authorized for design review. Navigation labels, grouping, and launch availability remain proposed. Implementation status: Design foundations contains ten base-only families. Complete sidebars, forms, feature panels, and screen compositions live on dedicated feature pages; full-page provider definitions and the Page patterns gallery are removed. Feature screens, login, and signup use shared base instances. No application routing or content implementation is authorized by this design task.

## Context

Compallm combines conversations and notebooks with organization-scoped tools and usage. Members need a predictable way to navigate and recognize their current workspace. The Add provider flow and its shell are reviewed compositions; captured feature examples remain visual references.

## Direction

Retain the reviewed Compallm sidebar and its Uxercon icons. Reuse shared Design foundations color, typography, geometry, and spacing tokens. Keep the complete sidebar assembly on its dedicated Sidebar page; feature screens compose instances of that assembly. A 244-pixel sidebar uses compact desktop rows and independently collapsible sections. Keep the layout base's adjacent canvas empty; feature screens compose their content inside it.

Place workspace identity, search, and a small create-chat icon in a 44-pixel header. Proposed destinations are Chat, Notebooks, Plugins, API keys, and Usage. Group destinations into Workspace, Conversations, and Tools; each group has a clickable disclosure header. Recent conversation links illustrate navigation within chat. The sidebar bottom stays empty. Labels represent design intent, not completed features or authorization policy.

Selected navigation uses a quiet neutral surface with white text and icons; unselected rows use secondary gray. Navigation rows are 28 pixels high with 13-pixel Inter labels, 14-pixel icons, and a 6-pixel icon-to-label gap. Keep lightly rounded surfaces. The header create control replaces the oversized primary pill. Use Uxercon Duotone assets for navigation and workspace controls where the set provides a suitable symbol. Root category headings use 10-pixel text. Align first-level icons with the 12-pixel root category inset, including selected rows. Indent only children of an expanded folder. Bind Auto Layout gaps and padding to the existing spacing scale.

## Consequences

A stable shell gives features a shared navigation model without requiring their content screens to be designed together. The sidebar consumes horizontal space, so a compact option may be useful on smaller desktop windows. Mobile work remains deferred.

Workspace switching must follow [organization membership rules](../organization/adr.md). Visibility and action permissions remain a separate product decision; a mock sidebar does not grant access or establish new roles. Search scope and conversation ordering are unresolved.

## Review

Review workspace recognition, label hierarchy, selected and hover states, compact navigation, and alignment. Keep only base controls on Design foundations. The complete sidebar and feature compositions belong on dedicated pages. See [Sidebar](sidebar.md) and the [icon selection library](icons.md).

## Shared control exploration

Decision status: The Linear control styling and GitHub form hierarchy are accepted as sources for this design review. Application adoption remains proposed. Implementation status: Shared controls are retained; page compositions and complex assemblies are organized on dedicated feature pages. Design foundations contains only base components and visual constants.

Use the reviewed Compallm sidebar, not the captured Linear navigation. Keep it as one complete component family with expanded and collapsed states. Other controls derive from the reviewed Linear geometry; forms use GitHub's numbered section hierarchy. Shared fields retain Linear colors, 0.5 px borders, and 8 px corners. Dropdowns show closed triggers and expanded option lists. Login and signup use the same email-field, six-digit code-entry, button, prompt, and feedback bases. Main uses shared sidebar and provider-form components. Verify component links and master-to-instance propagation, not just matching styles.

Dedicated Forms and Preferences pages hold the settings and application compositions. The Page patterns gallery is removed; full pages are compositions of bases, not foundation components. The captured diagram remains an image preview; controls and text are editable. The [control guide](controls.md) records the geometry and source boundaries. Keep original H2D files and local comparisons as evidence without recreating a second reference gallery. Application business rules, authentication interactions, mobile work, and React/API integration remain separate. The icon catalog is unchanged.

The two expanded-dropdown captures establish seven theme choices with swatches and four diagram styles. The overview export supplies twelve views, including project/list states, billing, agents and connector selection, document templates, and reviews. Page examples combine these bases with our sidebar. Captured prices, features, and integration names are reference content; they do not establish Compallm product commitments. Coverage applies to the supplied views, not every unobserved Linear interaction.

The [Add provider design](add-provider.md) has two pages: a minimal searchable model catalog with a transparent table, compact toolbar filters, and a secondary custom-configuration path, then editable configuration with known defaults filled in. The second page retains numbered Name, Connection details, Model, and Test connection sections. Base URL and endpoint mode occupy separate fields. Token prices use USD per million tokens; other modes retain their actual billing units. Shared foundation components and the spacing policy govern both pages. LiteLLM informs catalog metadata; catalog availability, adapter coverage, testing behavior, and runtime integration remain separate decisions.

The [Create model design](../llm-router/custom-model.md) composes foundation controls on its dedicated page. A custom model uses an ordered list of configured provider/model pairs, with small observed serving-time percentages and down/inactive states. Drag handles and insertion indicators are atomic bases; the route editor stays with the feature. Retry and failure policy remain unresolved.
