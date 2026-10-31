---
title: Notebook
available: false
---

# Notebook

## Status

Implementation: Under development.

The following solution is planned; unresolved requirements are listed below.

## Solution

Build a reusable knowledge collection for focused work, research, and study.

- Create notebooks around a subject, project, or learning goal.
- Add documents, notes, and supported source links.
- Track source processing status, updates, and removals.
- Ask questions using selected notebook sources and show citations back to the supporting material.
- Distinguish source-backed answers from general model knowledge.
- Generate summaries, explanations, study guides, and practice questions.
- Save useful answers as notes and reuse notebook knowledge in chat.
- Share notebooks with authorized organization members.

## Consequences

Answers must distinguish source-backed content from general model knowledge. Source retrieval and sharing remain subject to the notebook's access rules.

## Behavior examples

### Source-grounded answers
The system should let authorized members organize sources into notebooks and identify supporting sources for source-backed answers.

#### Example: Source citation
- **WHEN** a member asks a question supported by selected notebook sources
- **THEN** the answer includes citations to those sources

## Open questions

Which source types and notebook capabilities ship first, and how do the [privacy and sharing rules](../data-privacy/adr.md) apply to notebooks?

## Related pages

- [LLM Chat](../llm-chat/adr.md)
- [Product overview](../overview.md)
