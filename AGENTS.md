# Agent instructions

Before planning or making changes in this repository, you MUST read [docs/content/architecture.md](docs/content/architecture.md) in full and follow its architectural decisions. Do not rely on a summary from a previous session. Read the root README for project commands and the relevant feature documents for business rules.

Use pragmatic domain-driven design, concrete external-service clients, and scoped connection contexts as described in that document. Core's connection context uses SeaORM directly; keep database models and queries in the PostgreSQL crate. Do not introduce custom database-driver traits, strict Clean Architecture layers, generic network traits, or custom units of work without a concrete requirement.

When a task is discussion or documentation only, do not implement the discussed integrations or refactors. Keep unresolved business rules explicit. If an authorized change affects an architectural decision, update the architecture document with the change.

Preserve unrelated working-tree changes. Follow explicit instructions not to execute migrations; application startup and database-backed tests can apply them automatically.
