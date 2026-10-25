---
title: Architecture
---

# Architecture

Compallm follows pragmatic domain-driven design (DDD). Organize code around business capabilities and keep rules close to the types they govern. Use modules, concrete services, and focused clients before adding abstraction layers.

These decisions guide implementation. The current scaffold does not necessarily implement every boundary yet. An architecture discussion or documentation change is not an instruction to implement the discussed feature.

## Workspace responsibilities

| Location | Responsibility |
| --- | --- |
| `api/` | Loco/Axum application, controllers, HTTP input/output, cookies, configuration, and dependency wiring |
| `libs/core/` | Shared business types, domain services, token behavior, and focused external-service clients |
| `libs/utils/` | Reusable stateless utility functions; no business rules or application state |
| `test/` | Core behavior and workflow tests as a separate workspace crate; no production application code |
| `libs/postgres/` | SeaORM entities, repository queries, connection access, and migrations |
| `portal/` | React frontend built with Vite |
| `docs/content/` | Human-readable product plans, decisions, and architecture |
| `docs/web/` | Quartz documentation website |

Axum is the public entry point for the frontend and API. In development it proxies Vite and hot reload; in production it serves Vite's built static files.

## Organize by business domain

Authentication, users, organizations, routing, and gateway usage are business capabilities. Group their types and behavior together rather than creating project-wide folders for every technical layer.

The authentication domain starts with:

```text
libs/core/src/auth/
  service.rs
  token.rs
  token/
    codec.rs
    email_otp.rs
    encrypted.rs
    user.rs
    refresh.rs
```

`AuthService` exposes `signin`, `signup`, and `refresh`, plus its constructor. It coordinates a supplied database connection, user repository, token codec, and explicit token lifetimes. Token creation, email-code issuance, parsing, and cryptographic validation belong to `TokenCodec` and token types, not additional public methods on `AuthService`.

`signin` verifies an email token/code for an existing active user and records the login. `signup` verifies the same proof, requires a non-empty display name, and creates an active user; it rejects an already registered email. Both workflows execute inside one transaction scope shared by their repositories and return credentials only after commit. They are separate entry points rather than implicitly registering unknown users during signin.

Each token struct owns its claims, invariants, and validation behavior. `AccessToken` has `Encrypted(EncryptedToken(String))`, `EmailOtp(EmailOtpAccessToken)`, `User(UserAccessToken)`, and `Refresh(RefreshToken)` variants. `EmailOtp` is an internal variant that must be encrypted before becoming the outer JWT payload; the codec rejects plaintext email-OTP JWTs.

Hero's `token::jwt` helpers sign and validate the outer JWT. After Hero verifies its signature and expiration, the service validates the variant's claims and strictly checks their timestamps. Startup passes one strong signing secret into `AuthService::new`; `EncryptedAccessToken::new` receives that same secret through its constructor. Encryption uses an HKDF-derived, domain-separated 32-byte key so no second configured secret is required and the raw signing key is not reused as the cipher key. Auth modules do not read environment variables themselves.

`EncryptedAccessToken::encrypt` accepts a plaintext `AccessToken` and returns `Result<AccessToken, AuthError>` containing its encrypted wrapper. `decrypt` accepts the wrapper and returns the decrypted, validated `AccessToken`; callers then use standard Rust `TryFrom<AccessToken>` conversions. Inner payloads retain their explicit enum variant, allowing future encrypted token types without ambiguous shape-based parsing. A different valid type yields `WrongTokenVariant`; authentication, integrity, and expiry failures must not be treated as a reason to bypass validation. Nested encrypted wrappers are rejected.

User and refresh payloads are signed, not encrypted; do not add secrets to their claims. IDs remain ordinary `u64` values in server-side token claims; the browser treats tokens as opaque strings. Token structs deliberately do not implement Debug. Email OTPs expire after 10 minutes; requesting another does not revoke old matching pairs and successful verification does not consume them. User and refresh lifetimes remain explicit inputs until policy is agreed.

The service integrates the user repository contract at `libs/core/src/persistent/user.rs`. Its PostgreSQL implementation, email delivery, HTTP endpoints, and persisted session/refresh handling remain separate work. `refresh` currently validates the refresh variant and checks that the user exists and is active, then issues a user access token while returning the original refresh credential unchanged. It does not extend that credential's expiry or claim replay detection, refresh rotation, or per-session revocation. Those require a defined storage model.

A domain module is not automatically a separate crate or bounded context. Split it when its business responsibilities or dependency needs justify that boundary.

## Core-owned entities

Core defines dedicated business entities in `libs/core/src/entity/`. `entity/user.rs` owns `User`, `UserStatus`, and the `NewUser` creation data. Business services and repository interfaces use these Core-owned types; they must not use or re-export SeaORM models as their business entities.

The persistent layer owns storage representations and maps them to and from Core entities. `libs/core/src/persistent/user.rs` defines storage operations, while `persistent/error.rs` centralizes persistence errors; neither defines the `User` entity. A storage schema change must not automatically change the domain entity or external API contract. Core may use SeaORM connection and transaction types through its scoped context. Business entities remain independent of SeaORM models.

## Function ownership and utilities

Avoid orphan (free-standing) business/helper functions outside `libs/utils/`. Put domain-specific behavior on the owning struct or enum as methods or associated functions. For example, token expiration rules and claim validation belong on token types, not in a miscellaneous utilities module.

Reusable stateless functions belong in `libs/utils/`, grouped by purpose. `compallm_utils::time::unix_timestamp()` reads the system clock; use it directly where the current Unix timestamp is needed instead of adding another `now()` wrapper inside a domain module. Utilities must not depend on Core, Postgres, Axum/Loco, or business configuration. Functions required by Rust entrypoints, framework callbacks, and tests are not orphan business helpers.

Keep IDs as `u64` in Rust and perform checked conversion for PostgreSQL BIGINT. If a future API response exposes large IDs for JavaScript to parse, address precision in that API DTO. Do not impose frontend-specific string serialization on internal token claims.

## Dependencies and third-party libraries

Core is not required to be dependency-free. Use established libraries for serialization, validation, cryptography, error handling, HTTP communication, and database execution when they support the domain's work.

- Keep framework-specific requests, responses, cookies, and HTTP status mapping in `api/`.
- Keep query and storage implementation details in `libs/postgres/`.
- Do not use SeaORM entities as API responses automatically; define the external contract deliberately.
- Keep provider URLs, authentication headers, and wire formats inside concrete integration clients.
- Avoid circular crate dependencies. Core owns repository contracts and a concrete SeaORM connection context. PostgreSQL repositories depend on Core and implement its contracts. `api/src/di.rs` constructs the context from Loco's configured database and supplies dependencies to the application.

There is no requirement for a trait around every library or service. Do not introduce repository interfaces, generic HTTP transport traits, or unit-of-work layers merely to imitate Clean Architecture. Add a trait when a concrete need appears, such as interchangeable implementations or a useful testing boundary.

## Persistence and transactions

The user repository interface is defined in `libs/core/src/persistent/user.rs`, with lookup, creation, and login-recording operations. Its methods take business arguments only; connection and ORM types are not repository method parameters. Concrete repositories fetch `DbContext::current()` for database access; they do not own fallback connections or receive database parameters on each method.

Repository interfaces and their implementations use `#[async_trait::async_trait]`, with every repository operation declared as `async fn` returning a normal `Result`; do not expose explicit `impl Future` signatures. All persistence errors belong in `libs/core/src/persistent/error.rs`. Repositories share `PersistenceError`, including typed failures such as `EmailAlreadyExists`, driver errors retained as `Database(anyhow::Error)`, and `General { message: String, source: anyhow::Error }` for other failures. Preserve the original error as the source.

### Async connection scopes

`libs/core/src/persistent/db_context.rs` owns a thin, cloneable `DbContext` context. It stores the concrete PostgreSQL connection directly as `postgres`, with no separate inner struct. `PostgresConnection` uses SeaORM's `ConnectionTrait` and represents either the configured database handle or an active transaction. SeaORM's trait has generic methods, so a concrete enum provides the shared query API instead of a trait object or custom `ConnectionDriver`.

`api/src/di.rs` constructs the context from the database already configured by Loco. This code does not create or manage a separate connection pool. Axum middleware scopes each API request. Background work must explicitly enter `db_context.scope(...)`; missing context is an error, not an implicit fallback connection.

Repositories fetch the context with `DbContext::current()?` and pass `context.postgres()` to their SeaORM queries. Services retain the configured context to establish scopes and transaction boundaries:

```rust
self.db.transaction(async {
    let user = self.users.find_by_email(email).await?;
    // Every repository reads the same current transaction context.
    Ok(user)
}).await
```

The wrapper starts a PostgreSQL transaction and scopes the supplied future with a context containing that transaction. It commits on success and rolls back on failure. Nested transactions use the current transaction's savepoints and restore the outer context when finished. Entering another application scope preserves the active context.

The supplied future runs on the caller's task. It can borrow local data and return borrowed values without `async move`, `Send`, or `'static` requirements. Tokio-spawned tasks do not inherit task-local values; keep transaction operations inside the scoped future.

Cancellation or panic closes the transaction context and releases transaction ownership, allowing SeaORM to roll back. Escaped context handles cannot query after completion. Database operations must finish before the scope exits; an outstanding operation prevents commit.

When another database is actually needed, add its concrete handle to `DbContext` and initialize it in `di.rs`. Do not add a generic driver interface in advance. The current transaction wrapper covers PostgreSQL only; adding another database does not imply atomic transactions across databases.

- Keep SeaORM models, repository queries, and migrations in `libs/postgres/`.
- Keep business entities and repository interfaces in Core.
- A repository must use the current context rather than silently choosing a different connection.
- Repositories participating in a larger operation must not independently commit it.
- Handle uniqueness conflicts and concurrent requests explicitly; atomicity alone does not prevent duplicate-signup races.
- Use the scoped context instead of adding a repository-container unit of work.

A database transaction does not cover email delivery or other HTTP calls. Avoid holding a transaction open during slow network requests. If delivery must be durably coordinated with a database change, evaluate an outbox for that specific workflow.

## External APIs and network clients

Follow the concrete-client approach demonstrated by [Zero to Production's EmailClient](https://github.com/LukeMathWalker/zero-to-production/blob/main/src/email_client.rs).

Use a configured `reqwest::Client` inside a client representing an external capability. For example, an `EmailClient` could expose `send_login_code`, while LLM provider adapters expose the supported generation operations. The email provider and its request contract still need to be selected; no Postmark or other delivery integration is implied by this reference.

The concrete client owns:

- The provider endpoint and provider-specific request/response structures.
- Credentials and authentication headers, without exposing them in logs or browser responses.
- Reused HTTP connection pools, connection/request timeouts, and HTTP status handling.
- Translation of provider responses and failures into errors useful to its caller.

Business services call these meaningful methods instead of constructing HTTP requests. Startup creates the configured clients and supplies them to services. Reuse clients rather than constructing a new connection pool for each request.

Do not create a generic networking trait as the starting point. LLM providers can still have separate adapters because their protocols differ; introduce a shared provider interface when the Router actually needs interchangeable implementations.

Retries must be chosen per operation. Do not blindly retry actions that could send duplicate emails, repeat external side effects, or incur additional provider charges. Streaming, cancellation, and fallback behavior require explicit Router rules.

## Testing

- Core's tests live in the separate `compallm-test` crate under `test/tests/`, exercising the public Core API. Run them with `cargo test -p compallm-test`; do not embed new tests in Core or expose private helpers solely for testing. Tests elsewhere in the workspace retain their own existing scope.
- Test domain rules and token validation directly without network or database access.
- Test concrete HTTP clients against a local Wiremock server using the real client implementation. Verify headers and payloads, successful delivery, non-success responses, and timeouts without sending real email or calling paid APIs.
- Use SeaORM `MockDatabase` where useful for query results and service flow. It does not reproduce PostgreSQL constraints, locks, or transaction isolation.
- Use PostgreSQL integration tests for uniqueness, rollback across repositories, and concurrent operations.
- When asked to write but not execute a migration, use compilation and SQL-generation tests only. Application startup and database-backed tests may auto-apply pending migrations.

## Reference boundaries

[Zero to Production](https://github.com/LukeMathWalker/zero-to-production) is a practical reference, not a strict persistence-isolation architecture. It uses validated domain types, a concrete `reqwest` email client, and handlers/helpers that directly use SQLx transactions. Compallm applies those pragmatic ideas with Loco, Axum, and SeaORM and retains its own business decisions.

See [business decisions](decisions.md) for open rules, [product overview](product.md) for capability scope, and the root repository README for local development commands.
