# Expanded menus and overview references

The three supplied H2D files establish two dropdown list bodies and twelve captured views. [source-manifest.json](source-manifest.json) records their hashes, view titles, source URLs, and component excerpts. [contact-sheet.png](contact-sheet.png) provides the source overview; `overview-0.png` through `overview-11.png` show each captured viewport. The original H2D files remain in Downloads.

The current Penpot library has **192 component masters in 29 variant groups across 14 frames**, including 84 added masters and 10 added variant groups. [library-verification.json](library-verification.json) records frame bounds, component links, page checks, restored propagation values, full Inter faces, and the saved version. It reports no frame overflow, broken component links, or file-validation errors.

| Current library preview | Contents |
| --- | --- |
| [Dropdowns](library-dropdowns.png) | Seven-choice theme menu, diagram styles, closed/expanded dropdowns, connector picker, and option anatomy. |
| [Text fields](library-fields.png) | Existing field states plus guidance, title, summary, description, document, and connector-search bases. |
| [Navigation](library-navigation.png) | Breadcrumbs, tabs, view pill, toolbar actions, and agent dock. |
| [Data and properties](library-data.png) | Project table, property chips/rows, properties panel, milestone, and activity parts. |
| [Cards and collections](library-cards.png) | Plans, usage, entitlement, integration, and empty-collection rows. |
| [Editors and reviews](library-editors.png) | Shared document/guidance compositions, review feature tiles, changed files, guided step, and code preview. |
| [Page patterns](library-patterns.png) | Project overview with our sidebar, project list, billing, and AI-settings compositions. |

Each preview has a matching `library-*.svg` with embedded fonts, exported from the final Penpot design and rendered locally. Penpot's PNG rendering service timed out during verification; these PNGs were rendered from the fresh SVG exports. They are separate from the historical H2D source screenshots.

Theme and diagram list geometry follows the captures; their floating surface is adapted because those exports contain only the list body. The connector picker retains its captured viewport while reusable option rows illustrate metadata and submenu anatomy. Its repeated icon slots are component examples rather than an imported brand-icon catalog. Review prose, abbreviated paths, empty editor placeholders, and adapted page compositions illustrate reusable layout. Captured plan terms and integrations do not establish Compallm behavior or availability. No React or API implementation is included.
