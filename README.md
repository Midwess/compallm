# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Product plan

See the [Compallm Wiki](https://github.com/Midwess/compallm/wiki) and [product overview](https://github.com/Midwess/compallm/wiki/Product-overview) for feature plans. Each feature's architecture page describes its solution and unresolved requirements. Documentation is maintained in the separate Wiki Git repository; this application repository contains no documentation site.

## Development

The frontend uses React with Vite, and the Rust server uses Loco and Axum. PostgreSQL is accessed through SeaORM. GraphQL is served at `POST /api/graphql` through Juniper and its Axum adapter, with Rust definitions under `api/src/graphql`. The authenticated session query and email-login, explicit refresh and signout mutations preserve credential policies, access freshness and cookie renewal. Portal uses the generated Genql client for authentication and role/group summaries. Protected role/group metadata reads use flat role/roles and group/groups fields, with roleCreate and groupCreate mutations; editing, member assignments, and provider runtime operations remain pending. Core services and offline component previews remain available.

Axum serves the frontend in both local development and production. In development, it proxies the Vite dev server, including hot reload. In production, it directly serves Vite's built static files. The frontend and API share the same public origin.

Requires Rust, Node.js 22.18 or newer, and Docker Compose. From the repository root:

```bash
npm --prefix portal ci
docker compose up -d
cargo run -p compallm-api -- start
```

Compose starts the backing services, including PostgreSQL on port `7887`. Debug builds start the backend and Vite with frontend hot reload. Use `cargo run -r -p compallm-api -- start` to serve prebuilt `portal/dist` assets without Vite; build the Portal assets first. Vite uses internal port `41868` with strict binding; browse through the API origin. Open the URL logged by the API server: it prefers port `3000` and chooses the next available port when occupied. `PORT` or the server CLI port option sets the starting point, within `1–50100`. The API listens on all IPv4/IPv6 interfaces in development and production; deployment port mappings must match the selected port.

Regenerate the Rust-owned GraphQL schema, Genql TypeScript query builder, and Rust-owned validation rules/constraints without starting the application or accessing the database:

```bash
npm --prefix libs/contract ci
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

## License

Compallm is source available under the [Elastic License 2.0](LICENSE); see [NOTICE](NOTICE) for attribution and third-party licensing information. The license permits use, modification, and redistribution but prohibits providing Compallm to third parties as a hosted or managed service that exposes a substantial set of its features.
