---
title: Portal colors
available: false
---

# Portal colors

Decision status: Dark neutral/violet foundation accepted on 2026-10-31. Penpot's **Design foundations** page and active **Compallm Colors** token set define these roles. The shared Tailwind theme translates them explicitly; canvas edits do not update code automatically.

| Role | Value | Use |
| --- | --- | --- |
| `background` | `#09090A` | Page canvas |
| `surface` | `#121213` | Panels |
| `inset` | `#1A1A1B` | Fields and recessed sections |
| `composer` | `#1D1E1F` | Composer surface |
| `elevated` | `#232325` | Raised controls |
| `menu` | `#28292B` | Floating menus |
| `divider` | `#212224` | Quiet separators |
| `border` | `#303135` | Panel edges |
| `input-border` | `#48494D` | Field boundaries |
| `text` | `#E4E5E7` | Main foreground |
| `muted` | `#98999C` | Secondary foreground |
| `disabled` | `#626365` | Inactive controls |
| `primary` | `#5E6AD2` | Primary action |
| `primary-hover` | `#6D78D5` | Primary hover |
| `focus` | `#8FA6FF` | Keyboard focus |
| `on-primary` | `#FFFFFF` | Primary-action foreground |
| `success` | `#41BA56` | Success feedback |
| `warning` | `#F2994A` | Warning feedback |
| `error` | `#EB5757` | Error feedback |
| `transparent` | `transparent` | Unpainted surface |

Existing flow-only roles remain available for compatibility: `flow-canvas` is `#0D0C12` and `flow-node` is `#181622`. Use general surface roles for new controls unless the flow needs these distinctions. Provider logos and other brand artwork retain their own colors.

## Tailwind use

`portal/src/styles/theme.css` owns the CSS-first Tailwind v4 configuration. Import it after `tailwindcss`. Use `bg-background`, `bg-surface`, `text-text`, `text-muted`, `border-input-border`, and `focus-visible:outline-focus`. Penpot camel-case roles map to kebab-case utility names: `color.primaryHover` becomes `primary-hover`.

Default Tailwind color families are removed so new components use these semantic roles. Color alone must not convey status; include readable labels or symbols. Check foreground contrast on the actual surface, including hover and focus states. A palette does not establish the accessibility of every possible pairing.
