---
title: "Component development workspace"
---

# Component development workspace

The component library implementation is maintained in the separate UI change; this page describes its target architecture and integration contract.

`libs/storybook/` is an independent npm library and preview workspace. Node.js 22.12 or newer and npm are required.

## Run locally

```bash
cd libs/storybook
npm ci
npm start
```

Storybook listens on `http://localhost:6006`. Its MCP endpoint is `http://localhost:6006/mcp`. The fixed port keeps local tool configuration stable. Stop an existing listener before starting another instance.

Local MCP configuration belongs in the ignored `.codex/config.toml`; `.codex/config.example.toml` supplies a template. Keep credential-bearing endpoints out of version control and logs. Do not expose the development MCP endpoint publicly.

## Library and previews

Components and pages are exported from the library. Stories use those same implementations with sample data and local callbacks. They do not require the application backend or perform real authentication, email delivery, or provider requests. The portal supplies application routing and API integration.

## Verification and builds

Run from `libs/storybook/`:

```bash
npm run lint
npm run check
npm run build
npm run build:storybook
npm test
```

`build` produces ESM, declarations, and styles in `dist/`. `build:storybook` produces a static preview in `storybook-static/`. Neither command starts Rust or runs database migrations. `lint` applies ESLint fixes and Prettier formatting with a 128-character line limit.

The test command starts Storybook when needed. Install Chromium with `npx playwright install chromium` if the local Playwright browser is missing.

See [portal architecture](../portal/adr.md) and [component contracts](../portal/components.md).
