# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Product plan

See the [product overview](docs/content/overview.md) for feature plans. Each feature's `adr.md` records its decisions and open questions.

## Development

Read the [development decisions](docs/content/development/adr.md) before changing the code.

The frontend uses React with Vite, and the Rust server uses Loco and Axum. PostgreSQL is accessed through SeaORM; Utoipa and Orval generate the TypeScript API client.

Axum serves the frontend in both local development and production. In development, it proxies the Vite dev server, including hot reload. In production, it directly serves Vite's built static files. The frontend and API share the same public origin.

Requires Rust, Node.js 22 or newer, and Docker Compose. From the repository root:

```bash
npm ci
docker compose up -d
cargo run -p compallm-server --features frontend-dev -- start
```

Compose starts the backing services, including PostgreSQL on port `7887`. Cargo starts the backend and Vite with frontend hot reload. Open the URL logged by the API server: it prefers port `3000` and chooses the next available port when occupied. `PORT` or the server CLI port option sets the starting point, within `1–50100`. The API listens on all IPv4/IPv6 interfaces in development and production; deployment port mappings must match the selected port.

## License

Compallm is source available under the [Elastic License 2.0](LICENSE); see [NOTICE](NOTICE) for attribution and third-party licensing information. The license permits use, modification, and redistribution but prohibits providing Compallm to third parties as a hosted or managed service that exposes a substantial set of its features. The vendored Quartz source retains its [MIT license](docs/web/LICENSE.txt).
