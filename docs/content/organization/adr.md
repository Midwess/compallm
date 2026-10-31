---
title: Organization and membership
available: false
---

# Organization and membership

## Status

Implementation: Under development.

The following solution is planned; unresolved requirements are listed below.

## Solution

- Provide an organization workspace for members, provider connections, plugins, notebooks, and usage settings.
- Automatically create a personal organization owned by each user when they complete first-time onboarding.
- Allow users to join and access other organizations according to those organizations' membership policies.
- Manage membership, administrative permissions, and disabled accounts.
- Restrict access to shared resources according to their owner and sharing settings.
- Keep organization data and credentials separated from other organizations.

## Organization records

Support multiple organizations. First-deployment bootstrap provisions an initial organization from environment configuration; the configuration fields remain unresolved. Signup creates a personal organization and owner membership atomically.

Organizations and memberships omit generic creation/update timestamps. Default membership selection uses the lowest organization ID among eligible active organizations. Signin requires owner/admin membership.

`organization_members` entries with role `owner` are the sole ownership record. Organizations have no separate creator field or creator history. Membership foreign keys cascade on user or organization deletion.

## Open questions

See [membership administration](members.md) for operation boundaries and unresolved policies.

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

- [Product overview](../overview.md)
