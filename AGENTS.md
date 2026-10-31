# Agent instructions

Before planning or making changes in this repository, you MUST read [docs/content/development/adr.md](docs/content/development/adr.md) in full and follow its architectural decisions. Read the relevant feature's `docs/content/<feature-name>/adr.md` and sub-feature pages as well. Do not rely on a summary from a previous session. Read the root README for project commands and the relevant feature documents for business rules.

Use pragmatic domain-driven design, concrete external-service clients, and scoped connection contexts. Core defines the connection lifecycle trait without depending on SeaORM or Postgres; the PostgreSQL crate depends on Core and implements that trait. Keep SeaORM connections, transactions, models, and queries in the PostgreSQL crate. Do not introduce strict Clean Architecture layers, generic network traits, or custom units of work without a concrete requirement.

When a task is discussion or documentation only, do not implement the discussed integrations or refactors. Keep unresolved business rules explicit. If an authorized change affects an architectural decision, update the relevant ADR with the change.

Follow documentation first for new features and changes to behavior or architecture. Complete the relevant documentation before changing application code, migrations, or configuration. For drafting, revising, or reviewing documentation, read and apply the [Technical Writing skill](.agents/skills/technical-writing/SKILL.md). Organize documentation directly by feature: `docs/content/<feature-name>/adr.md` owns decisions and open questions, and `<feature-name>/<sub-feature>.md` owns each focused flow and its configuration. Keep shared engineering decisions in `development/adr.md`. Do not group pages under central `features/` or `adr/` folders or recreate a central business-decisions page. ADRs describe the latest architecture and solution directly: responsibilities, data flow, contracts, constraints, and relevant operational guidance. State briefly when a solution is planned rather than implemented, and keep unresolved requirements explicit. A documentation-only request does not authorize implementation afterward.

Generated public API documentation lives under `docs/content/api/`, organized by endpoint URL path and HTTP method, rather than under the implementing feature. It is a guide for API consumers, separate from internal ADRs and generation instructions. Keep descriptions sourced from Rust/OpenAPI and preserve them in generated TypeScript JSDoc for editor hover.

Write documentation only about the current architecture and direct solution. Describe the latest decision in place; do not retain decision dates, timelines, superseded alternatives, review history, implementation diaries, or PR/branch narratives. Do not document visual interface design, screen layouts, colors, spacing, typography, mockups, design references, or canvas-editing workflows. Keep application behavior, component architecture, security, API contracts, and necessary setup/operational instructions when they explain the solution. Apply this rule when creating or revising any documentation, including ADRs; it takes precedence over conflicting historical/status templates in writing skills. Visual specifications belong in design assets and component code, not documentation.

Every feature ADR must take less than six minutes to read, including diagrams and tables. Apply the estimate and editing checks in the [Technical Writing skill](.agents/skills/technical-writing/SKILL.md). The [writing guidelines](docs/content/development/writing.md) explain the convention for human readers. The six-minute ceiling is a project decision, not a rule attributed to the handbook.

Preserve unrelated working-tree changes. Follow explicit instructions not to execute migrations; application startup and database-backed tests can apply them automatically.

Write tests only in `tests/` directories, using the existing `test/tests/` suite for shared Rust tests. Do not add tests under `api/` or inline in application source. Test the application's core business logic; do not write tests that only verify dependency or framework behavior.

Do not add generic `created_at` or `updated_at` columns to migrations or matching models. Keep timestamps only when required by a concrete business rule.

Only use transaction! when there are multiple update command, transaction must be ended as soon as possible while still maintaining correctness
If there are multiple query before updating data, then the query should performed outside of transaction
Transaction only for grouping multiple update command
