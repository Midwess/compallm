---
title: Portal icons
available: false
---

# Portal icons

Decision status: Phosphor Regular is the shared UI SVG library, accepted on 2026-10-31. It replaces Pixelarticons and the earlier mixed purchased sets. Penpot's **Design foundations → 11 · Icons · Phosphor** contains editable reusable masters. Reuse those masters rather than importing another copy or creating separate icon-catalog pages.

Use official [Phosphor](https://phosphoricons.com/) geometry. UI icons normally use Regular weight; small disclosure triangles may use Fill when an outlined caret is unclear. Preview icons at their actual size: navigation uses 14 px, compact status indicators use 12 px, and standalone controls may use 16 or 20 px. Set color through semantic foreground tokens. Preserve provider logos and the Compallm workspace mark as branding.

## React and SVG use

Use `@phosphor-icons/react` in the portal and Storybook. The package renders inline SVG and retains Phosphor's MIT license. Import only the required icon through its supported per-icon export:

```tsx
import { MagnifyingGlassIcon } from '@phosphor-icons/react/dist/csr/MagnifyingGlass'

<MagnifyingGlassIcon size={14} weight="regular" aria-hidden="true" />
```

Let icons inherit `currentColor`. Decorative icons use `aria-hidden="true"`; an icon-only button needs an accessible name on the button. Do not use an icon font, bitmap export, or a runtime name-to-entire-library registry. Raw vector exports from the Penpot masters remain available for uses outside React.

Component stories use Phosphor icons from this library. Do not copy the entire SVG catalog into application code; import the icons needed by each base.
