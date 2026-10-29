# Linear capture references

These local references were reconstructed from the supplied `.h2d` exports. They are historical source comparisons for the current Penpot component library, rather than previews of its latest organization.

Open **preferences.html** or **applications.html** for the browser reference. Both are self-contained and embed the font files from the capture. The matching PNGs are browser-rendered previews. The HTML uses fixed captured positions, so it is a visual comparison rather than application component code. The SVGs contain outlined body text shaped from those fonts and can be used as portable vector comparisons; they do not provide editable text. Nested SVG artwork and shadow rendering can vary between importers, so use the HTML and PNG for visual comparison. Browser checks in `verification.json` confirm the capture dimensions, select geometry, helper weight, loaded fonts, and zero external requests.

The files show the captured 640 px content column. The original capture coordinates and the 1496 × 1079 viewport are recorded in `measurements.json`; the browser sidebar and app shell were outside the supplied selection. The JSON capture files retain the node hierarchy, text, and styles while omitting embedded font bytes already contained in the HTML/SVG.

## Corrections established by the capture

| Detail | Captured reference |
| --- | --- |
| Switch | 30 × 20 track; 14 × 14 thumb; 3 px inset; 42 × 32 hit area |
| Select disclosure | 10 × 5 px, with the captured path |
| Helper typography | Inter Variable at weight 450 |
| Title | 24/32 px, weight 500, −0.16 px letter spacing |
| Groups | 10 px corners, half-pixel edge, 12 px between groups within Interface |
| Rows | 16 px padding, 12 px between copy and control, 3 px label/helper gap |
| Appearance | Separate System preference, Light, and Dark rows |
| Code preview | Berkeley Mono, actual syntax spans and line-number positions |
| Font settings | Collapsed 60 px row in this capture; the previous demo showed an expanded state |
| Diagram | Captured 267 px group with the hand-drawn preview |
| Application form | 32 px inputs, 64 px textareas, 8 px corners, 0.5 px border, 6 × 12 px padding |

## Historical combined reference

Open **combined-frame.html** for the three captures together at their captured sizes. **combined-frame.png** provides the full overview, and **combined-frame-manifest.json** records source hashes and panel positions. The Inbox capture is navigation only; no message pane is invented.

The mixed **10 · UI reference / Linear + GitHub** frame has been replaced by fourteen clean library and pattern frames on Penpot **Design foundations**. The original nine families are extended with Navigation & toolbars, Data & properties, Cards & collections, Editors & reviews, and Page patterns. The complete Compallm sidebar is retained in one family; captured Linear navigation and superseded galleries are removed. Login, signup, and Main use shared component instances. The Form patterns examples retain editable capture-derived controls and text; the diagram remains a captured preview image.

The source `.h2d` files remain in Downloads. No application behavior or database configuration changed.

[component-library-manifest.json](component-library-manifest.json) records the initial nine-family pass and its propagation checks. The subsequent overview additions are documented in [overview-captures/source-manifest.json](overview-captures/source-manifest.json), with all twelve captured views, the two expanded-dropdown sources, and editable source excerpts. These captures are reference evidence; the current Penpot masters and linked compositions are the design library.

The overview pass recorded 192 masters in 29 variant groups. See its [previews and verification](overview-captures/README.md) for dropdowns, fields, five additional frames, and composed page examples. The later [Add provider redesign](../add-provider/README.md) adds shared form anatomy, protocol dropdowns, and thinking-level controls, bringing the library to 211 masters and 33 variant groups.

## Application form adaptation

The Create application pattern uses the GitHub new-repository H2D hierarchy and continuous numbered step line with Linear field colors, borders, and 8 px corners. Open **application-form.html** for the local adaptation. **applications.html** remains the original Linear capture for comparison. Local combined previews show that adaptation; the current canvas uses shared component instances while preserving focus/error states and authentication sizing. See [Portal controls](../../docs/content/portal/controls.md) for the current design rules.
