# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Database schema viewer

Open [db_scheme/dbml.html](db_scheme/dbml.html) directly in a browser. The file includes the schema, hover explanations, an editor, and HTML/DBML downloads; it needs no local server. The diagram uses dbdiagram and requires internet access. It describes the target provider/model catalog design and does not inspect or modify the running database.

## Product plan

See the [Compallm Wiki](https://github.com/Midwess/compallm/wiki) and [product overview](https://github.com/Midwess/compallm/wiki/Product-overview) for feature plans. Each feature's architecture page describes its solution and unresolved requirements. Documentation is maintained in the separate Wiki Git repository; this application repository contains no documentation site.

## Development

Portal uses Svelte 5 and SvelteKit SSR with adapter-node. Rust/Loco/Axum owns `/api` and `/api/*` and streams every other request to the private Svelte server. Rust retains authentication, authorization, validation, database access, and business operations. Svelte renders pages and calls Rust for initial data; browser controllers handle interactive commands.

Requires Rust, Node.js 24.13 or newer, and Docker Compose. From the repository root:

```bash
npm --prefix libs/contract ci
npm --prefix libs/contract run build
npm --prefix libs/storybook ci
npm --prefix libs/storybook run build
npm --prefix portal ci
npm --prefix portal run build
docker compose up -d
cargo run -p compallm-api -- start
```

For pnpm, use the root workspace and Node.js 24.13+:

```bash
pnpm install
pnpm --dir libs/contract build
pnpm --dir libs/storybook build
pnpm --dir portal dev
```

When running the frontend separately, start Rust in another terminal with
`FRONTEND_MANAGED=false cargo run -p compallm-api -- start`. Axum forwards to the existing dev server on port 4868.

Running `pnpm install` from Portal also installs the workspace. Its local contract and Storybook packages are linked to
source directories. Use the root pnpm lockfile; keep package-local npm lockfiles for the npm CI workflow.

The API configuration owns `settings.frontend.port`, default `4868`, overridden with `FRONTEND_PORT`.
PostgreSQL remains on `7887`; the frontend uses `4868`.
Axum binds its public port first, passes the actual API origin to Node, starts `portal/build/index.js`, and checks readiness.
Frontend binding is strict. `FRONTEND_MANAGED=false` connects to an independently supervised loopback Node server on the configured port.
Set its `RUST_API_ORIGIN` to the internal API address, `HOST=127.0.0.1`, `PORT` to the frontend port,
`HOST_HEADER=x-forwarded-host`, and `PROTOCOL_HEADER=x-forwarded-proto`.

Browse the URL logged by Axum, normally `http://localhost:3000`. Restarting reuses recently closed addresses;
only an occupied API port causes an upward scan. `PORT` selects the starting API port. The frontend port never scans.
Use localhost for development passkeys; 127.0.0.1 is a different WebAuthn domain.
For production set `AUTH_PUBLIC_ORIGIN` to the public HTTPS origin, retain secure cookies, configure the signing and provider secrets,
and supervise both processes. Deploy the complete `portal/build` output beside the API binary's Portal directory, with its Node runtime dependencies.
Node is required in production; Vite is build/development tooling. API-managed startup uses the built Node server in both environments.
Rebuild and restart it after frontend edits. Standalone `npm --prefix portal run dev` offers Vite development on port 4868;
its API target defaults to port 3000 and can be overridden with `RUST_API_ORIGIN`.

Provider creation requires a stable server-only `PROVIDER_CREDENTIAL_SECRET` of at least 32 bytes.
MailDev exposes the development inbox at http://localhost:1080 and SMTP on 1025.
GraphiQL is available only in Development at `/api/graphiql`, targeting `/api/graphql`.
Approximate session location lookup failures leave location unknown and do not affect authentication.

## Portal and Storybook

`portal` is the official SvelteKit app. `libs/storybook` exports `@compallm/storybook` and provides offline previews on port 6007.
React reference projects and Genql have been removed. The UI library owns Tailwind foundations, six palettes, fonts, and artwork;
Portal compiles its own utilities. Public provider assets and attribution live in `portal/static`.
Storybook uses those assets offline and never invokes business APIs or device credentials.

```bash
npm --prefix libs/storybook start
```

The library includes 220 page scenarios across authentication, invitations, passkeys, workspace, providers/models, onboarding,
settings, and form coordination. Stable typed contexts supply consumer-owned controllers. Nested controllers use constructor dependencies.
Native deep rune state provides property reactivity. Forms retain Rust-generated validation, partial commits, queues, response reconciliation,
retry, cancellation, and drafts. Tables/lists render loaded rows normally with bounded continuation and accessible Load more/Retry.

Initial server loads verify identity, renew Rust-owned cookies, read essential sidebar/page data, and return serializable generated results.
Component-local controllers adopt those results before rendering and continue after hydration without repeating initial reads.
No request-specific authentication, controller, or transport state is shared between SSR requests.
Protected loaders verify authority independently; a route name cannot select organization authority.
Private HTML and route data are no-store and noindex. Public authentication pages include metadata.

Native SvelteKit routing and experimental fork preloads retain workspace shells, sidebar disclosure/scroll, and drafts.
Shared Preload links opt into hover data preparation. Hover stays silent; accepted navigation shows progress while unfinished.
Explicit API calls retain ordinary progress. Browser reads use the service worker, cancellation, and authority/read versions.
Authentication and uncertain writes are never replayed. Onboarding presentation and credential ceremonies remain browser commands.
Inference, saved-provider updates, and other unavailable backend features remain pending.

## Shared contracts

Rust definitions under `api/src/graphql` export the SDL and validation contracts into `libs/contract`.
Houdini is the sole API type/client generator. Shared Zod validation, normalization, defaults, constraints, and messages remain Rust-owned.
The offline generation command starts no app, database connection, or migration:

```bash
npm --prefix libs/contract ci
cargo run -p compallm-api -- generate
```

Generation exports SDL/validation metadata, checks compatibility, emits validators, generates Houdini, and builds the contract package.
Author operations/fragments beside consumers or under `libs/contract/houdini/operations`; generated outputs always stay in
`libs/contract/houdini/generated`. The canonical SDL is `libs/contract/schema.graphql`.
After document changes run `npm --prefix libs/contract run generate:houdini`.
`check:houdini` validates every document and schema-wide inputs, enums, output objects, and fields. Schema pulling/polling is disabled.
Server reads use those same generated artifacts/types through a request-local executor; browser commands use generated stores.

## Checks and network worker

Both Portal and Storybook provide `lint:check`, `lint:fix`, `format`, `format:check`, `check`, and `build`.
Storybook also provides `build:storybook`; Portal provides existing consumer flow checks with `npm --prefix portal test`.
ESLint applies official Svelte recommended rules; Prettier owns formatting. Generated builds/dependencies are excluded.
Storybook is inspected manually and has no automated interaction/browser suite.

SvelteKit bundles `portal/src/service-worker/index.ts` to `/service-worker.js`. `src/network/service.ts` owns execution,
response validation, independent waiter cancellation, and grouping eligible identical in-flight queries.
Successful eligible results remain reusable for 100 ms after completion. There is no persistent response cache.
Matching includes document, canonical variables, browser client, authority epoch, and read/invalidation version.
Authentication, credentials, permission source, and mutations bypass reuse. Writes and authority changes invalidate entries.
The bridge falls back locally only before dispatch; a posted uncertain write is never resent.
Compression uses one shared bounded gzip implementation. Worker types are checked separately by the normal check command.

Architecture and operational details live in the [Compallm Wiki](https://github.com/Midwess/compallm/wiki/Portal).

## License

Compallm is source available under the [Elastic License 2.0](LICENSE); see [NOTICE](NOTICE) for attribution and third-party licensing information. The license permits use, modification, and redistribution but prohibits providing Compallm to third parties as a hosted or managed service that exposes a substantial set of its features.
