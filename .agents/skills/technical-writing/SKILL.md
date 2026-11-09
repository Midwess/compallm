---
name: technical-writing
description: Draft, revise, review, and publish Compallm documentation in its GitHub Wiki, including feature ADRs, proposals, runbooks, and public API guides. Use for documentation work; keep feature ADRs below six minutes and product docs outside the application repository.
---

# Technical Writing

Write so someone outside the conversation can understand a decision or complete a task. Choose the document's purpose before its structure. Follow the repository's documentation-first convention in `AGENTS.md`; writing a design does not authorize implementing it.

## Wiki source and publishing

The source of truth is [Midwess/compallm Wiki](https://github.com/Midwess/compallm/wiki), backed by `https://github.com/Midwess/compallm.wiki.git`. Product documentation must be edited there, not in the application repository or the superseded `Midwess/wiki-compallm` repository. Do not recreate a local documentation tree or site. Keep shared engineering instructions in `AGENTS.md` and skill instructions here.

Use an external temporary checkout and prefix shell commands with `rtk`:

```bash
rtk git clone https://github.com/Midwess/compallm.wiki.git /private/tmp/compallm-wiki-edit
```

Choose an unused checkout path for a new clone. When reusing a checkout, inspect its working tree, preserve uncommitted work, and fetch/pull with `--ff-only` before reading. Read `Home.md`, `_Sidebar.md`, and the relevant architecture and focused pages in full. Discover filenames from that checkout rather than guessing capitalization or slugs. A cached checkout alone is not evidence of current published content. If authentication or access fails, report it; do not substitute a stale local copy or invent business rules.

Edit the feature architecture and focused pages directly. Pages are grouped into feature folders in the Wiki checkout, with separate `api/` and `database-schema/` references. Discover pages recursively. Filenames remain globally unique, such as `provider/Provider.md`, `provider/Provider-management.md`, and `portal/Portal-add-provider.md`; public Wiki URLs use the filename without the folder or `.md`. Use full Wiki URLs for cross-folder links so moving files does not break navigation. Preserve existing page names when reorganizing folders.

The root `Home.md` is a short reader-oriented directory; `Feature-guide.md` lists feature architecture pages. Root `_Sidebar.md` and `_Footer.md` provide global navigation. Each feature/reference folder has its own `_Sidebar.md` and `_Footer.md`, used by the pages in that folder. Maintain the relevant local navigation when adding or renaming pages, and keep breadcrumbs consistent with the feature or reference landing page. Do not duplicate the entire Wiki index in every sidebar or child page. Do not add site frontmatter, central ADR folders, duplicated product documents, or visual specifications.

Validate page links and anchors, generated-content ownership, and ADR reading estimates. Inspect the diff, stage only the intended pages, and commit in the Wiki checkout. For an authorized documentation change, push to the Wiki's actual default branch and verify the remote commit; do not ask for redundant confirmation. Never force-push or overwrite concurrent edits. A conflict requires fetching and reconciling the affected pages before publishing. Report published page links and any unresolved publishing failure. For review or discussion only, do not publish changes unless requested. Documentation-first implementation work requires publishing its authorized documentation and recording its agreed decision as described below before application changes when a related issue is established.

## Record the decision in GitHub Issues

Docs first includes a concise final-decision note in each related issue affected by the decision. Reuse issue links from the user or current PR; use [Related Issues](../related-issues/SKILL.md) for read-only discovery when needed. Do not ask again for established issue context or attach the decision to an unconfirmed search match.

After publishing the Wiki change, read the issue's current description and comments. Record the agreed behavior, scope, acceptance criteria, and links to the published Wiki pages or commit. Describe the decision directly, not its discussion history or implementation progress; a published decision does not mean implementation is complete. Note unresolved requirements honestly and resolve them with the user before creating an issue under the Create Issue workflow.

For an existing issue, add a focused comment or update the relevant decision section in its description while preserving others' content. If an equivalent note already exists, reuse it; update only what changed. For a new issue, include the final decision and Wiki links in the initial description rather than adding a duplicate comment. For several issues, tailor each note to its affected scope.

Use `rtk proxy gh issue comment` or `gh issue edit` with an explicit repository and a temporary `--body-file`; read back the saved content to verify it. The authorized docs-first workflow includes this related issue update. It does not authorize unrelated comments, assignments, closure, or board changes. Report failed writes without claiming the decision has been recorded, and check existing state before retrying an ambiguous result.

When an issue is already established, publish its decision note before implementation. If no issue exists yet, publish the Wiki documentation as required, retain its decision summary and links in the current work context, and record them once an issue is created or linked, before PR creation. This preserves the rule that implementation can start without an issue. Report the Wiki and issue-note links, or explicitly state that issue recording is pending.

## Establish the reader and outcome

Identify who will read the document, what they already know, and what they must decide or do afterward. Use the request and existing documents first; ask only when a missing fact changes the outcome. Distinguish known requirements, assumptions, accepted decisions, and unresolved choices. Never invent research results, benchmarks, operational links, or implementation status.

## Designs and ADRs

Apply the handbook's design-review guidance (chapter 1, printed pages 2–6):

- Briefly describe the current behavior and required change in your own words. Link the actual requirement or feature context when available; a link alone does not establish shared understanding.
- Recommend the current solution and explain why it fits the requirements. Keep unresolved constraints explicit; do not retain superseded alternatives, decision dates, review history, or implementation diaries in published ADRs.
- Explain impacts across the system: existing behavior, database changes, API consumers, and dependent teams or services. Include compatibility and migration implications when they matter. Do not copy a database or API design from a book example into the project.
- Support important claims with research or measurements. When evidence is missing, qualify the claim instead of treating intuition as a benchmark.
- Use a diagram for a difficult relationship or branch. Use pseudocode only when an algorithm is central to the decision. Put detailed schema changes, migration instructions, and code in linked sub-feature pages.
- Preserve the reasons and constraints behind the choice so future readers can understand it. Update the document when new evidence changes the design.

For this repository, show decision status separately from implementation status. Keep ADRs understandable without reading every linked detail page.

## Runbooks and procedures

Apply chapter 2 (printed pages 8–11): write for a technically capable operator who does not know the service and may be under time pressure.

- Start with the symptom or trigger, required access, and direct links to the relevant logs, dashboard, and escalation contact when known. Define unfamiliar service names.
- Use ordered actions and explicit branches: what to inspect, how to recognize the result, and which action follows. Use a small condition/action table when it saves searching.
- Provide exact commands with clear placeholders and environment context. Do not copy the handbook's restart commands or infrastructure names as project instructions.
- State the expected result, how to verify recovery, and when to stop or escalate. Mark disruptive actions and required authorization; documenting a command does not grant permission to execute it.
- Avoid burying actions in explanatory paragraphs. If the procedure needs missing observability or tooling, identify that gap rather than pretending the resource exists.

## Proposals for technical change

Apply chapter 3 (printed pages 12–16):

- Start with demonstrated pain for users, maintainers, or the organization. Preference for a fashionable tool is not a reason to migrate.
- Compare the status quo and credible alternatives using relevant criteria: reliability, cost, maintenance, adoption, local development, and required capabilities. Select criteria fairly; do not cherry-pick advantages.
- Use a compact comparison table when it clarifies the choice. Briefly explain early exclusions without building an exhaustive catalog.
- Recommend an approach and include a feasible transition: affected consumers, dependencies, effort, compatibility, and how useful work continues during migration. Consider incremental adoption when appropriate; do not promise effort or schedules without evidence.

## Public guides and API references

Apply the same reader-first principles to consumers rather than internal reviewers. Explain prerequisites, inputs, outputs, failure recovery, and a realistic example. Keep internal architecture discussions out of the caller's task.

Rust owns the GraphQL schema and validation rules. Follow the [application API](https://github.com/Midwess/compallm/wiki/API-reference) and [GraphQL guidelines](https://github.com/Midwess/compallm/wiki/GraphQL-guidelines). Keep operation descriptions sourced from Rust and preserve them in generated TypeScript JSDoc. Frontend operation selections are authored inputs, not a second schema.

The single Cargo generation command is planned after transport cleanup. Do not invoke a generator that is not implemented or restore obsolete endpoint-generation crates and derives. Interim offline form types and validators are not active endpoint contracts. Public guides describe actually exposed operations; schema examples remain explicitly illustrative until runtime integration lands.

Publish API documentation in the separate Wiki and maintain page/navigation links together. Preserve generated-file ownership when a replacement generator exists. Neither startup, builds, nor ordinary generation may publish the Wiki automatically. Documentation generation and verification must not start the application or execute migrations.

## Edit for understanding

Remove repetition, unexplained terminology, irrelevant history, and claims that do not support the reader's decision or next action. Use paragraphs for reasoning, lists for actions, and tables for comparisons. Do not repeat an entire flowchart as a numbered walkthrough.

Read the result without relying on the conversation: can the intended reader explain the requirement, recommendation, main consequence, and next action? Check commands, links, examples, and status claims. Treat specific review feedback as evidence for revision (handbook bonus chapter, printed page 17).

## Repository conventions

These are project rules, not claims from the handbook:

- Put feature decisions in the feature's Wiki architecture page and focused details in named sub-feature pages. Public operation references belong in the Wiki under the application API; keep internal generation instructions separate from consumer operation guides.
- Keep every feature ADR below six minutes, including tables and diagrams. Aim for 500–800 words or fewer; estimate prose at 200 words per minute plus at least 30 seconds per diagram, and allow more for dense material. The estimate is an editing check, not proof of comprehension.
- Keep the requirement, decision, rationale, main flow, consequences, status, and important open questions in the ADR. Cut repetition or link secondary detail; do not hide essential reasoning or append a second long design to pass the limit.
- Scale detail to the change. Do not force every document into every section above or require an approval ceremony for routine edits.

## Source

Adapted in original wording from the user's supplied copy of Dmitry Kudryavtsev's *Technical Writing for Software Engineers: A Handbook* (2023), including all three chapters and the feedback section. Page references use the book's printed numbering, not PDF sheet numbers. [Author's book page](https://kudmitry.com/books/technical-writing-for-software-engineers/).

The handbook allows design-review length to vary with complexity; Compallm adds the six-minute ADR ceiling and linked detail pages. This skill summarizes practices, not the full text. It does not require access to the purchased PDF, and the PDF and extracted text are not repository assets.
