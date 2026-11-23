# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Product plan

See the [Compallm Wiki](https://github.com/Midwess/compallm/wiki) and [product overview](https://github.com/Midwess/compallm/wiki/Product-overview) for feature plans. Each feature's architecture page describes its solution and unresolved requirements. Documentation is maintained in the separate Wiki Git repository; this application repository contains no documentation site.

## Development

The frontend uses React with Vite, and the Rust server uses Loco and Axum. PostgreSQL is accessed through SeaORM. GraphQL is served at `POST /api/graphql` through Juniper and its Axum adapter, with Rust definitions under `api/src/graphql`. The authenticated session query and email-login, explicit refresh and signout mutations preserve credential policies, access freshness and cookie renewal. Portal uses the generated Genql client for authentication and role/group summaries. Protected role/group metadata reads use flat role/roles and group/groups fields, with roleCreate and groupCreate mutations, member role replacement, group membership changes, and invitations. Each member has one required direct role and can join multiple groups; groups cannot grant Owner. Provider creation and saved-provider reads are integrated through GraphQL, with private user/organization model defaults. Existing-provider updates and inference execution remain pending. Core services and offline component previews remain available.

Axum serves the frontend in both local development and production. In development, it proxies the Vite dev server, including hot reload. In production, it directly serves Vite's built static files. The frontend and API share the same public origin.

Requires Rust, Node.js 22.18 or newer, and Docker Compose. From the repository root:

```bash
npm --prefix portal ci
docker compose up -d
cargo run -p compallm-api -- start
```

Provider creation requires `PROVIDER_CREDENTIAL_SECRET`, a server-only secret of at least 32 bytes. It encrypts stored provider API keys; keep it stable across restarts. Missing or invalid configuration makes creation unavailable.

Access and Security settings request approximate city labels directly from the public location service. Lookup failures leave location unknown and never affect authentication. Portal uses the Railway endpoint directly. To run the DB-IP service locally:

```bash
./geolocation/download-dbip.sh
cargo run -p compallm-geolocation
```

See [browser sessions](https://github.com/Midwess/compallm/wiki/Login-session) for deployment and data updates.

Compose starts the backing services, including PostgreSQL on port `7887`. Debug builds start the backend and Vite with frontend hot reload. Use `cargo run -r -p compallm-api -- start` to serve prebuilt `portal/dist` assets without Vite; build the Portal assets first. Vite uses internal port `41868` with strict binding; browse through the API origin. Open the URL logged by the API server: it prefers port `3000`, reuses recently closed addresses on restart, and chooses the next available port when another listener occupies it. `PORT` or the server CLI port option sets the starting point, within `1–50100`. The API listens on all IPv4/IPv6 interfaces in development and production; deployment port mappings must match the selected port.

In the development environment, open `/api/graphiql` on the API server to browse the schema and run GraphQL requests. It uses `/api/graphql`; protected operations require a valid login cookie or an Authorization header.

Regenerate the Rust-owned GraphQL schema, Genql and Houdini clients, and Rust-owned validation rules/constraints without starting the application or accessing the database:

```bash
npm --prefix libs/contract ci
npm --prefix libs/graphql-svelte ci
cargo run -p compallm-api -- generate
```

Development email uses MailDev: SMTP on `1025`, inbox at http://127.0.0.1:1080. See [mailing](https://github.com/Midwess/compallm/wiki/Mailing) for queue limits and [email login](https://github.com/Midwess/compallm/wiki/Login-email) for SMTP environment variables.

To run the component library previews independently of the backend:

```bash
cd libs/storybook
npm ci
npm start
```

Open `http://localhost:6006` and use Codex CLI from the repository root to iterate.

## Parallel Svelte workspace

The React applications remain active. `portal-svelte` is an empty SvelteKit static SPA;
`libs/storybook-svelte` contains an offline foundations preview with the existing six Tailwind palettes and fonts.
Use Node.js 24.13 or newer for these projects. From the repository root:

```bash
npm --prefix libs/contract ci
npm --prefix libs/graphql-svelte ci
npm --prefix libs/storybook-svelte ci
npm --prefix portal-svelte ci
cargo run -p compallm-api -- generate
npm --prefix libs/storybook-svelte start
npm --prefix portal-svelte run dev
```

Storybook uses `http://127.0.0.1:6007`; Portal uses `http://127.0.0.1:41869`.
Neither opens a browser automatically. Portal proxies `/api` to `http://127.0.0.1:3000` during development;
set `RUST_API_ORIGIN` when Rust uses another origin. The scaffold makes no requests automatically.
Existing Axum startup and React serving remain unchanged. Deploying the Svelte output through Axum is a later step.

Queries and fragments belong beside their consumers in the new Portal and UI library. The shared
`libs/graphql-svelte` package generates their Houdini stores/types from Rust's exported schema.
After changing documents, rerun its `generate` script; after changing Rust contracts, run the Cargo command above.
The Vite adapter transforms documents but leaves generation under the explicit command's control.
Schema pulling and polling are disabled. Shared Zod validators remain in `@compallm/contract`.

Houdini outputs live in `libs/graphql-svelte/.houdini`: `schema.graphql` is the complete Rust-exported schema,
`graphql/inputs.ts` and `graphql/enums.ts` contain schema-wide inputs/enums, `runtime/generated.ts` contains all output
cache types/fields, and `artifacts/` plus `plugins/houdini-svelte/stores/` contain operation result/variable types and stores.
The package root exports operation stores/types, input types, enums, and `SchemaCacheTypes`; use operation result types
for selected response data. These follow Houdini's model rather than recreating Genql's query-builder interfaces.

Every current React Portal selection has an authored `.graphql` equivalent under the corresponding `portal-svelte/src`
feature folder. `portal-svelte/operation-coverage.json` maps each document to its source selection and variable expressions.
Permission conditions become Boolean variables defaulting to false; preference-update category flags preserve partial
result selections. No document runs automatically. The `check:coverage` script verifies each source selection is covered,
each document validates, and all Rust input objects, enums, output objects, and fields occur in Houdini's generated types.
Generation and CI run these checks. Retire the React-source coverage check when removing the old Portal.

Both new projects provide `check`, `lint:check`, and `build` scripts. The library also provides `build:storybook`.
The UI library owns the new theme source and exports `theme.css` and compiled `styles.css`;
Portal compiles its own Tailwind utilities from those foundations. Stories remain outside published production CSS scanning.

## License

Compallm is source available under the [Elastic License 2.0](LICENSE); see [NOTICE](NOTICE) for attribution and third-party licensing information. The license permits use, modification, and redistribution but prohibits providing Compallm to third parties as a hosted or managed service that exposes a substantial set of its features.
