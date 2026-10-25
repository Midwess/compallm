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
| `libs/core/` (planned) | Shared business types, domain services, token behavior, and focused external-service clients |
| `libs/postgres/` | SeaORM entities, repository queries, connection access, and migrations |
| `portal/` | React frontend built with Vite |
| `docs/content/` | Human-readable product plans, decisions, and architecture |
| `docs/web/` | Quartz documentation website |

Axum is the public entry point for the frontend and API. In development it proxies Vite and hot reload; in production it serves Vite's built static files.

## Organize by business domain

Authentication, users, organizations, routing, and gateway usage are business capabilities. Group their types and behavior together rather than creating project-wide folders for every technical layer.

For example, the planned authentication domain would start with:

```text
libs/core/src/auth/
  service.rs
  token.rs
  token/
    email_otp.rs
    user.rs
    refresh.rs
```

`AuthService` coordinates authentication workflows. Each token struct owns its claims, invariants, and validation behavior. `AccessToken` wraps `EmailOtpAccessToken`, `UserAccessToken`, and `RefreshToken` in its `EmailOtp`, `User`, and `Refresh` variants.

A domain module is not automatically a separate crate or bounded context. Split it when its business responsibilities or dependency needs justify that boundary.

## Dependencies and third-party libraries

Core is not required to be dependency-free. Use established libraries for serialization, validation, cryptography, error handling, HTTP communication, and database execution when they support the domain's work.

- Keep framework-specific requests, responses, cookies, and HTTP status mapping in `api/`.
- Keep query and storage implementation details in `libs/postgres/`.
- Do not use SeaORM entities as API responses automatically; define the external contract deliberately.
- Keep provider URLs, authentication headers, and wire formats inside concrete integration clients.
- Avoid circular crate dependencies. With the pragmatic approach, `api` may depend on Core and Postgres, and Core may depend on Postgres for application services. Postgres must not depend back on Core in that arrangement; map persistence results into business types at the Core boundary.

There is no requirement for a trait around every library or service. Do not introduce repository interfaces, generic HTTP transport traits, or unit-of-work layers merely to imitate Clean Architecture. Add a trait when a concrete need appears, such as interchangeable implementations or a useful testing boundary.

## Persistence and transactions

Use concrete SeaORM repositories. Repository operations that can participate in a transaction should accept a connection implementing SeaORM's `ConnectionTrait`; both `DatabaseConnection` and `DatabaseTransaction` implement it.

The application service decides the transaction boundary. When a workflow spans multiple repositories, start one transaction and pass that same transaction to every repository operation. Commit only after all required operations succeed.

For example, completing authentication may need user lookup/creation, session creation, and refresh-token persistence in the same transaction. These are planned operations, not claims that session storage already exists.

- A repository must not silently switch from the supplied transaction to a separate connection.
- Repositories participating in a larger operation must not independently commit it.
- Let transaction rollback cover errors, and handle uniqueness conflicts and concurrent requests explicitly. Atomicity alone does not prevent duplicate-signup races.
- SeaORM may appear in Core service signatures. Persistence independence is not a requirement of this architecture.
- A custom unit of work is not the default. Reconsider it only if we later need a persistence-independent Core or a concrete alternative storage implementation.

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

- Test domain rules and token validation directly without network or database access.
- Test concrete HTTP clients against a local Wiremock server using the real client implementation. Verify headers and payloads, successful delivery, non-success responses, and timeouts without sending real email or calling paid APIs.
- Use SeaORM `MockDatabase` where useful for query results and service flow. It does not reproduce PostgreSQL constraints, locks, or transaction isolation.
- Use PostgreSQL integration tests for uniqueness, rollback across repositories, and concurrent operations.
- When asked to write but not execute a migration, use compilation and SQL-generation tests only. Application startup and database-backed tests may auto-apply pending migrations.

## Reference boundaries

[Zero to Production](https://github.com/LukeMathWalker/zero-to-production) is a practical reference, not a strict persistence-isolation architecture. It uses validated domain types, a concrete `reqwest` email client, and handlers/helpers that directly use SQLx transactions. Compallm applies those pragmatic ideas with Loco, Axum, and SeaORM and retains its own business decisions.

See [business decisions](decisions.md) for open rules, [product overview](product.md) for capability scope, and the root repository README for local development commands.
