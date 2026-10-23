---
title: Organization and membership
tags:
  - features
---

# Organization and membership

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

- Provide an organization workspace for members, provider connections, plugins, notebooks, and usage settings.
- Manage membership, administrative permissions, and disabled accounts.
- Restrict access to shared resources according to their owner and sharing settings.
- Keep organization data and credentials separated from other organizations if multiple organizations are supported.

## Behavior examples

### Workspace permissions
The system should enforce configured membership and sharing permissions for organization resources.

#### Example: Unauthorized access
- **WHEN** a user requests a resource outside their granted workspace permissions
- **THEN** the system denies access

## Related pages

- [Data and privacy](data-privacy.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
