---
title: Portal icons
available: false
---

# Portal icons

Penpot uses an **Icons** index and separate `icons-<set-name>` pages for the purchased collection in `../icons`. It supports visual choice; importing an icon does not replace the reviewed sidebar icons or integrate assets into the application.

Import all distinct SVG icons from the purchased collection, including archive contents. Keep families, styles, and source categories separate. Put each brand or set on one page, such as `icons-huge`, `icons-uxercon`, and `icons-emerald`. Keep the same brand’s styles and categories together on that page. The index contains links and counts, without icon grids. Duplicate archive copies and identical IconJar exports share one library entry. Preserve distinct outline, filled, bulk, and duotone variants. SVGs belonging to full website templates or device mockup pages are not automatically standalone icons.

Convert Illustrator and EPS icon artwork to SVG through a vector-preserving converter. For Adobe XD, export actual icon components and resolve their vector geometry, transforms, paints, and opacity; do not import instruction or license pages as icons. Prefer supplied matching SVG exports over duplicate conversions. Unsupported effects or missing references must remain explicit until conversion is verified. Keep conversions separate from the supplied SVG families and compare representative converted icons with their source previews. Normalize non-zero SVG view-box origins into a zero-origin frame before native import, and check visible artwork size; a correct frame size alone does not prove the artwork was scaled correctly.

Imported icons are individually named native vector groups on category boards, with a source hash on each icon and provenance recorded in its batch and the local manifest. The initial curated selection also has reusable library components. Copy a named icon group from the canvas or create a component when adopting it in a control. Preview at 24 pixels on dark surfaces; actual usage must still be checked at the target size, including the sidebar’s 14-pixel navigation icons. The sidebar uses Uxercon Duotone assets: comment-captions, file-code-01, package, server-01, bar-chart-01, key-01, folder-code-01, package-search, and bracket-plus-01. Recolor monochrome paths for visibility while preserving opacity and geometry. Shared spacing and typography rules remain on Design foundations.

Keep gallery captions in bounded, locked preview layers so labels do not create page-sized SVG text wrappers. These caption layers are separate from the editable icon artwork. Three supplied brand SVGs contain embedded bitmap artwork; their canvas entries use faithful previews, while the original SVGs remain in the export.

Keep purchased source archives outside the repository. The Icons page is the editable browsing artifact; application asset export is a separate task.
