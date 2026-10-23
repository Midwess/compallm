---
title: Usage and administration
tags:
  - features
---

# Usage and administration

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

- Give members a view of their own usage and administrators a view of organization usage.
- Break down usage by member, application, model, provider, and time period.
- Configure model access, usage policies, and limits centrally.
- Notify members and administrators about approaching limits and provider failures.
- Record administrative changes, API key lifecycle events, and plugin actions for auditing.
- Distinguish provider-reported usage and prices from estimates when exact values are unavailable.

## Behavior examples

### Usage visibility
The system should expose members' own usage and organization usage to authorized administrators, distinguishing estimates from provider-reported values.

#### Example: Member usage
- **WHEN** an authenticated member opens their usage history
- **THEN** the system shows that member's usage and identifies estimates

## Related pages

- [Data and privacy](data-privacy.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
