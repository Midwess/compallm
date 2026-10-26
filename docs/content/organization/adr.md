---
title: Organization and membership
available: false
---

# Organization and membership

## Status

Decision status: Multiple organizations and personal-organization creation are accepted; additional membership policies remain proposed. Implementation status: Under development.

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Open questions are recorded below.

## Context and proposed scope

- Provide an organization workspace for members, provider connections, plugins, notebooks, and usage settings.
- Automatically create a personal organization owned by each user when they complete first-time onboarding.
- Allow users to join and access other organizations according to those organizations' membership policies.
- Manage membership, administrative permissions, and disabled accounts.
- Restrict access to shared resources according to their owner and sharing settings.
- Keep organization data and credentials separated from other organizations.

## Agreed decisions

Accepted on 2026-10-25; implementation is tracked separately from these product decisions.

- Support multiple organizations. The first deployment provisions one initial organization from environment variables; this bootstrap does not limit the platform to one organization.
- Create a personal organization owned by each user during first-time onboarding. Users may also join other organizations under their membership policies.

## Open questions

- Does joining another organization require an invitation, administrator approval, or another membership rule?
- Which roles are needed, and what may each role administer, share, and access?
- Do GitHub and GitLab identities follow exactly the same membership rules as email-code identities?
- What initial-organization fields are configured, and which capabilities belong in the first release?

## Consequences

First-deployment bootstrap does not limit the platform to one organization. Personal ownership does not grant access to other organizations; their membership policies still apply.

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

- [Data and privacy](../data-privacy/adr.md)
- [Product overview](../overview.md)
