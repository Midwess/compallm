---
title: Portal controls
available: false
---

# Portal controls

Design proposal. Penpot Design foundations owns the reusable component families and shared visual rules. Controls use reviewed Linear styling, forms use GitHub's hierarchy, and navigation retains our Compallm sidebar. The canvas does not establish application behavior or authorize integration.

The [shared spacing policy](spacing.md) supersedes the original capture spacing below for current component gaps and padding: halve nominal spacing with a 2 px nonzero minimum. The measurements remain source evidence. Control sizes, typography, borders, and corners retain their established values.

## Reference and scope

The component library uses the reviewed Linear and GitHub H2D sources, the existing Compallm sidebar, and the accepted login/signup designs. The captured Linear sidebar is removed from the library. Earlier inspection of [Linear settings](https://linear.app/devlogs/settings) on 2026-10-28 also covered Profile, Notifications, Code & reviews, Agent personalization, and Members. The following **20-family research inventory** describes that broader inspection; it is not a count of Penpot assets or Linear's internal components.

| Family | Reference and mapping |
| --- | --- |
| Button | Compact 32 px settings actions; retain the existing 48 px authentication action separately. |
| Icon button | 24–32 px edit, help, and overflow controls; give each an accessible name. |
| Switch | 30 × 20 px track, neutral off and violet on; pair with a persistent label. |
| Checkbox | Member selection, distinct from an immediate setting switch. |
| Text input | Profile inputs are 32 px high with 8 px corners and 13 px text. |
| Search field | 28 px sidebar search and 32 px member search; leading icon and clear affordance. |
| Multiline editor | Agent guidance; preserve label, helper, and a visible focus boundary. |
| Select trigger | Usually 30 px high, 8 px corners, 13 px text, trailing disclosure. |
| Listbox | Attachment-format options are 32 px high; selected option carries a checkmark. |
| Popover | Local floating menus keep secondary controls near their trigger; modal behavior needs separate validation. |
| Settings row | Label and muted description at left; value or control at right; subtle row separators. |
| Section header | 15 px medium heading with contextual description and generous separation. |
| Navigation item | Compact grouped sidebar links; reuse the existing Main sidebar system. |
| Avatar / upload | Profile identity and picture control; use fictional initials in examples. |
| Badge / status | Member roles, online state, and counts use quiet secondary emphasis. |
| Data table | Member list pairs a filter toolbar with aligned sortable columns and row actions. |
| Empty state | Agent skills explain the absence of items without filling the page with decoration. |
| Callout | Disabled agent MCP access explains the restriction and links to configuration. |
| Keyboard key | Code workflow helper text gives Option/⌥ a distinct inline treatment. |
| Code / diff viewer | Preferences includes a real unified diff preview, language selector, font controls, line numbers, and syntax highlighting. |

## Control rules

### Component-family library

Design foundations is organized into base-component families: text fields, buttons, dropdown controls, selection controls, feedback, content/code primitives, and small navigation/form units. Complete sidebars, full forms, feature panels, and screens belong on dedicated feature pages. Do not create a Page patterns gallery. Keep color, typography, and spacing foundations compact and separate from those families. Remove superseded galleries after their live uses have been migrated. The imported Linear sidebar is a reference, not our navigation component; use the reviewed assembly from the Sidebar page.

Make reusable Penpot components with named variant properties. Fields include compact single-line, email, search, multiline, and code-digit types, with empty/filled, focus, error, and disabled states where applicable. Dropdowns include closed triggers and expanded option lists, including theme and language forms. Buttons retain compact and authentication sizes. Selection controls cover switch and checkbox states. Feedback, inline errors, empty states, popovers, tooltips, keyboard hints, settings rows, avatars, badges, and code/diff content use their corresponding families.

Login and signup must use instances of these shared bases, preserving their desktop composition, text, field dimensions, entered code, focus/error treatments, and documented interaction intent. Form compositions also reuse the field, dropdown, button, and selection components. Validate actual component links and a representative master-to-instance change; matching colors alone is not component reuse. No React or API integration is authorized by this canvas cleanup.

### Source-based form compositions

The dedicated Preferences and Forms pages hold the settings and application compositions. Preferences derives from Linear's H2D source. Create application follows the supplied GitHub new-repository form structure while retaining Linear text-field styling. Keep the original captures as evidence; the separated component families replace the mixed reference board. The Inbox export contains only navigation and does not establish a message-pane design. Save a named version before restructuring. A file-validation pass alone does not establish visual fidelity.

The [Add provider pattern](add-provider.md) starts with an expanded searchable model catalog and a custom-configuration option. Continue opens a second page with editable known defaults and an empty API key. Base URL and endpoint Mode are separate fields. Foundations owns form labels, step markers, connectors, fields, and actions. Feature-specific introductions and progress assemblies stay with the relevant feature page. Those bases compose transparent catalog rows, compact toolbar filters, mode dropdowns, checkboxes, helpers, and buttons. Feature pages compose instances of the base controls; the configuration retains Name, Connection details, Model, and Test connection sections.

The model-catalog table has no independent surface fill: its header, grouping labels, and default rows inherit the page visually. Retain hover, focus, and selected-row treatments. The current chooser removes featured cards and the permanent filter rail. Its 28 px toolbar actions and pill tabs adapt `linear_app_devlogs_team_DEV_all (1).h2d`; the on-demand provider/mode/capability popover is our composition of shared controls.

### Expanded menus and overview coverage

Dropdown triggers use the same captured 0.5 px white shadow edge (14.3% opacity) and small 30% black depth shadow as Linear's neutral controls. Their actual half-pixel border is transparent outside focus states. Floating menus use a real 0.5 px quiet gray border and the captured three-layer 12.5% black shadow (vertical offsets 3/2/1 px with 8/5/1 px blur). Apply these through shared dropdown surface tokens to standard, theme, language, protocol, model-catalog, and endpoint-mode controls. Preserve existing control dimensions, menu corners, selected-row styling, disabled opacity, and explicit focus rings. These border/shadow dimensions do not inherit the 2 px spacing minimum.

The additional `linear_app_dropdown_expand_1.h2d` and `linear_app_dropdown_expand_2.h2d` captures establish the theme and diagram list content. Theme has seven choices with individual preview swatches and a trailing selection check. Its captured list is 201.51 × 232 px; diagram style is 124.05 × 136 px with Default, TUI, Hairline, and Hand-drawn. Both use 32 px rows, 4 px vertical list padding, a 12 px leading inset, and reserved trailing selection space. These exports contain the list body, so the surrounding floating surface must be identified as an adaptation rather than measured from these two files.

`linear-overview.h2d` contains twelve captured views, including alternative states. The library extends the existing families with focused navigation, data/property, card/collection, editor/review, and page-pattern frames. Input and editor masters remain together in Text fields; their compositions use instances. The captured anatomy is mapped as follows:

| Source views | Missing bases to cover |
| --- | --- |
| Project overview | Breadcrumbs, compact tabs, page toolbar, inline properties, property rows/panel, resource action, milestones, activity event, and agent dock. |
| Project list and detail-open state | View pill, filter/display/detail controls, column header, project row, health/progress indicators, and table composition. |
| Billing, Loops, AI & agents | Plan summary, upgrade/feature list, usage link, entitlement banner, and navigable integration rows with enabled, setup-required, and restricted states. |
| Personalization and connector-open state | Guidance editor, edit status, empty collection/action rows, search/list header, connector rows with metadata or submenu, and clipped scrollable picker. |
| Documents and new template | Empty template collection, title/body editor surfaces, document icon, back navigation, and action footer. |
| Reviews | Review introduction/actions, feature tile, guided-review step, changed-file summary, and review preview shell, alongside the existing diff bases. |

Use editable components and named variants for these captured states. Compound examples reuse the new bases rather than duplicating independent shapes. Keep the existing Compallm sidebar, authentication sizing, GitHub form step line, and shared Linear field rule. Empty inline editors may show clearly identified illustrative placeholders so their bounds are reviewable; they do not establish new product fields. Captured plan prices, integrations, and project/review content are reference samples, not Compallm commitments. This coverage concerns the supplied views, not every page or hidden interaction in Linear. React behavior, keyboard implementation, and backend integration remain separate.

Use complete Inter faces for shared text, retaining 400/450/500 treatments from the source. A font subset made from earlier settings text can omit glyphs used by new headings. The scaled review illustration is adapted into a readable composition using shared changed-file, guided-step, and code-preview bases. Its explanatory copy and abbreviated paths are illustrative; the original screenshots remain available for comparison. SVG exports with embedded captured fonts provide a portable review artifact when Penpot's PNG rendering service is unavailable.

### Form layout and shared text-field styling

Use GitHub's clearer form hierarchy for Create application: numbered section headings connected by a continuous vertical step line, persistent labels above controls, helper text below controls, and consistent vertical field spacing. The captured step line is 2 px wide with `#3D444D` at 70% opacity; align it with the number centers and draw it behind the markers through the final section. The inspected GitHub labels use 14 px type with a 21 px line box, 4 px between label and input, and 16 px between field groups. Its 16/24 px section headings separate general information from configuration. Adapt the layout to the existing 640 px review column and retain the application fields and sample content; do not import GitHub repository business rules or infer new required fields.

All portal text-field surfaces use the Linear reference treatment: background `#1A1A1B`, border `#48494D` at 0.5 px, 8 px corners, and text `#E4E5E7`. Compact forms use 6 × 12 px input padding and 32 px single-line controls; multiline fields retain an appropriate taller size. Use shared field tokens for this treatment across text inputs, textareas, and search fields. This changes field styling rather than the overall Compallm palette. Existing authentication layout, padding, and control heights remain governed by their feature design, with purple focus and coral error states preserved. Application code adoption is separate.

The captures supersede earlier measurements: standard settings rows are 65 px high with 16 px padding, a 12 px control gap, and a 3 px label/helper gap. Section headings use 15/23 px type with 48 px section separation and 16 px between the heading and its first group. Groups have 10 px corners, a half-pixel edge, and 12 px between related groups within Interface. Application inputs are 32 px high; textareas are 64 px high. Both use 8 px corners, 0.5 px borders, and 6 × 12 px padding.

The switch uses a 30 × 20 px track, 14 px thumb, and 3 px inset. Selects are 30 px high with 8 px corners and a 10 × 5 px disclosure. Helper text uses Inter Variable at weight 450; titles use 24/32 px, weight 500, and −0.16 px letter spacing. Preserve the captured appearance rows, collapsed code-font row, code preview, and diagram preview under Interface. Installed capture font families remain scoped to reference work; general Compallm UI typography remains Inter. Preserve unsupported capture properties as source metadata instead of claiming an exact round trip.

For the reusable Compallm library, use existing color roles. The compact desktop scale complements the larger authentication controls. Bind layout padding and gaps to spacing tokens. Inputs, selects, switches, and buttons need default, focus, and disabled designs; errors must include text. Hover, focus, error, and loading samples are Compallm proposals unless a reference was directly inspected. Do not imply these states were all exercised in Linear.

### Button surfaces

Use Linear's captured surface treatment on shared button masters. Neutral filled actions use a 0.5 px white shadow outline at about 14% opacity plus a black 30% drop shadow (`x = 0`, `y = 0.5`, blur 1, spread 1). The actual border is transparent; the outline comes from the shadow. Purple primary actions use that small black shadow with a transparent 0.5 px border. Raised icon and destructive actions use the same neutral surface and edge, retaining their semantic foreground. Ghost and text-link actions use the transparent treatment. Preserve the explicit focus ring and attenuate the same treatment through disabled opacity.

Apply this across compact, standard, authentication, loading, hover, focus, disabled, and content-sized button samples through shared bases. Shadow/border dimensions are geometry and do not scale with the spacing density. A shared pill-radius token uses Linear's large-radius rule, visually clamping to half the current control height when instances resize. Preserve edited control dimensions, including the revised neutral base. The reusable Content button family retains the earlier 166 × 28 px sample. The current provider footer follows the user-edited selected frame with a 142 × 28 px neutral **Use custom provider** action.

The H2D preview uses the captured Berkeley Mono face at approximately 12 px with 20 px rows. Preserve its line numbers, syntax colors, row tints, and inline highlights. Keep reference styling separate from Compallm form-error colors. The diagram is a faithful captured preview image; the rest of the reference uses editable text, boards, and vector icons. The inspection covers the settings preview; full review, commenting, split view, and conflict flows remain unverified.

Keyboard behavior is an implementation requirement: Tab reaches controls, Space operates switches and checkboxes, arrows move through options, Enter selects, and Escape dismisses a floating panel and returns focus. The Penpot samples illustrate visual states; they do not implement those interactions. Preserve current settings when inspecting references; do not activate integrations or submit forms merely to explore a state.

## Open choices

Compact control adoption, final error and diff colors, code font, responsive settings rows, and touch target expansion require review. A switch may save immediately or belong to an explicit Save form; that business behavior must be decided per feature rather than inferred from this reference.
