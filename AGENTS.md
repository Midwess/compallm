# Agent instructions

Before planning or making changes in this repository, you MUST read [docs/content/development/adr.md](docs/content/development/adr.md) in full and follow its architectural decisions. Read the relevant feature's `docs/content/<feature-name>/adr.md` and sub-feature pages as well. Do not rely on a summary from a previous session. Read the root README for project commands and the relevant feature documents for business rules.

Use pragmatic domain-driven design, concrete external-service clients, and scoped connection contexts. Core defines the connection lifecycle trait without depending on SeaORM or Postgres; the PostgreSQL crate depends on Core and implements that trait. Keep SeaORM connections, transactions, models, and queries in the PostgreSQL crate. Do not introduce strict Clean Architecture layers, generic network traits, or custom units of work without a concrete requirement.

When a task is discussion or documentation only, do not implement the discussed integrations or refactors. Keep unresolved business rules explicit. If an authorized change affects an architectural decision, update the relevant ADR with the change.

Follow documentation first for new features and changes to behavior or architecture. Complete the relevant documentation before changing application code, migrations, or configuration. For drafting, revising, or reviewing documentation, read and apply the [Technical Writing skill](.agents/skills/technical-writing/SKILL.md). Organize documentation directly by feature: `docs/content/<feature-name>/adr.md` owns decisions and open questions, and `<feature-name>/<sub-feature>.md` owns each focused flow and its configuration. Keep shared engineering decisions in `development/adr.md`. Do not group pages under central `features/` or `adr/` folders or recreate a central business-decisions page. ADRs include context, decision, consequences, decision status, and implementation status; leave unaccepted proposals explicit. Documentation describes the intended design and must distinguish it from implemented behavior. A documentation-only request does not authorize implementation afterward.

Generated public API documentation lives under `docs/content/api/`, organized by endpoint URL path and HTTP method, rather than under the implementing feature. It is a guide for API consumers, separate from internal ADRs and generation instructions. Keep descriptions sourced from Rust/OpenAPI and preserve them in generated TypeScript JSDoc for editor hover.

Keep feature documentation focused on ADRs and explanations useful to users and the team: decisions, behavior, tradeoffs, and relevant operational or compatibility guidance. Do not create implementation diaries, progress reports, or pages narrating source layout, query mechanics, or PR/branch history. Include implementation details only when needed to understand a decision or perform a task; keep routine mechanics in code and PR descriptions.

Every feature ADR must take less than six minutes to read, including diagrams and tables. Apply the estimate and editing checks in the [Technical Writing skill](.agents/skills/technical-writing/SKILL.md). The [writing guidelines](docs/content/development/writing.md) explain the convention for human readers. The six-minute ceiling is a project decision, not a rule attributed to the handbook.

Preserve unrelated working-tree changes. Follow explicit instructions not to execute migrations; application startup and database-backed tests can apply them automatically.

## Project structure

This is a Rust workspace with a React portal and a Quartz documentation site:

```text
api/                    Loco/Axum server, HTTP controllers, application startup,
                        configuration, dependency injection, and API contracts
libs/core/              Business types, domain services, persistence contracts,
                        scoped database context, and focused service interfaces
libs/postgres/          PostgreSQL/SeaORM connection adapter, generated schema,
                        domain mappers, concrete repositories, and migrations
libs/utils/             Small reusable stateless helpers; domain behavior does
                        not belong here
test/                   Cross-crate and integration tests
portal/                 React/Vite frontend and generated API contract
config/                 Development, test, and production server configuration
docs/content/           Product, feature, business-decision, and architecture docs
docs/web/               Quartz documentation website implementation and output
```

The dependency direction is `api -> core + postgres`, `postgres -> core`, and `core -> utils`. Core does not depend on SeaORM or Postgres. The API owns concrete dependency wiring in `api/src/di.rs`; do not construct repositories or database connections inside controllers or Core services.

### Core source organization

Organize `libs/core/src/` by business capability, not by global technical layers:

```text
auth/                   Authentication workflow, errors, and token types
mail/                   Email request/template types and delivery interface
organization/           Organization entities, rules, errors, and services
user/                   User entities, rules, errors, and services
persistent/             Shared connection lifecycle, DbContext, and repository
                        contracts grouped by domain
lib.rs                  Public top-level modules only
```

For a new capability such as provider management, prefer this shape:

```text
libs/core/src/provider/
  mod.rs                Module declarations and deliberate public re-exports
  entity.rs             Provider domain types and invariants
  error.rs              Provider-specific service/domain errors
  management-service.rs Concrete provider-management workflow when required

libs/core/src/persistent/provider.rs
                        Provider persistence contract when Core needs one

libs/postgres/src/
  repository/provider.rs  SeaORM queries implementing that contract
  mappers/provider.rs     Persistence-to-domain mapping
  migration/...           Additive PostgreSQL schema changes
  schema/...              Generated SeaORM entities; do not hand-edit
```

Keep API request/response DTOs and authorization at the HTTP boundary. Do not expose SeaORM entities or secrets such as provider API keys through API responses. Services own workflow and transaction boundaries; repositories use the active scoped `DbContext` and must not open or commit an independent transaction.

### Writing a Core service

A service coordinates one business workflow inside its owning domain. Before adding one, check whether the behavior belongs on an existing entity or service; do not create a service that only renames a repository operation.

Follow the current `RegistrationService`, organization `CreationService`, and `ConfigService` patterns:

1. Put the service in its capability module, for example `libs/core/src/provider/management-service.rs`, and expose it deliberately through that module's `mod.rs`.
2. Give it a specific workflow-oriented name. Prefer `CreationService`, `ManagementService`, or a domain-specific name over generic names such as `ProviderManager` or `Service`.
3. Store only required dependencies. Database workflows normally receive `&'static DbContext` and `&'static Box<dyn DomainRepository>` through `new`; cross-domain workflows may receive another concrete Core service.
4. Do not access `Di`, Axum state, Loco context, environment variables, SeaORM models, or PostgreSQL types from Core services. The API composition root supplies configured dependencies.
5. Validate and normalize domain input before persistence. Put reusable invariants on the domain type where practical; keep HTTP-only parsing and authorization in the controller.
6. Generate application-owned IDs in the service with the established Hero ID mechanism before calling the repository. Do not rely on database sequences when the schema uses application-generated IDs.
7. Use `transaction!(self.db, { ... })` when the workflow writes data, performs multiple dependent repository operations, or must join a caller's transaction. Read-only operations can use `self.db.scope(...)` when they need an active context without a transaction.
8. Keep the transaction boundary in the outer workflow service. Nested services join the current scoped transaction; repositories must use `DbContext::current()` and must not begin, commit, or switch connections independently.
9. Do not hold a database transaction open while calling a slow external API. Perform network work outside the transaction unless a documented workflow requires durable coordination such as an outbox.
10. Return Core domain types or workflow-specific result types, never SeaORM models or API DTOs. Do not return credentials or secret-bearing values unless the workflow explicitly requires a protected server-only type.
11. Define a capability-specific error enum in `error.rs`. Preserve actionable domain outcomes such as invalid input, not found, or duplicate name; wrap lower-level database/internal causes without exposing them to clients.
12. Keep methods focused on business orchestration. SQL and persistence filtering belong in the PostgreSQL repository; HTTP status codes, headers, and response wording belong in the API controller.

A typical database-backed service has this shape:

```rust
pub struct ManagementService {
    db: &'static DbContext,
    #[allow(clippy::borrowed_box)]
    providers: &'static Box<dyn ProviderRepository>,
}

impl ManagementService {
    #[allow(clippy::borrowed_box)]
    pub fn new(
        db: &'static DbContext,
        providers: &'static Box<dyn ProviderRepository>,
    ) -> Self {
        Self { db, providers }
    }

    pub async fn create(
        &self,
        organization_id: u64,
        input: NewProvider,
    ) -> Result<Provider, ProviderManagementError> {
        let input = input.validate()?;
        transaction!(self.db, {
            self.providers
                .create(Provider {
                    id: hero::distributed_id::try_gen_id().await?,
                    organization_id,
                    name: input.name,
                    url: input.url,
                    api_protocol: input.api_protocol,
                })
                .await
                .map_err(Into::into)
        })
    }
}
```

Treat the example as a shape, not an instruction to add those exact provider types. Credential protection, authorization, update behavior, and duplicate rules must follow the feature's decided business rules.

Test a service at the narrowest useful level:

- Test pure entity validation without a database.
- Test service orchestration, transaction participation, and error propagation with existing repository test doubles or SeaORM mocks through the PostgreSQL adapter.
- Use PostgreSQL integration tests for constraints, rollback, and concurrency behavior that mocks cannot reproduce.
- Test external clients separately against Wiremock; do not call paid or production services.
- Verify that failures do not commit partial writes and do not expose secrets in errors or debug output.

### Writing a repository

A repository implements persistence operations needed by a business capability. Define the contract in Core and keep its PostgreSQL/SeaORM implementation in `libs/postgres`; do not place SQL, SeaORM types, or database constraint names in domain services.

#### Core repository contract

Create `libs/core/src/persistent/<domain>.rs` when a Core service has a concrete persistence need:

1. Define a focused `#[async_trait]` repository trait whose methods use Core domain types, primitives, and domain-specific errors only.
2. Name methods after required persistence behavior, such as `find_by_id`, `create`, or `update_name`. Do not expose generic query builders, SeaORM filters, active models, or transactions.
3. Keep the contract as small as the current workflows require. Do not add speculative CRUD methods or a generic base repository.
4. Define the repository error beside the contract. Include actionable persistence outcomes, such as `NotFound` or `NameAlreadyExists`, only when callers need to distinguish them; wrap all other failures as a database/internal cause.
5. Implement `From<ConnectionError>` so failure to access the active scope propagates through the repository error.
6. Register the module in `libs/core/src/persistent/mod.rs`. Do not re-export every repository from the Core crate root unless that is an established convenience needed by callers.

Typical contract shape:

```rust
#[derive(Debug, thiserror::Error)]
pub enum ProviderRepositoryError {
    #[error("provider not found")]
    NotFound,
    #[error("provider name is already in use")]
    NameAlreadyExists,
    #[error("database operation failed")]
    Database(#[from] anyhow::Error),
}

#[async_trait::async_trait]
pub trait ProviderRepository: Send + Sync {
    async fn find_by_id(
        &self,
        provider_id: u64,
    ) -> Result<Option<Provider>, ProviderRepositoryError>;

    async fn create(
        &self,
        provider: Provider,
    ) -> Result<Provider, ProviderRepositoryError>;
}
```

#### SeaORM schema names and domain mapping

SeaORM generates a struct named `Model` in every table module. Do not import or expose ambiguous names such as `users::Model`. Compallm's entity-generation step rewrites `libs/postgres/src/schema/mod.rs` into a persistence facade with meaningful public aliases:

```text
users::Model                         -> UserSchema
organizations::Model                 -> OrganizationSchema
organization_members::Model          -> OrganizationMemberSchema
sea_orm_active_enums::UserStatus     -> UserStatusSchema
```

The naming convention is the singular table name in PascalCase plus `Schema`. Generated PostgreSQL enum names also receive the `Schema` suffix. Use these names to make the persistence/domain distinction explicit:

```rust
use compallm_core::user::User;                  // Domain type
use crate::schema::{UserSchema, UserStatusSchema}; // Persistence types
```

These aliases are generated by `libs/postgres/src/codegen.rs` when running:

```sh
cargo run -p compallm-postgres --features cli -- generate-entity
```

Follow these rules:

1. Do not hand-edit files under `libs/postgres/src/schema/`, including `schema/mod.rs`; entity generation replaces them.
2. Do not manually add aliases for a newly generated table or enum. Regenerate entities so `SchemaAliases` creates and validates them consistently.
3. Keep generated table modules and original SeaORM `Model`/enum paths private. Import the public `*Schema` aliases in repositories, mappers, tests, and PostgreSQL-only code.
4. `*Schema` always means a database representation. Never use it as a Core entity or an API response, and never import it into `libs/core`.
5. Core domain types use business names without the suffix, such as `User`, `Organization`, or `Provider`. API response DTOs use explicit response names rather than schema aliases.
6. Convert a generated schema value into its domain type in `libs/postgres/src/mappers/<domain>.rs`, normally with `impl TryFrom<DomainSchema> for Domain`.
7. Make mapping fallible whenever stored data can violate application invariants or conversion ranges. Add context identifying the invalid field, but do not include credentials or other secrets in errors.
8. Map database enums to domain enums exhaustively. Do not pass generated `*Schema` enum values through Core boundaries.
9. Keep persistence-only columns out of the domain type unless the business uses them. For example, a mapper may intentionally omit storage timestamps from a domain entity.
10. If entity generation reports an alias collision, resolve the table/enum naming conflict; do not bypass the check with a second public alias.

Typical mapper shape:

```rust
impl TryFrom<UserSchema> for User {
    type Error = UserRepositoryError;

    fn try_from(model: UserSchema) -> Result<Self, Self::Error> {
        let id = crate::from_bigint(model.id)
            .context("invalid user ID in database")?;
        if id == 0 {
            return Err(UserRepositoryError::database(anyhow::anyhow!(
                "user ID in database must be positive"
            )));
        }

        Ok(Self {
            id,
            email: model.email,
            display_name: model.display_name,
            status: match model.status {
                UserStatusSchema::Active => UserStatus::Active,
                UserStatusSchema::Disabled => UserStatus::Disabled,
            },
        })
    }
}
```

#### PostgreSQL implementation

Create `libs/postgres/src/repository/<domain>.rs`, following the existing user and organization repositories:

1. Define a small concrete repository, usually a stateless unit struct with `#[derive(Default)]`, and implement the Core repository trait for it.
2. For every operation, obtain `DbContext::current()` and convert it with `PostgresConnection::from_context(&context)`. This preserves the caller's scoped connection or transaction.
3. Never create a new database connection, begin or commit a transaction, or fall back to an application-global connection inside a repository.
4. Use the generated schema aliases exported by `compallm_postgres::schema`, plus local type aliases for SeaORM entity, column, and active-model types when they improve readability.
5. Convert application `u64` IDs with `to_bigint` and persisted IDs with `from_bigint`. Reject zero, negative, and overflowing IDs explicitly rather than casting with `as`.
6. Build `ActiveModel` values explicitly and use `..Default::default()` only for columns intentionally supplied by database defaults. Map domain enums exhaustively to generated PostgreSQL enum types.
7. Return Core domain types. Put reusable schema-to-domain conversions in `libs/postgres/src/mappers/<domain>.rs` as fallible `TryFrom<Schema> for Domain` implementations; validate corrupt or out-of-range stored values during mapping.
8. Treat `Option<T>` and `NotFound` deliberately. Lookups normally return `Ok(None)` when absence is expected; update/delete workflows may return a typed `NotFound` when exactly one row is required.
9. Translate only known database constraints into specific repository errors. Match the exact named constraint, such as a unique index, and leave unrelated unique, foreign-key, check, and driver failures as database errors.
10. When an update or delete expects one target, inspect `rows_affected` or use `exec_with_returning`; do not report success after changing zero or unexpected multiple rows.
11. Set persistence-owned timestamps consistently with the existing schema and repository conventions. Do not duplicate a database default in Core unless the timestamp is part of a business rule.
12. Register the implementation in `libs/postgres/src/repository/mod.rs`, register mapper modules in `libs/postgres/src/mappers/mod.rs`, and wire the concrete repository in `api/src/di.rs`.

Typical scoped implementation shape:

```rust
#[derive(Default)]
pub struct ProviderRepository;

#[async_trait::async_trait]
impl CoreProviderRepository for ProviderRepository {
    async fn find_by_id(
        &self,
        provider_id: u64,
    ) -> Result<Option<Provider>, ProviderRepositoryError> {
        let id = database_id(provider_id)?;
        let context = DbContext::current()?;
        let db = PostgresConnection::from_context(&context)?;

        Providers::find_by_id(id)
            .one(db)
            .await
            .map_err(ProviderRepositoryError::database)?
            .map(Provider::try_from)
            .transpose()
    }
}
```

Treat this as a structural example, not a requirement to add those exact provider operations. In particular, secret-bearing fields require explicit types and mapping rules: do not derive or log `Debug` for plaintext credentials, and do not return an API key from ordinary lookup/list operations.

#### Repository tests

- Use SeaORM `MockDatabase` tests to inspect generated queries, mapped values, expected timestamps, scoped transaction reuse, missing rows, and generic failure propagation.
- Test exact constraint translation: only the intended constraint may become a duplicate/conflict domain outcome.
- Test invalid application IDs and invalid persisted IDs at conversion boundaries.
- Test domain enum mappings in every direction used by the repository.
- Use a real PostgreSQL integration test for unique/check/foreign-key constraints, rollback, locking, and concurrent writes. MockDatabase does not prove PostgreSQL behavior.
- Verify multiple repository calls inside one service transaction produce one `BEGIN`/`COMMIT` pair and never switch connections.

### API controllers and dependency injection

Create one controller module per API capability under `api/src/controllers/`, following `auth.rs`:

1. Define a `router()` returning `OpenApiRouter<State>` and register every handler with `routes!(handler)`.
2. Keep HTTP request and response DTOs in the controller. Derive `Deserialize`/`Serialize` and `utoipa::ToSchema`; use `#[serde(rename_all = "camelCase")]` when the JSON contract has multiword fields.
3. Describe every endpoint with `#[utoipa::path(...)]`, including method, `/api/...` path, stable `operation_id`, tag, request body, and all documented response statuses.
4. Extract dependencies with Axum `State`, parse JSON as `Result<Json<T>, JsonRejection>`, and return `Result<impl IntoResponse, ApiError>`.
5. Keep handlers thin: parse transport input, authorize the caller, invoke one Core service workflow, and map the result into an explicit API response. Business validation and persistence queries do not belong in controllers.
6. Add the module to `api/src/controllers/mod.rs` and merge its router into `api_router()` so it is included in both runtime routes and the generated OpenAPI document.
7. Add new error variants and safe HTTP mappings in `api/src/error.rs`. Log internal causes server-side, but do not return database details, provider responses, credentials, tokens, or other secrets to clients.
8. Add request tests under `api/src/tests/requests/` using `axum_test::TestServer` and a test `Di`. Cover successful behavior, malformed input, authorization, domain failures, safe error bodies, and relevant headers/status codes.

The current API router uses `&'static AuthService` directly as its Axum state. That is acceptable while every route uses that one service. When a second controller needs a different service, introduce one small cloneable API state instead of nesting routers with unrelated state types or accessing the global `Di` from handlers:

```rust
#[derive(Clone, Copy)]
pub struct ApiState {
    pub auth: &'static AuthService,
    pub providers: &'static ProviderManagementService,
}
```

At that point, make `controllers::api_router()` and each merged controller router use `OpenApiRouter<ApiState>`. A handler then extracts `State(state): State<ApiState>` and selects the required service. Only expose dependencies that handlers actually need. Do not place database connections or repositories in `ApiState`.

The DI container in `api/src/di.rs` is the composition root. To wire a new repository and service:

1. Add the Core repository contract and its concrete PostgreSQL implementation first.
2. Add the concrete repository as a `Di` field, usually `Box<dyn ...Repository>` when the Core service consumes that contract.
3. Add the long-lived Core service as `OnceLock<Box<Service>>`. Services borrow permanent dependencies only after `Di` has reached its static address.
4. Construct repositories in `Di::new`; do not construct them in controllers or service methods.
5. Add repository and service getter methods. Build a service lazily with `get_or_init`, passing `db_context()`, repository getters, configuration, and any other required services.
6. Eagerly invoke fallible service constructors from `initialize_services()` so invalid configuration fails during startup rather than on the first request. Extend its error type deliberately if the new service has a different initialization error.
7. In `inject()`, build the controller state from service getters, call `.with_state(...)` once on the complete API router, retain the middleware that wraps every request in `db.scope(...)`, and merge the resulting routes into Loco's router.
8. Extend `Settings` and the files under `config/` only for actual application configuration. Secrets must not have committed production defaults.

Use this wiring flow:

```text
request -> controller DTO/authorization -> Core service
        -> Core repository contract -> PostgreSQL repository
        -> Core domain result -> explicit controller response DTO
```

After changing API routes or DTOs, run the relevant Rust tests and regenerate the contract with `cargo run -p compallm-api -- generate` when the task includes generated API artifacts. Review changes to both `portal/openapi.json` and `portal/src/contract/api.ts`; do not hand-edit generated contract files.
