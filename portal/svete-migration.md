# Svelte migration notes

Working notes for migrating Portal and the shared UI library to Svelte while preserving the current design and behavior. Houdini
is the agreed GraphQL direction. Implementation has not started; library choices and package layouts marked proposed or open
require verification. This file is the requested local planning exception; product documentation stays in the Compallm Wiki. It
does not authorize implementing the migration.

## Decisions and boundaries

| Topic                    | Requirement / decision                                                                                                                              | Status                  |
| ------------------------ | --------------------------------------------------------------------------------------------------------------------------------------------------- | ----------------------- |
| Scope                    | Portal and the reusable components, pages, and previews in `libs/storybook` move together toward Svelte.                                            | Planning                |
| Design                   | Preserve typography, artwork, tokens, palettes, dimensions, responsive layouts, interaction states, and animation behavior. No redesign is implied. | Required                |
| GraphQL client           | Adopt Houdini in place of Genql for the target frontend. Migrate typed selection objects into authored GraphQL operations and fragments.            | Agreed direction        |
| Rust ownership           | Rust remains authoritative for the API schema, input validation, normalization, defaults, messages, authorization, and business invariants.         | Required                |
| Framework                | Svelte with Vite can retain Axum hosting. SvelteKit is optional and adds routing/application conventions; it is not required by GraphQL.            | SvelteKit decision open |
| Performance              | Smaller runtime overhead is plausible. No user-visible speedup is established until equivalent interactions are measured.                           | Verification required   |
| Backend scope            | Preserve existing API behavior and available operations. A frontend migration does not implement pending backend features.                          | Required                |
| Protected infrastructure | Do not change DbContext, connection traits/lifecycle, scope handling, transactions, pooling, or RLS setup without separate explicit permission.     | Required                |
| Working tree             | Preserve unrelated work, including current provider/catalog and generated-contract changes.                                                         | Required                |

## Technology and architecture mapping

| Current technology / practice                                                          | Svelte equivalent or candidate                                                                              | What must carry over / opportunity                                                                                                                                      | Status                    |
| -------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------- | ------------------------- |
| React components and JSX                                                               | Svelte components and templates                                                                             | Preserve semantic DOM, public UI responsibilities, and layout; rewrite component implementation.                                                                        | Target                    |
| Zustand React subscriptions and selector hooks                                         | Svelte stores or `$state` / `$derived` in suitable modules                                                  | Subscribe/read only needed state. Vanilla Zustand can bridge temporarily; avoid permanent duplicate stores.                                                             | State API open            |
| React Context with stable controller instances                                         | Svelte context with typed keys/accessors                                                                    | Define public contexts centrally; consumer supplies stable instances at scope boundaries. Keep constructors inert and lifecycle explicit.                               | Target                    |
| Controller constructor injection                                                       | Explicit constructor dependencies                                                                           | Retain narrow URL/auth/organization/permission dependencies; no ancestor traversal, `.parent`, service locator, or component API calls inside controllers.              | Keep                      |
| Workflow stores plus confirmed entity copies                                           | Houdini query stores/cache plus reactive workflow state                                                     | Give confirmed server data one owner. Controllers orchestrate domain workflows; avoid mirroring cache entities into another editable store.                             | Proposed ownership change |
| React hooks and effects                                                                | Svelte reactivity, lifecycle, and explicit cleanup                                                          | Cancel obsolete reads, ignore stale results, clean up observers/timers, and avoid request effects in constructors or module singletons.                                 | Target                    |
| React compound components, render props, `asChild`, refs                               | Svelte components, snippets, context, element bindings, and actions/attachments as appropriate              | Preserve wrapper-free DOM where required, child event cancellation, disabled restrictions, native elements, and focus targets. No mechanical one-to-one API conversion. | Prototype                 |
| React Router                                                                           | SvelteKit routing or a selected Svelte SPA router                                                           | Keep lazy routes, loading/error boundaries, not-found routes, modified-click behavior, deep links, and native history.                                                  | Router open               |
| Root URL controller                                                                    | Adapter over the chosen router                                                                              | One history owner; push workflow transitions, replace typing/filter corrections; preserve query-only controller lifetime and discard guards.                            | Keep contract             |
| React Hook Form and `@hookform/resolvers`                                              | Svelte-compatible form library plus retained/shared commit coordinator                                      | Bindings alone do not replace dirty tracking, field arrays, validation boundaries, queues, or response reconciliation. Candidate library must pass the Form contract.   | Library open              |
| Base UI React                                                                          | Accessible Svelte primitives, with Bits UI as a candidate                                                   | Shared wrappers own keyboard handling, positioning, dismissal, portal containers, dialog boundaries, and focus restoration.                                             | Candidate, unverified     |
| Radix Themes/Skeleton                                                                  | Existing tokens/CSS plus shared Svelte Skeleton/LoadingRegion                                               | Audit actual usage before removal. Preserve placeholder geometry and accessibility; Svelte supplies no equivalent design system automatically.                          | Audit required            |
| Motion and shared animation module                                                     | Shared Svelte animation adapter; evaluate native transitions and reusable framework-independent Motion APIs | Preserve centralized presets, interruption/reversal behavior, reduced motion, and immediate logical dismissal. Do not scatter animation settings across pages.          | Adapter open              |
| Phosphor React                                                                         | Phosphor Svelte package or local SVG wrappers                                                               | Preserve exact artwork, weights, sizing, accessible labels, and selective imports.                                                                                      | Verify package            |
| Sonner                                                                                 | Svelte toast host, with a Svelte Sonner integration as a candidate                                          | Keep the shared Toast appearance, deduplication, safe messages, dismiss/retry behavior, and non-stealing focus.                                                         | Candidate                 |
| Paper shaders React                                                                    | Svelte/browser adapter over reusable renderer                                                               | Retain shader appearance and teardown; inspect current browser renderer before selecting a replacement.                                                                 | Audit required            |
| TanStack-backed collection engine                                                      | TanStack Svelte Virtual or core adapter inside shared Table/List                                            | Keep collection coordination and editing lifetime; keyed each blocks alone do not virtualize rows.                                                                      | Candidate                 |
| Storybook React Vite                                                                   | Storybook Svelte Vite integration compatible with selected versions                                         | Preview the same production components offline; rewrite stories and retain separate package/preview builds.                                                             | Target                    |
| Genql CLI and runtime                                                                  | Houdini compiler and client                                                                                 | Rust-exported schema remains input; generated operation/fragment types replace Genql types.                                                                             | Agreed direction          |
| Tailwind v4, semantic CSS tokens, fonts, SVG assets                                    | Reuse existing assets and Tailwind pipeline                                                                 | Preserve palette-based colors, spacing scale, font files, artwork attribution, production scanning, and compiled CSS exports.                                           | Keep                      |
| Cedar WASM worker, GraphQL-independent utilities, Zod, Shiki, DOMPurify, OGL, confetti | Reuse framework-independent implementations where applicable                                                | Adapt lifecycle/rendering glue; avoid replacing working browser logic just because its host changes.                                                                    | Audit individual imports  |

Svelte provides reactivity, stores, context, bindings, and transitions. Accessible composite widgets, complete form workflows,
virtualization, routing for plain Svelte, and GraphQL caching require additional libraries or application code.

## Rust-first generation and Houdini

| Concern            | Rule / target practice                                                                                                                                                                                                             |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Single entry point | Retain `cargo run -p compallm-api -- generate` as orchestration: export contracts, validate compatibility, generate validators, invoke Houdini, then build/check outputs in dependency order.                                      |
| Schema source      | Juniper definitions under `api/src/graphql` produce `libs/contract/schema.graphql`. No handwritten server schema or parallel TypeScript API model tree.                                                                            |
| Validation source  | Validify and ExportValidation produce `validation.schema.json`, shared Zod validators, constraints, input guards, and inferred FormInput/FormOutput aliases. Houdini does not infer business validation from SDL.                  |
| Compatibility      | Reject metadata absent from SDL and mismatched field sets, enum members, scalar/list shapes, nullability, omission, and defaults. Unsupported rules and scalar mappings fail generation.                                           |
| Defaults           | Preserve Rust defaults, including the existing enum-default encoding normalization. Normalization must not invent defaults.                                                                                                        |
| Frontend documents | Developers author operations/fragments by converting current Genql selections. Rust cannot infer screen field requirements or generate these files into component folders.                                                         |
| Colocation         | Put component fragments beside shared components and feature queries/mutations beside their Portal consumers. Compose component fragments in queries.                                                                              |
| Discovery          | Configure Houdini to scan the relevant Portal and shared UI source folders and read the exported local schema. Validate monorepo support with the pinned release.                                                                  |
| Generated output   | Use one generated artifact/type boundary accessible to Portal and shared UI without importing Portal source. Exact package placement, aliases, and build order remain open.                                                        |
| Separate schemas   | Keep Houdini client-only directives/schema extensions separate from the canonical Rust SDL. They do not add backend capabilities.                                                                                                  |
| Offline generation | No running API, database, migrations, schema polling/pulling, remote calls, or Wiki publication during generation. Node tooling is a local build dependency.                                                                       |
| Type ownership     | Houdini generates API operation/fragment types. Zod-inferred form types describe editable/normalized values; local draft types cover incomplete text and UI state. No retained handwritten API snapshots.                          |
| Scalar fidelity    | Keep IDs as strings, exact prices as decimal strings, and ContextTokens as an exact positive-u32 JSON number. Specify marshal/unmarshal mappings where needed; never fall back silently to `any`.                                  |
| Reproducibility    | Pin compatible Houdini/Svelte/tooling versions. Generation must work from missing outputs, be deterministic, and fail CI on stale tracked artifacts. Decide generated-runtime tracking separately from required tracked contracts. |
| Source updates     | Preserve live local package links for Portal and Storybook. Regeneration and source edits must not require reinstalling copied package snapshots.                                                                                  |
| Convention updates | Explicitly replace Genql-only selection/type rules and generation scripts during implementation. Do not leave contradictory instructions claiming Genql remains the sole generator.                                                |

Proposed dependency direction: Portal imports shared UI and generated GraphQL exports; shared UI imports generated exports.
Generated code must not import Portal application logic or create a cycle back through the UI package. Verify Houdini runtime
requirements before treating the generated boundary as a standalone publishable package.

## Shared components, fragments, and offline previews

| Practice                  | Target contract                                                                                                                                                                                 |
| ------------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| UI ownership              | `libs/storybook` continues to own reusable components and page compositions. Portal consumes public exports; the library never imports Portal source.                                           |
| Basic controls            | Use ordinary typed props and actions for buttons, fields, layout, and other primitives. They do not fetch data.                                                                                 |
| Data component fragments  | Colocated fragments declare required fields. Fragment masking and generated references are distinct from plain result objects.                                                                  |
| Plain-data components     | Generated data projections may type props; stories supply checked fixture objects. Avoid inventing parallel API interfaces.                                                                     |
| Fragment-aware components | If components resolve Houdini fragment references, provide a shared isolated Storybook client/cache harness seeded with fixtures. A type assertion is insufficient.                             |
| Network boundary          | Stories never contact live APIs, authenticate, use production credentials, or require Rust/database startup. Imports of generated types do not authorize network calls.                         |
| Isolation                 | Create/dispose preview state per story as needed; no shared authenticated cache or session singleton. SSR, if introduced, also requires per-request authenticated state.                        |
| Packaging                 | Preserve ESM, declarations, supported subpath exports, compiled styles/fonts/assets, tree shaking, and separate UI-library and static-Storybook builds.                                         |
| Styling                   | Keep Tailwind utilities and semantic tokens. Do not introduce component stylesheets or scoped CSS merely because Svelte supports them. Preserve existing narrowly scoped global browser styles. |

## Forms and mutations

| Existing guarantee | Svelte/Houdini requirement                                                                                                                                                                                       |
| ------------------ | ---------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| One draft owner    | The shared form model owns editable values/errors; Houdini owns confirmed server records. Bindings must not create another draft copy.                                                                           |
| Named field scopes | Fields bind to the nearest registered Form subtree; standalone controls retain caller-owned values/listeners. Fields never call GraphQL.                                                                         |
| Commit modes       | Preserve button, change, blur/Enter, debounce, and completion modes. Default button-only suppresses automatic/implicit commits. Preserve multiline Enter and IME composition.                                    |
| Validation         | Normalize and validate through generated Rust schemas. Validate on blur/submission; after failure revalidate the affected boundary on change.                                                                    |
| Partial patches    | Submit only triggered changed fields or a complete cross-field group. Preserve omission versus explicit null; submit parsed output.                                                                              |
| Queues             | One active request per form; coalesce compatible triggered snapshots only. Never include newer uncommitted typing because a response arrived.                                                                    |
| Reconciliation     | Apply confirmed mutation values to baselines; replace visible fields only when edit versions match the submitted snapshot. Cache updates must not overwrite newer drafts.                                        |
| Failures           | Retain drafts, pause automatic submission after rejection, and allow explicit retry of intended values. Never replay authentication or uncertain writes.                                                         |
| Lifecycle          | Cancel unsent commits on record change/disposal; ignore obsolete responses. Unmount never flushes a write. Keep Blob/File identities and upload data intact.                                                     |
| Virtual rows       | Retain dirty forms and in-flight operations above row lifetime; evict unmounted clean models. End the registry only when its feature/identity ends.                                                              |
| Feedback           | Preserve shared field error presentation, generated messages, native semantics, accessible descriptions, and focus. API errors use the existing contextual/toast path; Cedar diagnostics stay beside the editor. |
| API constraints    | Keep one root mutation per request and request authoritative updated fields. A GraphQL error or lost response does not prove rollback.                                                                           |
| Availability       | Registered input types and offline mock success never establish an executable production operation.                                                                                                              |

## Requests, cache, authentication, and permissions

| Concern           | Rule to preserve / adapt                                                                                                                                                                                    |
| ----------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Transport         | One shared Houdini client integration retains safe HTTP/GraphQL error handling, one response parse, cancellation, credentials, and request activity tracking.                                               |
| Loading ownership | Query stores describe network state; shared LoadingRegion owns presentation delay/hold. Global overlay is startup-only.                                                                                     |
| Cache identity    | Configure type identity and verified scope explicitly. Embedded payloads/connections must not accidentally become shared entities. Test scoped IDs, aliases, filters, ordering, and projections.            |
| Cache lifetime    | Clear scoped records on signout/user/organization changes and prevent old responses from repopulating the new scope. Avoid persistent storage of authenticated cache data by default.                       |
| Sensitive reads   | Cache hits and partial results never establish fresh identity or permissions. Select appropriate network policies for security-sensitive reads. Cached permission source cannot replace server enforcement. |
| Authentication    | Rust keeps cookie issuance/renewal, session freshness, OTP, revocation, and credential-kind verification. Houdini does not introduce a second authentication server or token store.                         |
| Session flows     | Deduplicate restoration; keep failures recoverable. After confirmed OTP acceptance, retry identity loading rather than confirmation. Preserve existing signout and safe authorization-error distinctions.   |
| Permission worker | Retain one Cedar worker/client lifecycle per applicable scope, deduplicated batches, latest-fact evaluation, cancellation, and denied-by-default unknown/stale facts.                                       |
| Optimism          | Explicitly design any optimistic updates. Do not show successful authentication, permission grants, or destructive/uncertain writes before authoritative confirmation.                                      |
| Updates           | Reconcile changed records and affected lists without reloading directories. Normalized entity updates do not automatically establish list membership, sort position, or pagination correctness.             |
| Reads             | Fetch bounded pages and necessary fields; compose related reads. Cancel obsolete work and retain confirmed content on recoverable failure.                                                                  |
| API shape         | Keep flat roots, resource-first mutation names, shared Payload/Collection wrappers, field authorization, and truthful connection metadata. Verify Houdini pagination against our schema.                    |
| Secrets           | API keys, tokens, OTP, passwords, private headers, and files remain outside URLs and persistent browser storage. Clear/revoke temporary objects at scope teardown.                                          |

## Rendering, design, navigation, and verification

| Area                    | Required practice                                                                                                                                                                                                                 |
| ----------------------- | --------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Initial loading         | Keep shells, headers, sections, columns, padding, and scroll geometry mounted. No empty state before successful data; no form initialization from placeholders.                                                                   |
| Refresh                 | Preserve confirmed content, drafts, focus, editors, and scroll. Empty, zero, and null are valid values, not loading signals.                                                                                                      |
| Skeletons               | Shared inert, accessibility-hidden placeholders; announce once per region. Retain shared 180 ms reveal delay/minimum presentation and reduced-motion behavior without delaying requests.                                          |
| Progress                | Preserve the request-progress visual reference and shared lifecycle, including its 500 ms minimum visible time. Cache-only reads should not pretend to be network activity.                                                       |
| Collections             | Shared data-driven Table/List own virtualization, measurements, spacers, sticky categories, stable keys, two-row overscan, and the existing scroll owner.                                                                         |
| Collection interactions | Retain focused/open-editor rows and drafts. Preserve Tab boundaries, logical row positions, removed-focus behavior, load-more/retry controls, and independent cursors.                                                            |
| Work bounds             | Reuse yieldToBrowser/processInSlices; retain bounded reveal batches and cancellation. Svelte reactivity cannot interrupt an expensive synchronous task.                                                                           |
| Profiling               | Trace equivalent loading/scrolling before and after. Measure tasks, frame gaps, framework rendering, mounted rows, and worker messages. Review targets: no attributable task over 50 ms; incremental render p95 at or below 5 ms. |
| Animation               | Centralize presets; preserve interrupted transitions, immediate inert/dismissal state, focus restoration, and reduced motion.                                                                                                     |
| Accessibility           | Preserve labels, native forms/tables, dialog containment, menu/select keyboard behavior, error descriptions, forced colors, and keyboard-accessible disabled explanations.                                                        |
| Tables                  | Keep responsive columns defined once, gutter sizes, truncation/clamping, and container fit before horizontal scrolling.                                                                                                           |
| URL state               | Retain root navigation ownership, non-secret draft codecs, direct entry/reload, Back/Forward, discard guards, stable draft IDs, and immediate merged query updates. URL state never establishes authority or saved baselines.     |
| Hosting                 | Keep same-origin relative API URLs, Axum development proxy/static production serving, strict Vite port 41868, and current browser routes unless explicitly redesigned. SvelteKit adoption must resolve deployment mode first.     |
| Source/build checks     | Use lint, Svelte/TypeScript checks, UI package build, Storybook build, generation drift checks, and manual browser inspection. Select compatible tooling/scripts during implementation.                                           |
| Test policy             | No Storybook unit/browser/interaction suites or runner. Keep authorized meaningful consumer business-flow tests in application tests directories; follow project limits on new tests.                                             |
| Manual coverage         | Compare themes, responsive sizes, Safari/WebKit and Chromium; inspect keyboard/focus, loading/refresh/error, form retries/reset, stale responses, virtual edits, navigation, and cancellation.                                    |
| Documentation           | Update applicable Wiki architecture, focused pages, and agent conventions when implementation changes behavior. This local file remains planning notes.                                                                           |
| Desktop use             | Never activate applications, browser windows, or tabs automatically, including to recover from tool timeouts.                                                                                                                     |

## Open decisions and next notes

| Decision                           | Evidence needed                                                                                                                    | Current note           |
| ---------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------- | ---------------------- |
| SvelteKit or Svelte SPA router     | Deep links, nested scope lifetime, URL drafts, lazy loading, Axum deployment compatibility                                         | Open                   |
| Houdini version and package layout | Generate a component fragment and route query across packages without cycles; validate aliases in package and Storybook builds     | Open                   |
| Form library                       | Demonstrate existing commit modes, versioned reconciliation, nested groups, virtual draft retention, and generated Zod integration | Open                   |
| Accessible primitives              | Match nested menus, native dialog containers, combobox behavior, focus restoration, and current visual markup                      | Bits UI candidate only |
| Fragment component boundary        | Compare plain data props with runtime fragment references; prove isolated offline Storybook fixtures                               | Open                   |
| Cache rules                        | Audit identities, authority transitions, protected projections, mutation/list updates, and complete versus paginated collections   | Open                   |
| Migration order                    | First prove generation and one shared component/feature; preserve a working app and design reference through later stages          | Proposed               |
| Performance benefit                | Same data, design, environment, and interaction in production builds; include replacement dependency costs                         | Unmeasured             |

## References

| Reference                                                                         | Purpose                                                                           |
| --------------------------------------------------------------------------------- | --------------------------------------------------------------------------------- |
| [Portal architecture](https://github.com/Midwess/compallm/wiki/Portal)            | Hosting, controllers, integration                                                 |
| [Component library](https://github.com/Midwess/compallm/wiki/Portal-components)   | Public UI contracts and interactions                                              |
| [Form ownership](https://github.com/Midwess/compallm/wiki/Portal-form)            | Drafts, commit policy, reconciliation                                             |
| [Shared validation](https://github.com/Midwess/compallm/wiki/Portal-validation)   | Rust contracts and generation                                                     |
| [Session integration](https://github.com/Midwess/compallm/wiki/Portal-session)    | Identity and permission lifecycles                                                |
| [Collection rendering](https://github.com/Midwess/compallm/wiki/Portal-rendering) | Virtualization and performance                                                    |
| [URL state](https://github.com/Midwess/compallm/wiki/Portal-url-state)            | Navigation and draft restoration                                                  |
| [GraphQL guidelines](https://github.com/Midwess/compallm/wiki/GraphQL-guidelines) | Apply current repository flat-root/shared-output rules where Wiki examples differ |
| [Houdini configuration](https://houdinigraphql.com/svelte/v3/core/config/)        | Local schema, document discovery, scalars, generated output                       |
| [Houdini CLI](https://houdinigraphql.com/svelte/v3/core/cli/)                     | Offline generation from local contracts                                           |

Repository `AGENTS.md`, `libs/storybook/AGENTS.md`, and explicit user decisions continue to govern implementation. Candidate
APIs and version-specific Houdini behavior require verification before adoption.

## Parallel scaffold implementation

| Area | Implemented boundary |
| --- | --- |
| Portal | `portal-svelte`: SvelteKit static SPA, blank workspace route, strict port 41869, development API proxy. No production Axum switch. |
| UI and previews | `libs/storybook-svelte`: shared foundations component, six-palette toolbar, Tailwind v4, original fonts, separate library/Storybook builds, strict port 6007. |
| GraphQL | `libs/graphql-svelte`: shared generated stores/types and browser client; a compile-only typename query makes no requests. |
| Generation | Cargo `generate` retains Genql and validation generation, then invokes Houdini. Install the new generator dependencies first. |
| Colocation | Portal and library source documents are scanned together. Vite transforms consume artifacts; explicit generation updates them. |
| Toolchain | Node 24.13+ and pinned Svelte 5 / SvelteKit 2 / Houdini 2 with the Svelte 3 integration. |
| Version adapter | The generator adapts Houdini's compiler API to a repository scan root and shared output. Keep the adapter under review when updating Houdini. |
| Remaining migration | Business components, forms, authentication, cache scope policy, and production hosting integration remain pending. |

## Complete GraphQL contract and operation coverage

| Output | Location / guarantee |
| --- | --- |
| Canonical schema | `libs/graphql-svelte/.houdini/schema.graphql`, copied from Rust export during generation. |
| Inputs and enums | `.houdini/graphql/inputs.ts` and `enums.ts`; all inputs are exported from the package root. IDs stay strings. |
| Output schema | `.houdini/runtime/generated.ts` contains every output type/field and arguments; exported as `SchemaCacheTypes`. Use generated operation results for wire projections. |
| Operations | One authored document per existing Portal selection, colocated by feature under `portal-svelte/src`. No live request integration. |
| Generated stores and results | `.houdini/plugins/houdini-svelte/stores` and `.houdini/artifacts`, exported through the shared package. |
| Conditional fields | Boolean include variables default to false; permission checks remain the caller's responsibility. Partial preference responses select submitted categories only. |
| Coverage | `operation-coverage.json` maps documents to React selections and variable sources. Generation/CI reject missing mappings, invalid documents, or missing schema types/fields. |
| Parallel catalog work | Commit generated outputs against committed Rust/React sources; keep local catalog-refactor variants with that uncommitted work. |
