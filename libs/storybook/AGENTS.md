# Portal design workspace

Read the root agent instructions and `docs/content/portal/adr.md` before component work. Keep user prompting in Codex CLI. Use
the local Storybook MCP tools to inspect available components and stories when connected; source files remain authoritative and
can be edited directly.

- `npm start` launches the workbench at `http://localhost:6006`, with MCP at `/mcp`.
- Base-component previews live under `Components`, organized by Penpot Design foundations family. Keep feature screens separate
  and use sample data. Do not connect prototypes to APIs, authentication, Rust, or databases unless the user authorizes that
  work.
- This npm library owns all reusable components in `src/components/` and page compositions in `src/pages/`. Preview each in
  Storybook first. The portal consumes package exports later; never import portal source.
- Preview demos and sample data belong in `stories/`, grouped under Components or Pages. Reusable page implementations belong in
  `src/pages/`.
- Main design: dark neutral surfaces, violet primary actions, Inter typography, and Phosphor Regular SVG icons. Reuse the shared
  Tailwind v4 theme in `src/styles/theme.css`.
- Add meaningful variants for loading, empty, disabled, and error states where relevant. Keep semantic HTML, keyboard focus, and
  responsive behavior.
- Use Tailwind utilities directly in components, pages, and previews. Do not add custom CSS selectors, component stylesheets, or
  `@apply` wrappers. CSS files are limited to Tailwind entry points, theme configuration, and font imports.
- `npm run build` builds the optimized UI package. `npm run build:storybook` builds the separate preview site. Exclude stories
  and tests from the production Tailwind scan.
- Run `npm run lint`, `npm run check`, `npm run build`, `npm run build:storybook`, and relevant browser checks. `npm run lint`
  applies lint fixes and formatting; keep source lines at most 128 characters. Do not run the portal's backend-backed build
  merely to verify stories.
