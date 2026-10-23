# Quartz site tooling

The editable documentation is in [`../content/`](../content/index.md). This directory contains the official Quartz v4.5.2 source, pinned to commit `4923affa7722dfc751f1074348e6dad214fe0c08`, with Compallm configuration and layout. Its upstream code remains MIT-licensed; see [LICENSE.txt](LICENSE.txt). Build dependencies have security updates, including esbuild 0.28, sharp 0.35, and toml 5. Four low-severity audit findings remain in the optional KaTeX dependency chain; the site does not enable the LaTeX transformer.

From the repository root:

```bash
cd docs/web
npm ci
npm run dev
```

Run `npm run build` in this directory to build static pages.

The preview runs at http://localhost:8080 and rebuilds when Markdown changes. Static output is written to `docs/web/public/`. Search, graph navigation, backlinks, and popovers are enabled. The site uses dark mode only and standard system fonts, with no theme toggle or external font downloads; analytics are disabled. No hosting is configured automatically.

Colors are configured in `quartz.config.ts`, layout in `quartz.layout.ts`, and typography/spacing in `quartz/styles/custom.scss`. Both palette slots intentionally contain the same dark colors. Two small upstream component changes enforce dark HTML on first render: `renderPage.tsx` sets `saved-theme="dark"` and `Head.tsx` declares the dark color scheme. Preserve these when upgrading Quartz.

Use ordinary relative Markdown links. The documents remain readable on GitHub or in an editor, and can be opened as an Obsidian vault without depending on wiki-link syntax.

Quartz is kept separate because it includes its own Preact build tooling. Upgrade it deliberately from https://github.com/jackyzha0/quartz and review the config/layout changes; do not use Quartz's Git sync commands to manage the application repository.
