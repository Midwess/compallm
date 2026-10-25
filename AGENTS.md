# Agent instructions

Before planning or making changes in this repository, you MUST read [docs/content/architecture.md](docs/content/architecture.md) in full and follow its architectural decisions. Do not rely on a summary from a previous session. Read the root README for project commands and the relevant feature documents for business rules.

Use pragmatic domain-driven design, concrete external-service clients, and scoped connection contexts. Core defines the connection lifecycle trait without depending on SeaORM or Postgres; the PostgreSQL crate depends on Core and implements that trait. Keep SeaORM connections, transactions, models, and queries in the PostgreSQL crate. Do not introduce strict Clean Architecture layers, generic network traits, or custom units of work without a concrete requirement.

When a task is discussion or documentation only, do not implement the discussed integrations or refactors. Keep unresolved business rules explicit. If an authorized change affects an architectural decision, update the architecture document with the change.

Preserve unrelated working-tree changes. Follow explicit instructions not to execute migrations; application startup and database-backed tests can apply them automatically.
