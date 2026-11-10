---
name: writing
description: Draft, revise, review, and publish Compallm documentation, issue descriptions, and other project prose; apply concise, natural writing and Wiki publishing rules.
---

# Writing

## Write and review

Read and apply [Humanizer](references/humanizer.md) when drafting, revising, or reviewing prose. Follow project rules and the user’s instructions if guidance conflicts.

- Identify the reader and outcome; scale detail to the task. Explain decisions, constraints, consequences, and unresolved questions without inventing facts, evidence, or implementation status.
- Use direct, concrete language. Remove repetition, filler, inflated claims, stock AI phrases, forced contrasts, decorative formatting, and chatbot openings or closings. Use lists for actions and tables for comparisons; avoid repeating diagrams in prose.
- Preserve the author's voice, meaning, uncertainty, attribution, and requirement strength. Preserve code, commands, identifiers, metadata, data, and link targets during prose edits. Read existing files in full before editing.
- For designs, include the requirement, solution, rationale, impact, status, and open questions. For runbooks, include triggers, prerequisites, ordered actions, expected results, verification, and escalation. For proposals, compare credible options and describe the transition. For public guides, cover prerequisites, inputs, outputs, errors, and examples.
- Check claims, commands, examples, links, and whether the document makes sense without the conversation. For feature ADRs, keep the complete reading estimate under six minutes: 200 words/minute plus at least 30 seconds per diagram; aim for 500–800 words or fewer.

## Wiki

- Follow `AGENTS.md` for documentation requirements, scope, and product documentation boundaries. A documentation request does not authorize implementation.
- Source: [Compallm Wiki](https://github.com/Midwess/compallm/wiki), `https://github.com/Midwess/compallm.wiki.git`. Use a fresh external checkout; when reusing one, preserve local changes and fetch/pull with `--ff-only`. Discover pages through `Home.md` and `_Sidebar.md`, and read relevant architecture and focused pages in full. Report access failures; do not use stale content as current or invent requirements.
- Maintain relevant navigation, breadcrumbs, and links when pages move or change. Use full Wiki URLs for cross-folder links; preserve globally unique page filenames. Keep generated API references and their source ownership intact; never publish during startup, builds, or ordinary generation.
- For authorized publishing, validate links, anchors, generated content, and ADR length; inspect the diff, stage only intended files, commit and push to the Wiki's default branch, and verify the remote commit. Never force-push or overwrite concurrent work; reconcile conflicts. Report published links or failures. Discussion or review alone does not authorize publishing.

## Issue decisions

- For related decision records, reuse established related issues; use [Related Issues](../related-issues/SKILL.md) when discovery is needed. Do not attach a decision to an unconfirmed match.
- After publishing, inspect each affected issue and record the agreed behavior, scope, acceptance criteria, unresolved requirements, and Wiki links. Preserve others' content and reuse equivalent notes. For new issues include the decision in the description; for existing ones use a focused comment or section update.
- Use an explicit repository and temporary `--body-file`; read back the result. Check current state before retrying an ambiguous write. Report failures. Do not change unrelated issues, assignments, or status.
- Record relevant decisions in established issues before PR creation. If no issue exists, retain the summary and links until one is created or linked.
