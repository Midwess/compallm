---
title: "Development: ADR"
---

# Development: ADR

Decision status: Accepted engineering direction. Implementation status: Apply to new work; individual feature ADRs distinguish plans from implemented behavior.

## Context and decision

Use pragmatic domain-driven design: organize around business capabilities, keep rules near their types, and introduce abstractions for concrete needs. Modules and focused services come before strict Clean Architecture layers or a trait around every library.

Axum serves the frontend and API on one origin. It proxies Vite during development, including hot reload, and serves built files in production. Vite binds strictly to internal port `41868`; the API starts its listener search at `3000` by default. If Vite's port is occupied, startup fails rather than choosing another port.

The API uses Hero's listener discovery in both modes: start at the configured port, keep the first available socket reserved, and log the actual URL. Hero listens on all IPv4/IPv6 interfaces. Browser API requests use relative URLs on the selected Axum origin; Vite needs no API proxy. Axum keeps the `/api` namespace, including JSON 404 responses, while frontend requests reach Vite in development. Deployments must route to the selected port.

## Responsibilities

| Location | Responsibility |
| --- | --- |
| `api/` | Loco/Axum, HTTP contracts, cookies, configuration, and dependency wiring |
| `libs/core/` | Domain types, business rules, services, and persistence contracts |
| `libs/postgres/` | SeaORM connections, transactions, schemas, repositories, mappings, and migrations |
| `libs/utils/` | Stateless, general-purpose helpers |
| `portal/` | React and Vite frontend |
| `docs/content/<feature>/` | Feature ADR and focused flows |
| `docs/web/` | Quartz publication |

Core owns domain entities and the database lifecycle contract without depending on SeaORM. PostgreSQL depends on Core and implements that contract. Keep storage models and queries in PostgreSQL and map them to domain entities. Third-party libraries are welcome where useful; framework-specific HTTP types stay in the API application.

A domain module is not automatically a crate or bounded context. Split it when responsibilities or dependencies justify the boundary. Each token owns its claims and validation; `AuthService` coordinates authentication.

## Request validation

Use Validify on HTTP request DTOs and axum-valid extraction to normalize fields before validating them. Keep Serde for JSON shape and Utoipa for documented constraints. Email input is trimmed and ASCII-lowercased, then checked with the shared email policy. Never normalize signed token contents. Return safe 400 errors for malformed fields without echoing submitted credentials.

Core remains callable outside HTTP: keep domain invariants, cryptographic checks, expiry, and authorization there. Domain types own reusable field rules and Validify-based preparation; services coordinate those operations instead of repeating field checks. Utils contains formatting helpers, not user or organization validation policy. Repositories accept prepared domain values without normalizing or validating their fields again. Keep fallible storage-type conversions, uniqueness handling, and conditional updates; positive-ID checks already enforced by database constraints are not repeated in adapters. Do not make Core trust a controller-only DTO.

## Persistence and transactions

Application entry points establish `DbContext::scope`. Services and repositories use that context without routine scope wrappers. Services choose transaction boundaries; nested transactions join the active transaction. Multiple repositories in one workflow share it, and only the outermost boundary commits.

Repositories must not switch connections or independently commit a larger operation. Rollback covers failures, but uniqueness constraints and conflict handling must also address concurrent requests.

Do not hold a transaction open during slow email or HTTP calls. Database transactions do not cover external delivery; consider an outbox only when durable coordination is required. Avoid an additional unit-of-work layer without a concrete need.

Generic `created_at` and `updated_at` columns are omitted from every application table (accepted 2026-10-31). Keep timestamps only for explicit business events or validity, such as email verification, revocation, and token expiry. Creation/update history is unavailable. Existing migration definitions and matching models are updated directly; already-migrated databases need separate schema reconciliation before using the revised schema.

## External integrations

Follow the concrete-client approach in [Zero to Production's EmailClient](https://github.com/LukeMathWalker/zero-to-production/blob/main/src/email_client.rs). Startup supplies configured clients with reused connection pools and bounded timeouts.

Each client owns provider URLs, credentials, wire formats, status handling, and useful error translation. Business services call meaningful methods rather than assembling HTTP requests. Never expose credentials in logs or browser responses.

Retries are operation-specific: repeating a request may resend email, repeat a side effect, or incur charges. Router streaming, cancellation, and fallback rules must be explicit. Introduce interchangeable-provider traits when the Router needs them; avoid a generic network abstraction by default.

## Verification

GitHub PR diffs collapse test directories, conventional test filenames, and documentation by default using `.gitattributes` and `linguist-generated=true`. Reviewers can expand those files; this display setting does not exclude tests from execution or files from the AI explainer's code reads. Documentation website source code stays visible unless it is itself a test or documentation file.

PR explanations use the PR diff and code at pinned commits as their default context. They must not require any project-specific documentation path; additional repository files can be read on demand through the existing tools. This lets documentation move or disappear without breaking the automation.

Keep tests in `tests/` directories, using the existing `test/tests/` suite for shared Rust tests. Do not put tests under `api/` or inline in application source. Focus coverage on core business logic without external services; dependency and framework behavior do not need their own tests. This keeps verification focused on application-owned rules and reduces maintenance of tests for third-party behavior.

Compilation or SQL-generation checks are appropriate when migrations must not execute. Application startup and database-backed tests may migrate automatically, so respect explicit restrictions.

## Documentation before implementation

Complete the relevant design before changing behavior. Every feature ADR must take **less than six minutes** to read, including diagrams and tables. Aim for 500–800 words or fewer; estimate prose at 200 words per minute plus at least 30 seconds per diagram.

Keep the problem, decision, rationale, main flow, consequences, status, and important open questions. Cut repetition and put configuration, contracts, and detailed instructions in linked sub-feature pages. The limit measures reader effort; it does not justify omitting essential decisions. See [Technical writing](writing.md) for the editing standard and reference handbook.

## Portal design workspace

Decision status: Accepted on 2026-10-27; foundation-first reset accepted on 2026-10-31. Implementation status: Storybook starts empty. Shared Tailwind colors and spacing match Penpot; Phosphor SVG dependencies are installed. Base component translation and Storybook previews are authorized; feature integration remains pending.

Use Storybook in `storybook/` to review real React components and screen prototypes before connecting them to application behavior. Components belong in `portal/` and are imported directly by stories so the production application can reuse the reviewed code. Stories use sample data and must not require Rust, PostgreSQL, authentication, or generated API clients to start.

Keep prompting in Codex CLI. Storybook's local MCP addon exposes component and story context; source edits and hot reload drive iteration. Penpot's Design foundations page owns the initial dark neutral/violet palette, spacing, and Phosphor SVG library. Implement requested base components in `portal/src/components/ui/`, grouped by foundation family, and preview them under Storybook’s **Components** folder. Translate the accepted constants into one shared Tailwind v4 CSS theme used by Storybook and available to the portal. Translation is explicit, without automatic synchronization. Do not add local Penpot services. Keep token-bearing MCP configuration local and untracked.

Use the ignored `.dev/penpot/` JavaScript workbench for local Penpot automation and agent notes. Scripts run directly with Node, reuse cached design IDs and a serialized MCP connection, and keep credentials in the existing local MCP configuration. Its loopback bridge is a client-side convenience, not a local Penpot deployment. For routine canvas edits, batch the change, check the affected frame, and save once; refresh the catalog or generate extra review artifacts only when needed.

Review wireframes before styling complete screens. Dark mode uses the accepted foundation palette; Inter remains the shared UI font. Phosphor Regular replaces Pixelarticons and mixed UI icon sets. Navigation and screen composition remain design proposals. Prototype screens do not establish business rules or authorize API integration. See [Design workspace](design-workspace.md) for commands and the review workflow.

Penpot's Design foundations page owns shared visual rules, tokens, and base components only: fields, buttons, dropdown controls, selection controls, feedback primitives, typography, and small content/navigation units. Full screens, complete sidebars, forms, feature panels, and complex assemblies belong on dedicated feature pages. Do not create a Page patterns gallery or define whole pages as shared foundation components. Feature compositions reuse the bases; their internal assemblies stay with the feature. This accepted organization supersedes the earlier complete-sidebar and form/page-pattern galleries on Foundations.

Use token-bound Auto Layout for spacing. Verify component links after transfers and restore temporary propagation-test values before review. Use Inter as the shared Penpot UI font. Sidebar Label is 13 pixels with a 14-pixel icon; root category headings are 10 pixels and the workspace name is 12 pixels. Penpot changes do not automatically update React code. Keep reusable Phosphor SVG masters in Design foundations; do not recreate the superseded purchased-icon catalogs.

The [shared spacing policy](../portal/spacing.md) controls library and screen gaps, padding, and margins through one scale and minimum. Current density is half the preceding values, with a 2 px floor for nonzero spacing and zero preserved. Components inherit constants from their masters; geometric control sizes, typography, borders, and radii remain independent. Agents can edit the policy through Penpot MCP, refresh token resolution, and verify the resulting layouts.

## Related decisions

- [API reference](../api/index.md): Rust/OpenAPI generates TypeScript hover documentation and the public endpoint guide.
- [Login](../login/adr.md): identity verification, organization scope, cookies, and refresh sessions.
- [Mailing](../mailing/adr.md): bounded SMTP delivery, worker concurrency, and recipient throttling.
- [Product overview](../overview.md): feature plans and their own decisions.
