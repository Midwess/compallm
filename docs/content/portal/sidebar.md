---
title: Portal sidebar
available: false
---

# Portal sidebar

Design proposal, desktop only. Main content, routing, and backend behavior are outside this design task.

The [shared spacing policy](spacing.md) supersedes the preceding spacing measurements below: halve gaps and padding with a 2 px nonzero minimum. For example, the 6 px icon-label gap resolves to 3 px and 8 px between groups resolves to 4 px. Sidebar width, row heights, type sizes, icons, and corners remain separate geometric values.

## Composition

Use Linear’s compact proportions: a 244-pixel sidebar, compact header, 28-pixel navigation rows, 13-pixel labels, and 14-pixel navigation icons. The width is independent of the authentication screen’s golden-ratio split. Use the [Phosphor Regular masters](icons.md) on Design foundations for navigation, search, and add controls. Section disclosure uses a small filled triangle. Set a 6-pixel nominal icon-to-label gap. The workspace mark, workspace dropdown, and Models folder icon are 10 × 10 pixels. Root category headings use 10-pixel type. Section disclosure is an 8-pixel filled triangle. Category dimensions come from shared tokens, rather than individual SVG overrides. The orange workspace mark remains the product identity.

Use a 12-pixel root inset from the sidebar edge. Align first-level navigation icons with root category headings at x = 12 pixels, including selected rows; selection must not shift their contents. Keep navigation text in one left-aligned column. Root category labels sit above their items. Folder disclosure markers precede folder labels. Rows sit together without large gaps.

The header contains a small workspace mark, workspace name and dropdown, plus search and create-chat icon controls. Avoid a large New chat pill. Leave the sidebar bottom empty; Settings and account identity are omitted from this layout.

Root categories use quiet 10-pixel headers. The proposed order is Providers, Routers, Workspace, Conversations, and Tools.

- Providers: an expandable category containing sample providers OpenAI, Anthropic, and Gemini. Keep the disclosure arrow beside the title and a plus icon at the far right; the plus shows the tooltip “add provider” on hover.
- Routers: an expandable category with Stats, API key, and an expandable Models folder. Keep its disclosure arrow beside the title and a far-right plus button with an 8-pixel icon, matching the disclosure size. Models inside the folder are sample custom names that a member added to that router.
- Workspace: Chat, Notebooks, Plugins.
- Conversations: sample recent conversation titles; ordering and retention remain unresolved.
- Tools: Usage; access rules remain unresolved. The Compallm API key entry belongs to Routers.

Each section collapses independently. Its filled triangle follows the label, pointing down when expanded and right when collapsed. Use a shared 8-pixel Disclosure icon token, smaller than the 13-pixel navigation icons. Hiding its rows moves later sections upward. Keep the header visible and clickable across its row. A future application must expose expanded state and the controlled group to assistive technology, support Enter/Space, and show keyboard focus. The Penpot Sidebar and Add provider pages illustrates root and nested expanded states; it does not simulate every state or store preferences.

The Sidebar and Add provider pages illustrate two disclosure levels. Items expanded directly under a root category keep the same item alignment. A folder row also stays at that alignment; only its expanded children receive an additional 4-pixel (`space.1x`) left indent. Root category headings use 10-pixel type, while folder names and navigation labels use the 13-pixel Label style. The Models folder contains sample model identifiers `gpt-4o-mini` and `claude-3.5-sonnet`. Provider and model names are illustrative. The API key belongs to Compallm’s Gateway, not an upstream provider credential. The exact provider list, router lifecycle controls, and policies remain unresolved.

Reuse the [dark neutral/violet palette](colors.md) from Design foundations. Use a quiet neutral selected surface with white text and icons; unselected navigation uses `color.muted`. The selected surface extends 4 pixels beyond each side of the regular row and uses 10-pixel corners. Its icon stays at the same 12-pixel inset as other root items and its label stays in the same text column. Keep the small orange workspace mark. Avoid oversized purple action surfaces. The sidebar uses `color.background`; the content uses `color.surface` so it reads as a raised panel. The sidebar navigation area and content start 16 nominal pixels from the top and reserve 32 nominal pixels at the bottom. The content has a 16-pixel nominal right inset, a `color.border` edge, and 12-pixel corners.

The current spacing baseline uses `space.0.5x = 2 px`, `space.1x = 4 px`, `space.2x = 8 px`, and subsequent doubling steps. Root rows use 4-pixel leading and trailing padding, placing their icons at the same 12-pixel inset as category headings. Use a 6-pixel icon-to-label gap and 1-pixel spacing between adjacent navigation rows. Keep 8 pixels between root categories. A folder’s expanded children receive a 4-pixel additional inset.

## Typography foundations

Use shared Inter text styles from Design foundations: Sidebar / Label (13 px, regular), Sidebar / Selected (13 px, bold), Sidebar / Section (10 px, regular), and Sidebar / Workspace (12 px, bold). Labels fit their text height and center vertically inside the fixed 28-pixel row. Navigation icons are 14 pixels with a 6-pixel gap to the label. Category headers use a fixed 28-pixel row with centered contents. Keep text and icon sizes as named shared tokens. Selection changes weight and color without changing Label size. Root section labels sit above their items; folder disclosure markers precede the folder label and point down when open or right when closed.

## Review constraints

The shared layout assembly keeps its content canvas empty. The separate [Add provider composition](add-provider.md) uses that base with shared form components. The sidebar illustrates expanded root and folder states; it does not run search, create conversations, or switch organizations.

Retain our Compallm sidebar instead of the imported Linear navigation. Consolidate it into one complete Sidebar component family on the dedicated Sidebar page, with expanded and collapsed navigation states. Keep workspace identity and navigation sections as meaningful internal groups, without separate top-level galleries for every row or add button. Feature screen compositions use instances of this assembly. Design foundations keeps only the underlying base controls. Shared button, dropdown, and text-field families remain in their own frames. Save a named Penpot version, validate the file, and reopen affected pages before reporting completion. Mobile work remains deferred.
