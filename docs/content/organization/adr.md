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

Accepted on 2026-10-31: organizations and memberships omit generic creation/update timestamps. Default membership selection uses the lowest organization ID among eligible active organizations, rather than membership creation time. Signin still requires owner/admin membership. Implementation status: repository selection follows this ordering; membership policy proposals remain open.

Ownership decision accepted on 2026-10-31: `organization_members` entries with role `owner` are the sole ownership record. Organizations have no `created_by` field or separate creator history. Creation saves the organization and its initial owner membership atomically. Removing the creator foreign key also removes its restriction on deleting the original user; membership foreign keys retain their existing cascade behavior. The existing migration and matching models implement this decision; already-migrated databases require separate schema reconciliation.

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
