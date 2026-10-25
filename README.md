# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Product plan

See the [product overview](docs/content/product.md), [roadmap](docs/content/roadmap.md), and [business decisions](docs/content/decisions.md).

## Development

Read the [architecture guidelines](docs/content/architecture.md) before changing the code.

The frontend uses React with Vite, and the Rust server uses Loco and Axum. PostgreSQL is accessed through SeaORM; Utoipa and Orval generate the TypeScript API client.

Axum serves the frontend in both local development and production. In development, it proxies the Vite dev server, including hot reload. In production, it directly serves Vite's built static files. The frontend and API share the same public origin.

Requires Rust, Node.js 22 or newer, and Mocker. From the repository root:

```bash
npm ci
mocker pull docker.io/library/postgres:18-alpine
mocker compose up -d
cargo run -p compallm-server --features frontend-dev -- start
```

Compose starts the backing services, including PostgreSQL on port `7887`. Open http://127.0.0.1:3000; Cargo starts the backend and Vite with frontend hot reload.

Mocker 0.9.7 can show no output while Compose pulls a missing image, making the first startup appear stuck. The explicit pull above separates that download from startup; it is cached on subsequent runs. Use `mocker compose ps` to check service status and `mocker compose logs --tail 50` to inspect startup logs.

## License

Compallm is licensed under Apache 2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE). The vendored Quartz source retains its [MIT license](docs/web/LICENSE.txt).
