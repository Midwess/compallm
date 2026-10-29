---
title: Portal design workspace
---

# Portal design workspace

Use Storybook to design and review actual portal components and screens with Codex CLI. The workbench runs independently of the application backend. It is a live browser preview, not a vector drawing canvas.

## Start the workspace

Requires Node.js 22.12 or newer and pnpm. From the repository root:

```bash
cd storybook
pnpm install
pnpm start
```

Open `http://localhost:6006`. The port is fixed so Codex's MCP connection remains predictable. If another process owns the port, stop that process or the other Storybook instance first.

Keep the preview running and use Codex CLI in another terminal at the repository root. The project MCP configuration points to `http://localhost:6006/mcp`. Codex loads project configuration only for trusted projects; restart Codex after adding the configuration. Use `/mcp` to check the connection. No second AI account or design editor is required; normal Codex usage still applies.

Copy `.codex/config.example.toml` to `.codex/config.toml` on a new checkout. The local configuration is ignored by Git because a remote Penpot URL can contain a personal access token. It does not change approval, sandbox, or model settings. Start Storybook before starting Codex to avoid an unavailable-server warning.

## Optional Penpot canvas

An externally hosted Penpot instance can expose editable designs to the same Codex CLI session through its remote MCP server. In the local `.codex/config.toml`, add `[mcp_servers.penpot]` with the `url` supplied by Penpot's account integrations. Keep the full URL private: its `userToken` query parameter authenticates the connection. Do not place it in the example configuration or documentation.

Open the intended Penpot file and connect it through File → MCP Server → Connect. Keep the connected tab awake while designing. Restart Codex after configuration changes and check `/mcp`. A reachable MCP server alone does not establish that the design-file plugin is connected; verify with a read-only inspection of the active page.

Use Penpot for visual explorations, then ask Codex to implement the approved design with the existing Storybook components. Canvas components and React components are separate artifacts; there is no automatic synchronization. Local `pnpm start` continues to launch only Storybook; the external Penpot instance runs independently.

Keep shared visual rules, tokens, and base components on **Design foundations**. Complete sidebars, full forms, feature panels, and screens belong on dedicated feature pages. Do not add a Page patterns gallery. Feature-specific assemblies live with their screens and reuse foundation instances. Login and signup continue to use shared field, code-entry, button, and feedback bases.

Bind Auto Layout gaps and padding to shared spacing tokens. When editing token values through the deployed Penpot plugin API, refresh the active token set to propagate the change; assigning `value` alone does not recalculate existing layouts. Verify a representative layout, then restore any temporary test value before review. Do not repeatedly apply an already-bound token: that operation toggles the binding off in this version.

Use the [shared spacing policy](../portal/spacing.md) for agent-driven density changes. `layout.spacing.scale = 0.5` halves prior spacing; `layout.spacing.minimum = 2` clamps nonzero gaps and padding. Foundation masters and screen layouts must reference these constants, including migrated legacy token names.

## Design and review

Begin with the wireframe story. Agree on information hierarchy and screen composition before choosing final colors or adding business behavior. Sample content illustrates layout; it does not establish organization permissions, model access, or release scope.

Ask Codex to inspect existing stories and components, then describe a change:

> Inspect the portal components and wireframe stories. Propose a notebook wireframe using the existing components and sample data. Keep the palette provisional and show an empty state as a separate story.

Edit shared React components in `portal/`; keep stories, sample data, and prototype-only compositions in `storybook/`. Changes appear through hot reload. Save meaningful states as named stories so they remain reviewable. When a screen is approved, compose the real portal page from those same components; routing and API integration are separate authorized tasks.

Dark mode is the initial target. Pixelarticons may supply selected nostalgic accents; do not turn all typography, controls, or animation into a retro theme. Palette samples are proposals, not an accepted brand system. Respect reduced-motion preferences and give controls visible keyboard focus.

## Verify and share

From `storybook/`:

```bash
pnpm check
pnpm build
pnpm test
```

The build produces `storybook-static/`, which can be served as a static review site. It does not build the Rust application or run migrations. Do not publish the development MCP endpoint as a public service.

Browser checks verify story rendering, backend independence, keyboard navigation, and mobile overflow. On a machine without Playwright's browser, run `pnpm exec playwright install chromium` once. The test command starts Storybook automatically if it is not already running.

## References

- [Storybook AI workflow](https://storybook.js.org/ai/)
- [Codex MCP configuration](https://developers.openai.com/codex/mcp/)
- [Development decisions](adr.md)

When reorganizing Penpot pages through MCP, explicitly save a named file version after each page creation or transfer before navigating away. The plugin’s successful return confirms the in-memory operation; verify the saved revision and page list before reporting completion.

Serialize Penpot MCP tasks. Never overlap page changes, mutations, exports that navigate, or snapshot operations: they share one active browser page. A shell session ID means the request is still running, not that it is safe to start another task. Wait for completion before the next MCP request. After structural changes, validate the entire file and reopen each affected page to check its contents. Inspect fresh exports when checking appearance; an earlier screenshot does not verify current state.

## Local Penpot workbench

The ignored `.dev/penpot/` JavaScript project holds local agent instructions, a cached design catalog, and reusable MCP helpers. Read its `agent.md` before canvas work. Run scripts directly, for example `node .dev/penpot/status.js`, `node .dev/penpot/inspect.js --help`, and `node .dev/penpot/edit.js --help`. The bridge serializes requests and reuses its MCP session. It reads the private endpoint from `.codex/config.toml`; do not copy that URL into jobs, notes, logs, or generated caches.

Routine edits should need one targeted inspection, one batched edit with checks, and one save. Keep base controls in Design foundations and feature compositions on their dedicated pages. Do not regenerate HTML prototypes, screenshots, reports, or broad inventories for every small adjustment. Repository documentation requirements still apply to changes in behavior, architecture, or tooling.
