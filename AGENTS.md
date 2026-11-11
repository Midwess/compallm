The YAML below is the project’s agent configuration panel: treat its settings as feature toggles and its flows and conventions as instructions to follow.
Check it for every request, detect the user’s intent, follow all applicable flows, and read the linked files. `must`, `never`, `when`, and `except` express requirements, prohibitions, conditions,
and exceptions; `steps` are ordered. Paths are repository-relative. Explicit user instructions take precedence.

```yaml
policy:
  doc-first: false # Implementation need not wait for documentation.
  backward-compatibility: false # Preserve only when explicitly requested.
  issue-required-to-start: false
  preserve: unrelated working-tree changes
  migrations: respect user restrictions; startup and database tests may apply migrations
  before-planning-or-changes: read root README and relevant Wiki architecture and focused pages in full from a freshly fetched external checkout; discover filenames via Home.md and _Sidebar.md; report access failures without inventing rules or using stale content

flows:
  discuss-research:
    steps: inspect → research → explain
    never: implement discussed changes unless requested; keep unresolved business rules explicit
  coding-implement-refactoring:
    steps: inspect → code → verify
    conventions: apply engineering conventions below
    after-code:
      after-code: false
      unit-test: false
    testing:
      scope: core business logic; important main flows and critical edge cases
      unit-tests-per-function: 0–2
      location: tests/ directories only; shared Rust tests in test/tests/; consumer business logic in the consuming application’s tests/
      never: tests under api/ or inline in application source; tests of framework or dependency behavior, strings, logs, print content, error messages, or input validation
      storybook: no unit tests, automated browser or interaction suites, test runners, or libs/storybook/tests; validate with lint, type checks, builds, and manual inspection
    when-behavior-or-architecture-changes: update relevant Wiki documentation and ADR as needed; implementation need not wait for publication
    when-writing-code-comments: use .agents/skills/writing/SKILL.md
  write-review:
    applies-to: code comments, documentation, Wiki, technical prose
    steps: inspect → write → review
    skill: .agents/skills/writing/SKILL.md
    never: implement discussed changes unless requested
    product-docs: Compallm Wiki at https://github.com/Midwess/compallm.wiki.git; never recreate a local docs tree or documentation website
    organization: feature architecture pages own decisions and open questions; focused feature pages own flows and configuration; maintain Home.md and _Sidebar.md when adding or renaming pages
    content: current architecture and direct solution; keep responsibilities, data flow, contracts, constraints, operational guidance, implementation status, and unresolved requirements as relevant
    exclude: visual design, superseded alternatives, dates, review history, implementation diaries, and PR or branch narratives
    adr: under six minutes including diagrams and tables; estimate 200 words per minute plus at least 30 seconds per diagram; aim for 500–800 words or fewer
  create-issue:
    steps: clarify → write → create
    skill: .agents/skills/create-issue/SKILL.md
    when-bug: .agents/skills/create-bug/SKILL.md
    when-feature: .agents/skills/create-feature/SKILL.md
    when-task: .agents/skills/create-task/SKILL.md
  find-link-issue:
    steps: inspect → reuse or link
    skills:
      - .agents/skills/related-issues/SKILL.md
      - .agents/skills/link-issue/SKILL.md
  create-pr:
    steps: review → unit-test -> run linter -> find issue → link → create
    before-creation:
      - use .agents/skills/related-issues/SKILL.md to inspect current PR, branch, and issue links; reuse relevant linked issues
      - write unit-test
      - run linter
    when-no-related-issue: ask user to identify an existing issue or create one with the appropriate issue skill
    new-issues: Midwess/compallm; add to https://github.com/orgs/Midwess/projects/7
    link: use .agents/skills/link-issue/SKILL.md to link issues to the PR
    missing-issue: only PR creation waits; continue authorized preparation and verification
    when-decisions-affect-related-issues: record agreed behavior, scope, acceptance criteria, and published Wiki links or commit; reuse existing notes and preserve others’ content

conventions:
  graphql:
    applies-to: designing, writing, reviewing, or changing schemas, resolvers, or client operations
    rules: |
      Before designing, writing, reviewing, or changing GraphQL schemas, resolvers, or client operations, read the [GraphQL design and execution rules](https://github.com/Midwess/compallm/wiki/GraphQL-guidelines)
      in full from the freshly fetched Wiki checkout (`api/GraphQL-guidelines.md`).
      This is the required project reference for GraphQL naming, lookups, inputs/results, relationships, pagination, operation composition, authorization, batching, caching, limits, and compatibility. Follow its examples except where the flat-root rule below supersedes its nested-root examples pending a Wiki update.
      Check official GraphQL guidance for uncovered cases and keep unresolved business rules explicit.
      Document the solution directly without design-reference comparisons or claims about another product's private implementation.
      Keep project documentation, examples, issue notes, and explanations in Compallm's own terms; do not name the external product used as the GraphQL design reference.

      Publish flat Query and Mutation roots: expose operations directly as root fields, such as role, roles, roleCreate, and groupCreate. Use resource-first mutation names. Keep query, mutation, input, and entity implementations organized in domain folders under api/src/graphql; root fields delegate to those domain functions. Nested output fields represent data and relationships and must not perform writes. The system organization-policy catalog is a system-owned root query, not an organization-owned resource. Preserve verified organization scope and authorization regardless of root shape. Keep one root mutation per request until authorization freshness for multiple writes is explicitly designed and tested; serial execution does not imply a transaction.

      GraphQL read resolvers query the appropriate repository directly instead of adding a service that only forwards reads. Inject the repository through native FromContext projections from the existing DI. Require the appropriate verified identity and enforce Core-owned action and resource checks before returning protected data; derive organization scope from verified authority and retain PostgreSQL row security. Apply field-level authorization to sensitive relationships and fields. Keep database models and persistence conversions in PostgreSQL, and use SeaORM for repository queries. Mutations and reads requiring domain orchestration or invariants continue to use the appropriate Core service. Do not weaken authorization or expose policy source through metadata reads.

      Public API documentation lives in the Wiki. GraphQL definitions live in Rust under `api/src/graphql`. Core authentication uses `libs/core/src/authentication`; GraphQL authentication uses `api/src/graphql/authentication`, separate from authorization. POST /api/graphql exposes Query.authentication.session and the authentication mutation object containing signupEmailRequest/signinEmailRequest, signupEmailConfirm/signinEmailConfirm, sessionRefresh and sessionRevoke. Organization query and mutation objects expose role/group get/list and create operations. Provider management/catalog operations remain pending. Follow the [transport contract](https://github.com/Midwess/compallm/wiki/GraphQL-transport) when extending request handling. Reuse shared Axum middleware under `api/src/http/middleware.rs`; its request contexts lazily parse and verify the required credential and check user-session freshness once per request. Middleware never reads GraphQL documents or maintains resolver-name policies. Each resolver requires its own verified slot before effects; public fields require no credential. The Axum route assembles the GraphQL context from HTTP request state; the current endpoint extracts bounded JSON and enforces one nested business mutation per request, including aliases and fragments, before execution; migrate this validation to the one-root-mutation rule when flattening. Anonymous introspection exposes schema only. Resolver authentication failures use GraphQL errors with safe code/status; strict ordinary HTTP routes may still reject before the handler. Keep credential verification, freshness, and cookie renewal out of GraphQL handlers. Register the GraphQL route and its middleware explicitly in `App::before_routes`; keep the HTTP entry point at `api/src/graphql/endpoint.rs`. The GraphQL module owns schema construction, domain resolvers, and endpoint.rs, with no router or Transport wrapper. Reuse the existing `di::Di` as the sole service container. Construct request-local GraphqlContext from shared DI and the HTTP user, authentication, cookie delivery, and authorization contexts. Apply automatic user-renewal/clearing cookies only if a resolver consumes user authority; explicit cookies override renewal. Project these narrow dependencies with native FromContext; identity accessors reject the wrong credential kind. COOKIE delivery defaults in Rust; explicit null is rejected by Validify. Implement Juniper `FromContext<Di>` service projections and delegate request-context projections to DI. Resolvers request individual services/access through `#[graphql(context)]` parameters; they never receive DI, call global DI, or construct services. Do not add a duplicate Services container. Use #[graphql_object] on impl blocks. Each entity owns query.rs for read functions, mutation.rs for write functions, type.rs for entity structs and field implementations, and input.rs for input DTOs. Shared schema roots expose flat fields that delegate to these domain implementations; keep authentication logic in authentication. GraphQL type names must be unique. Role/group lists currently return arrays. Portal uses typed Genql selections with per-request cancellation and safe GraphQL error handling. Never replay authentication writes automatically. `cargo run -p compallm-api -- generate` exports Juniper SDL and Rust validation JSON Schema, invokes the local Genql CLI against the exported SDL (`--schema schema.graphql --output graphql --sort`), emits shared Zod validators/input guards and inferred form-value aliases, and builds the contract package without starting the app or executing migrations. Input DTOs derive Validify plus ExportValidation from `libs/validation-derive`; custom validators provide sibling metadata functions. Unsupported rules fail export. Controllers author typed object selections checked with `satisfies QueryGenqlSelection` or the corresponding mutation type; generic client inference alone can admit extra properties. Do not add per-operation SDKs or operation strings. Normalize Genql-generated relative imports for the shared ESM package during generation. Genql is the sole generator of API models. Validation export emits runtime rules/constraints and inferred FormInput/FormOutput aliases; do not recreate separate API interfaces or the contract/types barrel. Generation checks all validation metadata against registered SDL definitions and rejects metadata absent from GraphQL. All API types come from Genql; do not add retained TypeScript API snapshots or a preview contract export. Compose UI form helpers from generated validators and constraints. Regenerate tracked artifacts when Rust contracts change; never edit generated files or duplicate rules in UI code. Export only actual input definitions under `api/src/graphql`; do not create a parallel API contract DTO tree. Register reusable auth/provider input and catalog types in the Juniper schema without placeholder operations. Type registration does not expose executable queries/mutations. ContextTokens maps explicitly to TypeScript number and positive-u32 validation metadata. Use generated string session identities and uppercase API enums; local UI drafts may derive lower-case labels and convert them at submission. Never publish the Wiki automatically from startup, builds, or generation.

  architecture:
    style: pragmatic domain-driven design
    core:
      owns: connection lifecycle trait, domain invariants
      never: depend on SeaORM or Postgres; contain database models or connection types
      repository-filters: define ordinary Rust query structs under core/repository with Validify; the shared PostgreSQL executor calls validify before IntoSelect conversion or execution; implement IntoSelect in PostgreSQL to build SeaORM Select values; a derive macro for this conversion is future work
    postgres:
      depends-on: Core
      owns: SeaORM connections, transactions, models, queries, persistence conversions, concurrency guards
      implements: Core connection lifecycle trait
      repository-reads: declare QueryDefinition bindings and use impl_queryable for shared Validify, IntoSelect, scoped SeaORM execution, stable primary-key ordering, and domain conversion; query-specific joins and indexed expressions belong in IntoSelect
    avoid: strict Clean Architecture layers, generic network traits, custom units of work
    except: concrete requirement
    connection-contexts: scoped

  integrations:
    use: concrete clients, reused pools, bounded timeouts
    never: hold database transactions across email or HTTP calls

  database:
    repository-queries:
      must: SeaORM syntax
      never: raw SQL
    generic-created-at-updated-at:
      never: add to migrations or matching models
      except: timestamp required by a concrete business rule
    transaction-macro:
      when: grouping multiple update commands only
      before: perform prerequisite queries outside the transaction
      end: as soon as possible while maintaining correctness

  conversions:
    must: From or TryFrom instead of standalone mappers
    multiple-inputs: tuple source; destructure in try_from
    orphan-rules: local wrapper when tuple conversion for a foreign domain type is prohibited; convert wrapper into domain type
    never: custom conversion trait
    persistence: PostgreSQL crate

  validation:
    source: Rust
    library: Validify for HTTP normalization and shared domain validation
    rules: define in Rust; export to shared contract for Portal and Storybook
    field-errors: share codes, parameters, and messages through the same path when implemented
    detailed-error-message-generation: deferred
    UI: no independent validation messages or error tooltips as part of rule integration

```
