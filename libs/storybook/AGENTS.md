# Portal design workspace

Read the root agent instructions and the [Portal architecture](https://github.com/Midwess/compallm/wiki/Portal) and relevant
focused Wiki pages in full before component work. Follow the root Wiki workflow for documentation updates. Keep user prompting
in Codex CLI. Use the local Storybook MCP tools to inspect available components and stories when connected; source files remain
authoritative and can be edited directly.

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
- Always consider transition consistency when designing or changing components and pages. Reuse the shared Motion presets and
  helpers in `src/components/motion.tsx` for interaction feedback, entrances, exits, disclosure, and reordering. Keep equivalent
  interactions consistent in timing, easing, and movement. Adjust shared presets centrally; do not introduce component-specific
  timing, CSS animations, CSS transitions, or direct browser animation calls. Respect reduced motion and preserve keyboard,
  focus, and native control behavior, including during interrupted transitions.
- Compose menus and filters from the shared `Popup` parts. `Popup.Item` performs an action and closes, `Popup.RadioItem` selects
  one value and closes, and `Popup.CheckboxItem` stays open for multiple choices. Select fields compose `Select.Root`,
  `Select.Trigger`, `Select.Value`, `Select.Content`, and `Select.Item`; searchable multiple selections use `ChoicePicker`.
  These base components own dismissal, positioning, keyboard navigation, focus, and animation. Pages supply values and domain
  callbacks; never duplicate menu event handling, timers, popovers, or library primitives in page compositions.
- Keep the compact hover-background inset scoped to sidebar navigation items and folders. Workspace header buttons and dropdown
  items retain their full hover area.
- Disabled controls must remain hoverable and expose a tooltip explaining why they are disabled; provide a keyboard-accessible
  explanation.
- Add meaningful variants for loading, empty, disabled, and error states where relevant. Keep semantic HTML, keyboard focus, and
  responsive behavior.
- Use Tailwind’s default spacing scale in half steps for dimensions, spacing, gaps, and offsets: `0.5` is 2px at the default
  root size. Preserve existing design dimensions where possible and review any rounding. Keep functional and visual exceptions
  narrowly scoped with a reason beside the lint suppression. Do not add custom spacing tokens or override `--spacing`.
- Use Tailwind utilities directly in components, pages, and previews. Do not add custom CSS selectors, component stylesheets, or
  `@apply` wrappers. CSS files are limited to Tailwind entry points, theme configuration, and font imports.
- `npm run build` builds the optimized UI package. `npm run build:storybook` builds the separate preview site. Exclude stories
  and tests from the production Tailwind scan.
- Run `npm run lint`, `npm run check`, `npm run build`, `npm run build:storybook`, and relevant browser checks. `npm run lint`
  applies lint fixes and formatting; keep source lines at most 128 characters. Do not run the portal's backend-backed build
  merely to verify stories.

Consumers own page controller instances and wrap pages in the public context from `src/context.ts` using React’s context
provider directly. Pages consume context; do not create providers inside pages or pass controllers as page props. The consumer
owns lifecycle setup and cleanup.

Define all public React contexts once in `src/context.ts` and export them through `@compallm/storybook/context`. Do not add
provider wrapper components or per-page context definition files.
