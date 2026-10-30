---
title: Shared spacing policy
available: false
---

# Shared spacing policy

Decision status: Accepted for the Penpot library, screen designs, and shared Tailwind foundation. Implementation status: Penpot tokens and their shared CSS translation are implemented. The foundation reference shows resolved values. Canvas and code do not synchronize automatically.

All shared component and composition gaps, padding, and margins follow the same constants. The current density halves the previous spacing, with a **2 px minimum for every nonzero spacing value**. Zero remains zero. Control dimensions, type sizes/line heights, icon artwork, borders, and corner radii are separate and retain their values.

## Constants

The **Spacing policy** token set owns `layout.spacing.scale = 0.5` and `layout.spacing.minimum = 2`. Each nominal spacing token resolves as `max({layout.spacing.minimum}, nominal * {layout.spacing.scale})`; `layout.spacing.0` resolves to zero. Existing `ui.space.*`, `space.*`, `form.*`, field-padding, and capture-spacing names remain compatible aliases of this policy, avoiding repeated halving through references.

| Previous nominal spacing | Current spacing |
| --- | --- |
| 0 | 0 px |
| 1, 2, 3, 4 | 2 px |
| 6 | 3 px |
| 8 | 4 px |
| 12 | 6 px |
| 16 | 8 px |
| 24 | 12 px |
| 32 | 16 px |
| 48 | 24 px |
| 64 | 32 px |

Bind Auto Layout properties to tokens on masters. Instances inherit those bindings; screen-specific layout overrides also use the constants. Reflow composed forms and editor/card examples so their outer spacing follows the reduced gaps. Keep the foundation galleries legible with token-based insets and separation, and show the resolved values in the spacing reference. Original H2D screenshots remain historical source evidence.

## Tailwind use

The shared `portal/src/styles/theme.css` configures Tailwind v4. `--layout-spacing-scale: 0.5` and `--layout-spacing-minimum: 2px` mirror the policy constants. Named theme tokens follow Penpot's nominal suffix: `layout.spacing.16` becomes `--spacing-space-16`, resolving to 8 px. Use `gap-space-16`, `p-space-24`, and `mt-space-32` for 8, 12, and 16 px respectively; `p-0` remains zero. Names describe the original nominal scale, not the resolved pixel value.

The default numeric Tailwind spacing scale is disabled. Control dimensions use explicit sizes such as `size-[14px]` and `h-[32px]`; they must not inherit density changes. Borders and corner radii also remain independent. The named spacing tokens cover the current Penpot scale, including intermediate values already used by existing layouts.

## Agent editing

An agent can change the shared scale and minimum in Penpot through MCP, refresh the active token sets, and check the resolved values and representative instances. Save a named version first. In the deployed API, changing `value` alone does not refresh existing layouts; toggle the active policy set off and on after the edit. Apply a token only when the shape's property does not already reference it, since applying an existing binding can toggle it off.

Audit Design foundations and affected feature, sidebar, login, and signup pages after changes. Check unbound spacing, values below the floor, component links, composed-layout overflow, and fresh exports. A temporary propagation test must be restored before saving the final version. These instructions define a design workflow; they do not implement an in-product agent feature or automatic canvas-to-code synchronization.
