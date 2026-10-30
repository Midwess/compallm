# Portal design workspace

Read the root agent instructions and `docs/content/development/design-workspace.md` before design work. Keep user prompting in Codex CLI. Use the local Storybook MCP tools to inspect available components and stories when connected; source files remain authoritative and can be edited directly.

- `pnpm start` launches the workbench at `http://localhost:6006`, with MCP at `/mcp`.
- Base-component previews live under `Components`, organized by Penpot Design foundations family. Keep feature screens separate and use sample data. Do not connect prototypes to APIs, authentication, Rust, or databases unless the user authorizes that work.
- Reusable components belong in `../portal/src/components/ui/`. Stories import them directly; do not create a separate copy of production components here.
- Prototype-only screen compositions and fixture data belong in `stories/`. Promote reviewed compositions into the portal when application implementation is authorized.
- Main design: dark neutral surfaces, violet primary actions, Inter typography, and Phosphor Regular SVG icons. Reuse the shared Tailwind v4 theme in `portal/src/styles/theme.css`.
- Add meaningful variants for loading, empty, disabled, and error states where relevant. Keep semantic HTML, keyboard focus, and responsive behavior.
- Run `pnpm check` and `pnpm build` after changes. Do not run the portal's backend-backed build merely to verify stories.
