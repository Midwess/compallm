# Axum + Vite server functions

A root Vite/React application with an Axum server in the `server` Cargo workspace member. TypeScript API functions are generated from the handlers' OpenAPI metadata.

```text
.
|-- package.json
|-- vite.config.ts
|-- Cargo.toml
|-- src/                  # React frontend
|-- public/
|-- dist/                 # Vite production build
`-- server/
    |-- Cargo.toml
    `-- src/              # Axum server
```

## Commands

Install JavaScript dependencies once:

```bash
pnpm install
```

Start Axum and the Vite development server:

```bash
pnpm dev
```

Open `http://127.0.0.1:3000`. Axum owns that port and proxies non-API traffic, including Vite's HMR WebSocket, to Vite on port 5173.

Build both the frontend and release server:

```bash
pnpm build
```

Start the production server, which serves `dist/` and the Rust API:

```bash
pnpm start
```

## Generated API client

Rust DTOs derive `utoipa::ToSchema` in `server/src/api_types.rs`. Handlers in `server/src/api.rs` use `#[utoipa::path]`, and `utoipa-axum` uses those declarations for both Axum routing and the OpenAPI document.

Regenerate the OpenAPI document and Orval client manually with:

```bash
pnpm generate:api
```

This writes the temporary `openapi.json` document and the tracked `src/contract/api.ts` client. Commit changes to `src/contract/` whenever the Rust API contract changes.

The React application imports normal generated functions:

```ts
const greeting = await createGreeting({ name: 'Ada' })
```

The call is a typed HTTP request to the corresponding Axum handler. It resembles a server function at the call site, but it is not React's RSC Server Function protocol.

For automatic Rust restarts while developing server code, install `cargo-watch` and use:

```bash
cargo watch -w server/src -w server/Cargo.toml -x 'run -p cosin-server --features frontend-dev'
```
