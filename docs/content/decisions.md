---
title: Business decisions
---

# Business decisions

These questions remain open. A feature being listed in the product plan does not resolve its detailed business rules.

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

## Agreed direction

- [Passwordless login](features/passwordless-login.md): organization domain set through an environment variable; emailed codes expire after 15 minutes and can only be used once.
- [LLM Router](features/llm-router.md): supported client API format is independent of provider; translate both directions through a unified internal representation and explicitly reject unsupported capabilities.
- [Router](features/llm-router.md) selects providers and translates formats; [Gateway](features/llm-gateway.md) enforces access, allowances, and usage attribution.

## Router ideas still under review

The 9Router comparison surfaced named routing profiles, credential pools, quota awareness, OAuth refresh, explicit self-hosted policies, compression and prompt presets, client setup helpers, and configuration sync. These remain candidates rather than accepted scope.

## Recording a decision

For each resolved question, record the decision, date, rationale, and affected feature links here or on a dedicated decision page. Update the [roadmap](roadmap.md) when the decision changes release scope.
