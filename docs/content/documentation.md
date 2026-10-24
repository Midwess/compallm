---
title: Documentation guide
---

# Documentation guide

The documentation website is built with [Quartz](https://quartz.jzhao.xyz/). Its source is ordinary Markdown in `docs/content/`, readable directly on GitHub and in Obsidian.

## Run locally

From the repository root:

```bash
cd docs/web
npm ci
npm run dev
```

Open http://localhost:8080. Markdown edits trigger a rebuild and browser refresh.

## Feature availability

This site documents the current web application, with no release-version picker. Features default to **Under development**. Only the exact frontmatter value below makes a feature available:

```yaml
---
title: LLM Router
status: available
---
```

Missing status, `status: under-development`, or any other value means **Under development**. Use that status for all work in progress, and explicitly change it to `available` only once the feature is usable. Update its page to describe the shipped behavior when making that change.

The **Feature filter** beside the Compallm title defaults to **Available**. **All** exposes all feature plans, with a status badge on each page and feature entry. The filter applies to the feature list, sidebar, search, graph, and backlinks. Direct links to development content intentionally open that inclusive view.

Both views use the same Markdown sources: the public root contains available features and general documentation; `/planning/` contains the inclusive view. This is a static-site filter, not an access-control boundary. Development plans remain publicly accessible. Changing the filter reloads the view to keep its navigation and search data consistent. New visits to the root always default to available features.

Edit a page under `docs/content/features/` and rebuild to update its status. The development server rebuilds both views when content changes. There are currently no features explicitly marked available, so the default feature list shows an empty state.

## Build the website

From `docs/web/`:

```bash
npm run build
```

Static output is in `docs/web/public/`. The application and documentation site are separate builds.

## GitHub Pages deployment

The `Deploy documentation` workflow in `.github/workflows/docs.yml` runs on every push to `main` and can be run manually. It installs only the docs dependencies, builds both availability views, and deploys `docs/web/public/` to https://midwess.github.io/compallm/.

One-time repository setup: open **Settings → Pages**, then choose **GitHub Actions** as the build and deployment source. The workflow uses GitHub's built-in token and does not need a separate deployment secret. Other branches are not deployed.

Quartz defaults to the repository Pages address. The workflow supplies `DOCS_BASE_URL` from GitHub Pages configuration, so canonical URLs and sitemaps follow the configured site address. The All view uses its `/planning/` subpath. Internal links, assets, and availability filters stay relative to the site, including under `/compallm/`.

## Editing pages

Use standard headings, lists, tables, and relative Markdown links such as `[LLM Router](features/llm-router.md)`. Optional YAML frontmatter supplies titles and tags. Backlinks and the graph are derived from the links automatically; wiki links are not required.

Keep each feature on its own page. Record unresolved questions in [business decisions](decisions.md) and delivery progress in [roadmap](roadmap.md). Plans and behavior examples should remain clearly distinguished from implemented features.

Quartz source and MIT license are preserved in `docs/web/`. The site uses Quartz v4.5.2 at commit `4923affa7722dfc751f1074348e6dad214fe0c08`. Review upstream changes deliberately when upgrading.
