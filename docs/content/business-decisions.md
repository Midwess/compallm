---
title: Business decisions
order: 2
available: true
---

# Business decisions

These questions remain open. A feature being listed in the product plan does not resolve its detailed business rules.

1. Does verified ownership of an allowed email automatically create membership, or is invitation or administrator approval required?
2. Which roles are needed, and what can each role administer, share, and access?
3. Are provider credentials organization-owned only, or may members bring personal credentials?
4. Are allowances based on tokens, money, requests, or a combination, and what happens when a limit is reached?
5. How are simultaneous requests, fallback attempts, cancellations, and failed requests charged against limits?
6. Which API compatibility standard, providers, and model capabilities are required for the first release?
7. Are chats and notebooks private by default, and who may share or inspect them?
8. Which plugin types are supported, and which external actions require confirmation?
9. What are the retention and deletion rules, including files sent to external providers?
10. Which capabilities belong in the first release, and which are later product scope?
11. May a GitHub or GitLab account whose email address is outside the allowed organization domain sign in, and does an OAuth login satisfy the same membership rules as an email-code login?

## Agreed direction

- [Organization and membership](features/organization/organization-membership.md) (2026-10-25): the platform supports multiple organizations. The first deployment provisions one initial organization from environment variables; this bootstrap mechanism does not restrict the platform to a single organization.
- [Organization and membership](features/organization/organization-membership.md) (2026-10-25): each user receives a personal organization that they own during first-time onboarding. A user may also join other organizations according to their membership policies.
- [LLM Router](features/llm-router.md): supported client API format is independent of provider; translate both directions through a unified internal representation and explicitly reject unsupported capabilities.
- [Router](features/llm-router.md) selects providers and translates formats; [Gateway](features/llm-gateway.md) enforces access, allowances, and usage attribution.

## Router ideas still under review

The 9Router comparison surfaced named routing profiles, credential pools, quota awareness, OAuth refresh, explicit self-hosted policies, compression and prompt presets, client setup helpers, and configuration sync. These remain candidates rather than accepted scope.

## Recording a decision

For each resolved question, record the decision, date, rationale, and affected feature links here or on a dedicated decision page. Update the [roadmap](roadmap.md) when the decision changes release scope.
