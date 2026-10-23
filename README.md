# Compallm

Help individuals and teams work, learn, and build with LLMs through an organization workspace.

Compallm combines access to multiple providers, governed API usage, chat, interactive responses, plugins, and knowledge notebooks.

## Read the product plan

[Browse the Markdown documentation](docs/content/index.md), starting with the [product overview](docs/content/product.md), [roadmap](docs/content/roadmap.md), and [business decisions](docs/content/decisions.md).

All content remains standard Markdown in `docs/content/`. Feature pages describe planned scope rather than completed implementation.

## Run the documentation website

Requires Node.js 22 or newer. From the repository root:

```bash
cd docs/web
npm ci
npm run dev
```

Open http://localhost:8080. Edit Markdown in `docs/content/`; changes reload automatically. To build static pages, run `npm run build` inside `docs/web/`.

## Development

See the [development guide](docs/content/development.md) for application setup, database configuration, build commands, and API generation.

## License

Compallm is licensed under Apache 2.0; see [LICENSE](LICENSE) and [NOTICE](NOTICE). The vendored Quartz source retains its [MIT license](docs/web/LICENSE.txt).
