---
title: Notebook
tags:
  - features
---

# Notebook

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

Build a reusable knowledge collection for focused work, research, and study.

- Create notebooks around a subject, project, or learning goal.
- Add documents, notes, and supported source links.
- Track source processing status, updates, and removals.
- Ask questions using selected notebook sources and show citations back to the supporting material.
- Distinguish source-backed answers from general model knowledge.
- Generate summaries, explanations, study guides, and practice questions.
- Save useful answers as notes and reuse notebook knowledge in chat.
- Share notebooks with authorized organization members.

## Behavior examples

### Source-grounded answers
The system should let authorized members organize sources into notebooks and identify supporting sources for source-backed answers.

#### Example: Source citation
- **WHEN** a member asks a question supported by selected notebook sources
- **THEN** the answer includes citations to those sources

## Related pages

- [LLM Chat](llm-chat.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
