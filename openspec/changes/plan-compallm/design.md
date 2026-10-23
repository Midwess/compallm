# Planning approach

## Context

Compallm is an organization-oriented LLM workspace. The repository contains application infrastructure; proposed product features are not represented as completed work.

## Decisions

- Use official OpenSpec layout: baseline requirements in `openspec/specs/`, proposals in `openspec/changes/`, and completed changes in `openspec/changes/archive/`.
- Keep this proposal open while discussing business rules and release scope. Completed Markdown artifacts do not mean implemented features.
- Preserve existing feature scope in separate capability files. Initial scenarios are a starting point, not exhaustive acceptance coverage.
- Keep Router responsibilities (provider selection and translation) separate from Gateway responsibilities (authorization, limits, and usage attribution).
- Translate through a unified internal format while retaining the client's supported protocol at the external boundary.
- Candidate additions from the 9Router comparison require explicit scope decisions: routing profiles, credential pools, quota awareness, OAuth refresh, self-hosted policies, optimization modes, client helpers, and configuration sync.

## Business decisions to resolve

1. Is the first deployment one organization configured through environment variables, or a platform hosting multiple organizations?
2. Does verified ownership of an allowed email automatically create membership, or is invitation or administrator approval required?
3. Which roles are needed, and what can each role administer, share, and access?
4. Are provider credentials organization-owned only, or may members bring personal credentials?
5. Are allowances based on tokens, money, requests, or a combination, and what happens when a limit is reached?
6. How are simultaneous requests, fallback attempts, cancellations, and failed requests charged against limits?
7. Which API compatibility standard, providers, and model capabilities are required for the first release?
8. Are chats and notebooks private by default, and who may share or inspect them?
9. Which plugin types are supported, and which external actions require confirmation?
10. What are the retention and deletion rules, including files sent to external providers?
11. Which capabilities belong in the first release, and which are later product scope?

## Implementation readiness

Select first-release capabilities, resolve their blocking decisions, and create focused implementation changes with concrete scenarios, migrations, APIs, frontend, and tests before applying the plan.
