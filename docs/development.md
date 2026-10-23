# Development

A root React/Vite application backed by a Loco server. Loco owns application boot, configuration, health checks, middleware and `AppContext`; documented API endpoints remain ordinary Axum handlers registered through `utoipa-axum`. Orval generates the committed TypeScript client from that OpenAPI contract.

```text
.
|-- package.json
|-- vite.config.ts
|-- orval.config.ts
|-- Cargo.toml
|-- compose.yaml            # local PostgreSQL on port 7887
|-- config/                  # Loco environment configuration
|-- src/                     # React frontend
|   `-- contract/api.ts      # committed Orval client
|-- public/
|-- dist/                    # Vite production build
`-- server/
    |-- Cargo.toml
    |-- postgres/            # connection, schema and migrations
    |-- src/main.rs          # only server entrypoint
    |-- src/app.rs           # Loco Hooks implementation
    |-- src/controllers/     # Axum + utoipa controllers
    |-- src/dtos/
    |-- src/tasks/
    `-- src/workers/
```

## Commands

Install dependencies:

```bash
npm install
```

npm is the documented baseline, but the scripts do not invoke a package
manager internally. Developers may run the same scripts with Yarn, pnpm, Bun,
or another compatible Node package manager.

Start the local PostgreSQL database:

```bash
docker compose up -d postgres
```

Development and tests read `DATABASE_URL`, defaulting to
`postgres://root:shh@127.0.0.1:7887/compallm` to match `compose.yaml`.
Production requires `DATABASE_URL` to be set explicitly.

The `compallm-postgres` crate owns the database boundary. It exports the SeaORM
schema modules, Loco migrator and a cloneable connection handle backed by the
single pool that Loco creates. Controllers can extract the handle with Axum's
`State<compallm_postgres::Connection>`; it does not create a second pool.

Database IDs come from `midwess-hero`'s scoped Snowflake generator. Loco
initializes the single-instance generator during boot, so no etcd coordinator
or worker registration is required.

Loco's runtime commands (`db migrate`, `db status`, and related commands) use
the relocated migrator normally. Loco 1.2's scaffold generator still assumes a
`server/migration` directory, so new migrations should be added under
`server/postgres/src/migration/` and registered in its `Migrator` manually.

Start Loco and Vite with a single public origin at `http://127.0.0.1:3000`:

```bash
npm run dev
```

Loco starts Vite internally and reverse-proxies the UI and HMR WebSocket. Vite also proxies `/api` to Loco, so opening its internal `5173` address during debugging still reaches the Rust API.

Build the generated contract, frontend and release server:

```bash
npm run build
```

Start with Loco's production configuration and serve `dist/` from the Rust process:

```bash
npm run start
```

Useful Loco commands:

```bash
cargo loco db status
cargo loco db migrate
npm run server:routes
npm run server:doctor
cargo loco middleware --config
```

`server:routes` lists Loco-managed routes such as health checks. The documented API router is mounted through `Hooks::before_routes`; inspect its complete contract with `npm run generate` and `src/contract/api.ts`.

## API contract

DTOs derive `utoipa::ToSchema` under `server/src/dtos/`. Controllers use `#[utoipa::path]` and are registered by `utoipa-axum` in `server/src/controllers/`.

Regenerate the OpenAPI document and TypeScript client with:

```bash
npm run generate
```

The command exports the Utoipa contract to the ignored intermediate
`openapi.json`, then runs Orval to regenerate `src/contract/api.ts`. Commit the
generated TypeScript file whenever the Rust API contract changes.

The pipeline lives in Rust and can also be invoked directly:

```bash
cargo run -p compallm-server -- generate
```

Loco scaffolding remains available when a component follows `generate`, for
example `cargo loco generate model user`.

The generated functions use relative `/api/...` URLs, so the browser automatically calls the same host and port that served the page.

## Architecture boundary

```text
Loco Hooks
|-- builds AppContext and middleware
|-- mounts utoipa-axum API router before Loco routes
|-- exposes /_ping, /_health and /_readiness
|-- proxies Vite in development
`-- serves dist/ in production
```

The Utoipa router is mounted through `Hooks::before_routes`, so it receives Loco's configured middleware. Loco's own `Routes` builder remains available for non-OpenAPI controllers and generated Loco resources.

## License

Licensed under the Apache License, Version 2.0. See [LICENSE](../LICENSE) for the
full text and [NOTICE](../NOTICE) for attribution details.
