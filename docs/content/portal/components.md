---
title: Portal base components
available: false
---

# Portal base components

Decision status: React bases and Storybook previews accepted on 2026-10-31. Implementation status: Translation from Penpot is authorized; production feature adoption remains separate.

`portal/src/components/ui/` owns reusable React bases corresponding to **Design foundations** in Penpot. Group source files by family and import the required components directly. Storybook imports the same code under **Components**, with one family per sidebar group. Do not recreate the removed application wireframes or add backend dependencies.

| Family | Bases |
| --- | --- |
| Text fields | Text, multiline, search, email, code digit and code entry; guidance, inline title/summary, description, document title/body editors |
| Buttons & links | Button, content button, text link, account prompt, resend prompt |
| Dropdowns | Select, dropdown, theme/diagram menus, diagram dropdown, menu heading, select option |
| Selection controls | Switch, checkbox, radio, choice card, checkbox option |
| Feedback & overlays | Helper text, inline/code/registration errors, tooltip, keyboard key, empty state, callout, popover, drop indicator |
| Content & code | Section heading, settings row, avatar, badge, code preview, diff line, member row, timestamp, changed file, document icon |
| Form primitives | Introduction, step marker/heading/connector, field label, labeled field |
| Navigation controls | Breadcrumbs, back breadcrumb, view pill, agent dock, navigation tab, toolbar action, drag handle |
| Data primitives | Project table header/row, project icon/health/progress, resource/milestone actions, activity event, property chip/row |
| Icons | Official Phosphor SVG components imported per icon |
| Section surfaces | Shared section background with composable children |

Use the accepted [colors](colors.md), [spacing](spacing.md), and [icons](icons.md). Keep compact field heights at 32 px, email at 48 px, and code digits at 56 px. Layout gaps and padding use the half-scale spacing variables; control geometry remains independent. Font sizes follow the inspected base, including 13 px compact labels and 12 px helpers. Inter is the UI family.

Expose ordinary HTML attributes and explicit visual variants. Prefer children for icons, actions, and nested content rather than a flag for every composition. Fields associate labels and descriptions through IDs; errors include readable text. Native inputs preserve browser keyboard and form behavior. Dropdown options support arrow navigation, Enter/Space selection, and Escape dismissal; overlays must return focus to their trigger. Native popovers and disclosures provide local presentation without introducing product state.

Stories show normal, disabled, loading, error, selected, and expanded states where relevant. Sample account prompts, resend cooldowns, project rows, and provider labels illustrate the bases: callers own authentication, timers, ordering, persistence, clipboard actions, and drag-and-drop policy. A drag handle alone does not implement reordering. The code preview accepts text; syntax highlighting and full review workflows are separate requirements.
