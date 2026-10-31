---
title: Usage and administration
available: false
---

# Usage and administration

## Status

Implementation: Under development.

The following solution is planned; unresolved requirements are listed below.

## Solution

- Give members a view of their own usage and administrators a view of organization usage.
- Break down usage by member, application, model, provider, and time period.
- Configure model access, usage policies, and limits centrally.
- Notify members and administrators about approaching limits and provider failures.
- Record administrative changes, API key lifecycle events, and plugin actions for auditing.
- Distinguish provider-reported usage and prices from estimates when exact values are unavailable.

## Consequences

Members and administrators see different usage scopes. Reports must distinguish provider measurements from estimates and follow Gateway accounting rules.

## Behavior examples

### Usage visibility
The system should expose members' own usage and organization usage to authorized administrators, distinguishing estimates from provider-reported values.

#### Example: Member usage
- **WHEN** an authenticated member opens their usage history
- **THEN** the system shows that member's usage and identifies estimates

## Open questions

Which reports, notifications, and audit events ship first? Access rules depend on [organization roles](../organization/adr.md), and accounting follows the [Gateway's allowance decisions](../llm-gateway/adr.md).

## Related pages

- [Product overview](../overview.md)
