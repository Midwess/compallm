# Svelte migration notes

Portal and Storybook are the official Svelte projects. The React projects and Genql pipeline have been removed.
Rust remains the source of API contracts and business rules. Product documentation lives in the external Compallm Wiki;
this file is the requested local architecture record.

## Architecture and retained behavior

| Concern            | Current pattern                                                                               | Required boundary                                                                                                                                    |
| ------------------ | --------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------------------------------------------------------- |
| Framework          | Svelte 5.59.1, SvelteKit 3.4.0, adapter-node 6.0.5, Vite                                      | Native runes, routing, SSR, async compilation, experimental forkPreloads.                                                                            |
| Backend            | Rust/Loco/Axum                                                                                | Own authentication, authorization, validation, database access, mutations, and business invariants.                                                  |
| Public requests    | Axum owns `/api` and `/api/*`; other paths stream to Svelte                                   | Preserve request/response streaming, backpressure, cancellation, status, cookies, and same-origin browser access.                                    |
| Frontend process   | API configuration `settings.frontend.port`, default 4868                                      | Strict loopback binding. Managed startup uses built Node output; external supervision is configurable.                                               |
| Port conflict      | Compose currently publishes PostgreSQL on 7887                                                | Svelte defaults to 4868. Never silently scan the frontend port.                                                                                      |
| SSR data           | Request-local executor uses generated Houdini artifacts/types                                 | Verify identity per request; forward public Host/Origin and cookies; deliver renewal before headers. No shared user state.                           |
| Initial rendering  | Server data seeds component-owned controllers before rendering                                | No mount gate and no duplicate initial request. Keep form baselines and confirmed cursors.                                                           |
| Page coverage      | Authentication, invitation, workspace/sidebar, provider/model editors, settings and directory | Existing unavailable features remain unavailable. Credential ceremonies and onboarding presentation stay browser commands.                           |
| Permissions        | Rust supplies organization decisions and role source capability                               | Node never evaluates business authorization. Browser Cedar worker continues contextual checks; unknown/stale facts deny.                             |
| Controllers        | Stable typed Svelte contexts and explicit constructor injection                               | Nested controllers retain their lifetimes; no callback drilling, service locator, or global SSR instances.                                           |
| Reactive state     | One deep native `$state` object; property reads and `$derived`                                | Imperative lifecycle subscriptions remain. Class instances are not automatically proxied.                                                            |
| Forms              | Native form state plus the existing commit coordinator                                        | Preserve blur/change/button timing, partial commits, queues, edit versions, baselines, retry, cancellation, File/Blob identity, and detached drafts. |
| Navigation         | Real links, SvelteKit routing, shared Preload, forkPreloads                                   | Hover stays silent. Accepted unfinished navigation and explicit APIs show progress. Retain discard guards, history, drafts, and sidebar state.       |
| Collections        | Normal keyed Table/List rows and bounded load more                                            | Preserve page scroll owner, stable keys, focused editors, keyboard actions, cursors, and retries. No virtualization or TanStack.                     |
| Browser networking | TypeScript service worker and shared network service                                          | Parse once, bounded gzip, cancellation, safe errors. Group identical eligible in-flight reads and reuse successes for 100 ms.                        |
| Freshness          | Authority epochs and read versions                                                            | Writes and identity changes invalidate reuse. Reject stale responses; never replay authentication or uncertain writes.                               |
| Storage            | No persistent response cache                                                                  | Worker reuse remains transient and client scoped.                                                                                                    |
| GraphQL            | Houdini is the sole API generator                                                             | Rust exports SDL; authored documents declare selections. Artifacts/types/stores live in libs/contract/houdini/generated.                             |
| Validation         | Rust Validify and validation export                                                           | Shared Zod rules, constraints, defaults, normalization, messages, and inferred form aliases. No parallel UI rules/API interfaces.                    |
| Design             | Shared Svelte UI with copied Tailwind foundations, six palettes, fonts, artwork               | Preserve original typography, dimensions, behavior, accessibility, focus, and reduced motion.                                                        |
| SEO                | SSR public metadata; protected pages noindex                                                  | Private HTML and route data are no-store. SSR alone does not establish a measured performance gain.                                                  |
| Production         | Build portal/build and run Node behind Axum                                                   | Deploy complete adapter output and runtime dependencies. Configure public HTTPS origin and secure cookies.                                           |
| Verification       | Lint/format, Svelte/TS checks, builds, consumer flows, headless manual inspection             | Storybook remains offline with no automated browser/interaction suite.                                                                               |

## Generation

`cargo run -p compallm-api -- generate` exports Rust SDL and validation metadata, checks compatibility, emits validation,
generates Houdini, validates every authored document and full schema coverage, then builds the shared contract.
It starts no app, database connection, or migration. Genql output, dependencies, and legacy selection coverage are removed.
Documents may be colocated with consumers; generated code always stays in contract. Install contract dependencies first.

## Deployment and streaming

The API starts portal/build/index.js by default and passes the actual bound API origin. FRONTEND_MANAGED=false lets an
external supervisor own the Node process. AUTH_PUBLIC_ORIGIN defines the trusted public HTTPS origin in production.
Vite remains optional development tooling; API-managed startup uses the built Node server in both environments.
Rebuild/restart after frontend edits. Standalone Vite uses the configured frontend port and proxies API requests to Rust.

Axum pipes bodies without collecting them. This supports SvelteKit response/data streaming. Arbitrary asynchronous
component HTML streaming depends on framework support; it is not implied by enabling SSR. Initial identity, redirects,
cookies, and essential page/sidebar data settle before response headers. The browser worker does not run server reads.

## Remaining limits

Physical-device passkeys, exhaustive visual parity, and comparative production performance need separate measurement.
Native rune reactivity does not interrupt expensive synchronous work. Broader normalized caching remains undecided.
Database context/connection lifecycle and RLS infrastructure remain restricted and unchanged by this migration.
