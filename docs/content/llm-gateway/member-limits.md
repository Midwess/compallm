---
title: "Usage and spend limits"
available: false
---

# Usage and spend limits

Usage limits are organization-scoped. Workspace limits, per-member defaults, and individual overrides are distinct configuration values. Monthly member budgets are denominated in USD. Persistence, accounting, authorization, and enforcement remain planned.

An individual member can inherit the organization default or have an explicit monthly budget. Unset configuration differs from zero. Reports attribute previous-period spend, current-period spend, configured limits, and utilization to the member. Missing measurements must not be fabricated.

Resolve the reset schedule and timezone, workspace/default/override precedence, included costs, fallback-attempt accounting, concurrent reservations, cancellations, failed requests, over-limit behavior, and role permissions before implementation. Sample values do not define runtime policy.

The [Gateway](adr.md) owns enforcement and attribution. The [Router](../llm-router/adr.md) owns upstream selection and protocol translation.
