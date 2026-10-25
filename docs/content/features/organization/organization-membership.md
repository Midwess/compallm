---
title: Organization and membership
available: false
---

# Organization and membership

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../../roadmap.md) and [business decisions](../../decisions.md).

## Scope

- Provide an organization workspace for members, provider connections, plugins, notebooks, and usage settings.
- Automatically create a personal organization owned by each user when they complete first-time onboarding.
- Allow users to join and access other organizations according to those organizations' membership policies.
- Manage membership, administrative permissions, and disabled accounts.
- Restrict access to shared resources according to their owner and sharing settings.
- Keep organization data and credentials separated from other organizations.

## Behavior examples

### Personal organization

The system should give each user an organization they own without preventing them from joining other organizations.

#### Example: First-time onboarding
- **WHEN** a user completes authentication and first-time onboarding
- **THEN** the system creates a personal organization owned by that user
- **AND** does not create another personal organization on later logins

### Workspace permissions
The system should enforce configured membership and sharing permissions for organization resources.

#### Example: Unauthorized access
- **WHEN** a user requests a resource outside their granted workspace permissions
- **THEN** the system denies access

## Related pages

- [Data and privacy](../data-privacy.md)
- [Product overview](../../product.md)
- [Business decisions](../../decisions.md)
