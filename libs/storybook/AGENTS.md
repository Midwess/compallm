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

## Fields and form ownership

Follow [Form ownership](https://github.com/Midwess/compallm/wiki/Portal-form) and
[shared validation](https://github.com/Midwess/compallm/wiki/Portal-validation) when developing editable controls.

- Use the shared React Hook Form integration. Form owns editable values, validation, commit policy, submitted snapshots, queues,
  and response reconciliation. Zustand owns workflow state and shared confirmed entities; never duplicate drafts there.
- Named fields bind to their nearest Form and report changes, blur, and commit intent. Form decides when to call the controller.
  Outside Form, callers retain ordinary value and listener ownership. Fields never call GraphQL themselves.
- Register native text inputs through refs and events without subscribing to their values. Use `useController` for composite
  controls, `useFieldArray` for collection structure, and stable row identities. Compose caller and registration refs.
- Subscribe to named paths and the metadata a component actually displays. Avoid global `watch()`, whole-form subscriptions, and
  rebuilding combined controller snapshots on each keystroke. Keep preview and status observers separate from input trees.
- Preserve server rendering and initial field values without browser globals. Set up browser observers, timers, and listeners
  during lifecycle effects; clean them up on unmount. Programmatic reconciliation and local reset must update native inputs.

## Commit behavior and GraphQL

- Default to explicit button submission. Button-only forms suppress blur, debounce, completion, and implicit Enter commits;
  keyboard activation of the submit button remains available. Other flows explicitly choose change, blur/Enter, debounce, or
  completion behavior. Preserve multiline Enter and IME composition.
- Controllers accept explicit validated snapshots, perform domain/API operations, and return confirmed values. Request the
  changed values in the mutation response so Form can reconcile canonical server state. Follow the root GraphQL conventions and
  freshly fetched Wiki guidelines before changing operations; use typed Genql selections checked with `satisfies`.
- Partial updates submit only the triggered changed fields or commit group. Preserve omission and permit null only where the
  contract allows clearing. Keep independent operations in separate commit groups; one failed group cannot undo another's
  confirmed baseline. Keep one root mutation per request.
- Queue triggered snapshots only. An older response must never overwrite newer typing or submit uncommitted edits. Apply
  returned values to the confirmed baseline and replace visible values only when their edit versions still match.
- Failed operations retain drafts and allow explicit retry with the latest intended values. Never automatically replay
  authentication or uncertain writes. Cancel unsent work on record changes/unmount; never flush because a component unmounts.
- Keep unavailable production operations unavailable. Offline stories consume generated contracts and mock controller results;
  importing contracts does not authorize live requests, database access, migrations, or implementation of missing APIs.

## Contract validation and messages

- Rust API inputs under `api/src/graphql` own validation, normalization, defaults, and messages through Validify and
  `ExportValidation`. Import generated validators, constraints, input guards, and inferred values from `@compallm/contract`.
  Never duplicate business rules, API interfaces, or validation wording in components and stories.
- Supply useful Validify `message = "..."` literals in Rust. ExportValidation carries supported messages through
  `x-compallm-messages` into generated validators. Use separate minimum/maximum rules when their wording differs. Extend the
  exporter/runtime for unsupported rules rather than silently weakening validation or adding a frontend-only fallback.
- Use the official Zod resolver for complete form boundaries. Parse partial patches against generated update schemas and send
  parsed output. Keep cross-field refinements within their complete group; triggering one field does not scope a full resolver.
- Local draft adapters may preserve incomplete numeric text and display labels, then convert to generated wire types. Keep
  normalized submission values separate from newer visible drafts. Parser state must settle even when wire formatting differs.
- Validate on blur or submission; after failure, revalidate the relevant boundary on change. Map issue paths to the correct
  field/group, including collection entries. Each failed revalidation must reopen its error message even if wording is
  unchanged.
- Regenerate contracts with `cargo run -p compallm-api -- generate` after Rust input changes; never edit generated artifacts.
  Register actual missing input types without placeholder operations. Leave unresolved business rules explicit.

## Field error presentation

- Compose `Form.Error` or `FormField.Error` explicitly for each editable field/group. Reuse the shared implementation so page
  code does not add its own positioning, timers, tooltips, or animations. API operation errors use toasts; Cedar parser
  diagnostics remain beside their editor.
- Invalid fields have a red border and a red circular badge containing Phosphor's filled white `WarningIcon`. Keep the icon 12px
  (`size-3`), with 4px padding (`p-1`). Inset the badge from the field border; preserve field dimensions, alignment, and room
  for entered text. Error feedback must not move rows or vertically shift switches.
- The badge itself expands into one red capsule with white text. Reveal outward from the icon to the right; flip left near the
  viewport edge. Show for five seconds on first failure and each failed revalidation, then collapse to the badge. Hover or
  keyboard focus reopens it; Escape dismisses it. Remove the indicator when the field becomes valid.
- Preserve labels, `aria-invalid`, accessible error descriptions, keyboard access, and dialog boundaries. Error feedback must
  not steal focus. Use shared Motion presets and respect reduced motion.

## Practice and verification

- Keep one consolidated Form Practice page for learning commit modes, partial updates, returned values, normalization, delayed
  responses, retained drafts, retry, scoped dialogs, standalone fields, validation, and large-form behavior.
- Reuse offline controller mocks and generated contracts. Preserve the current UI unless a visual change is explicitly asked
  for. Include focused component previews for first display, collapsed/hover/focus errors, and viewport-edge expansion.
- Validate Storybook changes with lint, type checks, builds, and manual inspection. Do not add Storybook unit tests, automated
  browser/interaction suites, play-test runners, or `libs/storybook/tests`. Keep meaningful consumer business-flow tests in
  their application's `tests/` directory, subject to the root testing policy.
- For subscription changes, inspect a large form: native typing must not rerender the form root or unrelated fields, broadcast
  through workflow stores, or validate unrelated boundaries. Describe measured behavior accurately; RHF can still check its
  internal observers. Verify reset, SSR, stale responses, failure recovery, and repeated field commits when affected.
