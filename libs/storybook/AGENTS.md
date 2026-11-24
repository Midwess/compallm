# Svelte component library

This is the official Svelte UI library, published as `@compallm/storybook`. This package supplies reusable Svelte components and offline previews; it must not
import Portal source, contact live APIs, authenticate, or add feature operations without an explicit feature task.

Use Svelte 5 props, events, snippets, context, and element bindings. Retain familiar compound names and the existing design,
semantic HTML, accessibility, loading, keyboard, and focus contracts. Keep Tailwind utilities and the six shared palettes;
production scanning excludes stories and preview utilities. Selective icon and language imports keep unused modules out.

Consumers create stable controllers and supply typed Svelte context. Controllers own domain operations, confirmed state,
and navigation; Houdini owns confirmed GraphQL data. Forms own editable values and errors through NativeFormControl and
the shared save coordinator. Compose Rust-generated schemas/constraints from @compallm/contract/validation. Never duplicate
API models or validation rules. Preserve commit modes, partial patches, queued snapshots, versioned response reconciliation,
retry/reset, independent scopes, and cancellation. Detaching controls never flushes a write.

Use Bits UI through shared wrappers for composite widgets and native dialogs for modal focus. Floaters belong inside the
active dialog boundary. Preserve event cancellation, disabled restrictions, reduced motion, and focus restoration. Initial
loading uses inert placeholders and shared delayed presentation; refresh retains drafts, focus, confirmed values, and scroll.

Render all loaded Table/List rows in normal page flow. Do not add TanStack, virtualization, spacers, or a new scroll owner.
Use shared guarded near-bottom paging plus accessible Load more/Retry. Controllers retain cursor, cancellation, permission,
and stale-response responsibilities. Keep options lazy and application processing cancellable through shared scheduling.

Use Node 24.13+. Validate formatting/lint, Svelte/TypeScript checks, the library build, static Storybook build, and consumer
builds. Inspect previews manually. Add no unit, automated browser, interaction suites, or Storybook test runner. Do not
activate desktop applications or browser windows/tabs. Follow the repository writing skill for comments and prose.

Lint every authored `.svelte`, `.svelte.ts`, and `.svelte.js` file, including stories and preview configuration, with the
Svelte recommended preset and rune-aware Svelte parser. Keep generated outputs and dependencies excluded. Compiler warnings,
unused suppressions, and debug directives must fail checks. Prettier owns formatting through prettier-plugin-svelte; retain
the Svelte Prettier preset to avoid conflicting lint rules. Use `lint:check` for read-only verification, `lint:fix` for available
ESLint fixes followed by formatting, `format` for formatting only, and `check` for Svelte/TypeScript diagnostics.
