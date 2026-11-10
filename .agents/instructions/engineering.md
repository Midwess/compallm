# Engineering

Use pragmatic domain-driven design, concrete external-service clients, and scoped connection contexts.
Core defines the connection lifecycle trait without depending on SeaORM or Postgres; the PostgreSQL crate depends on Core and implements that trait.
Keep SeaORM connections, transactions, models, and queries in the PostgreSQL crate.
Do not introduce strict Clean Architecture layers, generic network traits, or custom units of work without a concrete requirement.

Do not add generic `created_at` or `updated_at` columns to migrations or matching models.
Keep timestamps only when required by a concrete business rule.

Use `From`/`TryFrom` for type conversions instead of standalone mapper functions.
When a conversion needs multiple inputs, use a tuple source, such as `TryFrom<(Model, String)>`,
and destructure it in `try_from`. When Rust orphan rules prevent implementing a tuple conversion directly for a foreign domain type, use a small local conversion wrapper and convert it into the domain type; do not move database types into Core or introduce a custom conversion trait.
Keep persistence conversions in the PostgreSQL crate.

Use Validify for HTTP normalization and shared domain validation.
Keep domain invariants in Core and persistence conversions and concurrency guards in PostgreSQL.
External integrations use concrete clients with reused pools and bounded timeouts.
Never hold database transactions open across email or HTTP calls.

Validation is Rust-first: define input rules in Rust and export them to the shared contract for Portal and Storybook.
Field-error codes, parameters, and messages should follow the same path when implemented; detailed error-message generation is deferred, so do not add independent UI validation messages or error tooltips as part of rule integration.

When writing repository implementation, always use sea-orm syntax, never write raw sql

Only use transaction! when there are multiple update command, transaction must be ended as soon as possible while still maintaining correctness
If there are multiple query before updating data, then the query should performed outside of transaction
Transaction only for grouping multiple update command
