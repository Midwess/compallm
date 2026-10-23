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

## Build the website

From `docs/web/`:

```bash
npm run build
```

Static output is in `docs/web/public/`. Hosting is not configured yet. The application and documentation site are separate builds.

## Editing pages

Use standard headings, lists, tables, and relative Markdown links such as `[LLM Router](features/llm-router.md)`. Optional YAML frontmatter supplies titles and tags. Backlinks and the graph are derived from the links automatically; wiki links are not required.

Keep each feature on its own page. Record unresolved questions in [business decisions](decisions.md) and delivery progress in [roadmap](roadmap.md). Plans and behavior examples should remain clearly distinguished from implemented features.

Quartz source and MIT license are preserved in `docs/web/`. The site uses Quartz v4.5.2 at commit `4923affa7722dfc751f1074348e6dad214fe0c08`. Review upstream changes deliberately when upgrading.
