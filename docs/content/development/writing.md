---
title: Technical writing
available: false
---

# Technical writing

Agents apply the repository's `technical-writing` skill at `.agents/skills/technical-writing/SKILL.md`. It adapts the purchased handbook's guidance for design reviews, runbooks, technical proposals, and feedback. This page records the project's reading-time convention for human readers.

## Six-minute ADRs

Accepted on 2026-10-26: every feature ADR must take less than six minutes to read, including diagrams and tables. The reader should understand the decision without searching through implementation details.

Aim for 500–800 words or fewer. Estimate prose at 200 words per minute and allow at least 30 seconds per diagram. This is an editing check, not a guarantee of comprehension: unfamiliar concepts and dense tables take longer. The six-minute limit is a ceiling, not a word target.

## What belongs in an ADR

Explain the problem, chosen approach, and why it fits. Show the main flow when that helps understanding. Record consequences, responsibilities, important alternatives or tradeoffs, decision and implementation status, and unresolved questions that could change the solution.

Lead with the decision. Use concrete terms, active sentences, descriptive headings, and short paragraphs. A diagram should explain a relationship or branch that prose alone makes hard to follow. Avoid repeating its steps in a second numbered walkthrough.

Configuration lists, endpoint schemas, code listings, detailed algorithms, and secondary flows belong in linked sub-feature pages. Keep enough context in the ADR to understand the decision independently. Do not move essential rationale out merely to pass a word count, or hide a long design in appendices.

## Editing check

Read the page as an engineer unfamiliar with the conversation. Can they explain the problem, decision, main flow, and most important consequence? If not, rewrite for clarity before adding more text.

Remove repeated facts, generic introductions, unsupported claims, and details that do not change the reader's understanding or next action. Check that links provide depth and that accepted decisions remain distinguishable from proposals and implemented behavior.

## Reference

[Technical Writing for Software Engineers — A Handbook](https://kudmitry.com/books/technical-writing-for-software-engineers/) by Dmitry Kudryavtsev is the source for the skill's writing guidance. The skill was developed from the user's supplied full PDF and paraphrases the chapters on design reviews, runbooks, technical proposals, and feedback. The book allows design-review depth to vary with complexity; Compallm's six-minute ADR ceiling is an additional project rule. The purchased PDF and extracted text are not included in the repository.
