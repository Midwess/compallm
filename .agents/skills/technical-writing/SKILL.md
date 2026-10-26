---
name: technical-writing
description: Draft, revise, or review technical documentation, including feature ADRs, design proposals, runbooks, and public API guides. Match the reader and purpose, justify decisions with evidence, and keep feature ADRs below six minutes.
---

# Technical Writing

Write so someone outside the conversation can understand a decision or complete a task. Choose the document's purpose before its structure. Follow the repository's documentation-first convention in `AGENTS.md`; writing a design does not authorize implementing it.

## Establish the reader and outcome

Identify who will read the document, what they already know, and what they must decide or do afterward. Use the request and existing documents first; ask only when a missing fact changes the outcome. Distinguish known requirements, assumptions, accepted decisions, and unresolved choices. Never invent research results, benchmarks, operational links, or implementation status.

## Designs and ADRs

Apply the handbook's design-review guidance (chapter 1, printed pages 2–6):

- Briefly describe the current behavior and required change in your own words. Link the actual requirement or feature context when available; a link alone does not establish shared understanding.
- Recommend a solution and explain why it fits the requirements. Record relevant alternatives and why they were rejected. Avoid ending with an unexplained menu for the reader to choose from; leave genuinely unresolved constraints explicit.
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

For generated API pages and TypeScript hover documentation, edit Rust comments and Utoipa/OpenAPI metadata, then run the repository's generation command. Do not hand-edit generated pages or the generated client. Document actual behavior separately from planned ADR behavior.

## Edit for understanding

Remove repetition, unexplained terminology, irrelevant history, and claims that do not support the reader's decision or next action. Use paragraphs for reasoning, lists for actions, and tables for comparisons. Do not repeat an entire flowchart as a numbered walkthrough.

Read the result without relying on the conversation: can the intended reader explain the requirement, recommendation, main consequence, and next action? Check commands, links, examples, and status claims. Treat specific review feedback as evidence for revision (handbook bonus chapter, printed page 17).

## Repository conventions

These are project rules, not claims from the handbook:

- Put feature decisions in `docs/content/<feature>/adr.md` and focused details in sibling pages. Public generated references live under `docs/content/api/`, following endpoint paths and methods.
- Keep every feature ADR below six minutes, including tables and diagrams. Aim for 500–800 words or fewer; estimate prose at 200 words per minute plus at least 30 seconds per diagram, and allow more for dense material. The estimate is an editing check, not proof of comprehension.
- Keep the requirement, decision, rationale, main flow, consequences, status, and important open questions in the ADR. Cut repetition or link secondary detail; do not hide essential reasoning or append a second long design to pass the limit.
- Scale detail to the change. Do not force every document into every section above or require an approval ceremony for routine edits.

## Source

Adapted in original wording from the user's supplied copy of Dmitry Kudryavtsev's *Technical Writing for Software Engineers: A Handbook* (2023), including all three chapters and the feedback section. Page references use the book's printed numbering, not PDF sheet numbers. [Author's book page](https://kudmitry.com/books/technical-writing-for-software-engineers/).

The handbook allows design-review length to vary with complexity; Compallm adds the six-minute ADR ceiling and linked detail pages. This skill summarizes practices, not the full text. It does not require access to the purchased PDF, and the PDF and extracted text are not repository assets.
