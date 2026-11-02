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

## Workspace page

The UI library exports `OrganizationPage` as the organization workspace shell. Consumers provide `OrganizationControllerContext` from the library’s public context module. The controller owns workspace data, selected destinations, and navigation intent. The page composes `MainPage` and sidebar components; child content is supplied by the consumer.

The organization shell has no default content. The portal owns the soft redirect from the organization entry to `/:organizationName/chat`; Storybook does not redirect. New-conversation and conversation-selection actions notify `onOpenChat(conversationId?)`. The portal owns navigation to `/:organizationName/chat` and any conversation identity passed through its router. Organization switching, Settings, signout, search, and resource actions also notify the controller. The signout action reports `onSignOut()`; the portal organization controller delegates it to the root auth controller. Storybook previews use local controllers and no API or browser routing. The portal integrates the shell at `/:organizationName/chat` and redirects `/:organizationName` there after validating the root session. Chat creation and model catalog APIs remain unimplemented.

The workspace label uses `organizationName` from `/api/auth/me`, read from the verified token’s embedded session. Login, renewal, and organization switching populate the snapshot from validated organization membership. Renames become visible when tokens renew; names do not determine session identity or access scope.

The portal provides one `OrganizationController` at the `/:organizationName/*` route boundary through the library’s public `OrganizationControllerContext`, wrapping the workspace shell and all child routes. Portal pages access that context through `useOrganizationController()`. The library interface includes `getCurrentOrganization()` and workspace actions; portal implementations own API calls and navigation. Its store is the single source for workspace data; `getCurrentOrganization()` resolves the active organization from that store rather than maintaining another copy. Root auth state supplies verified identity updates. Child navigation and successful identity refreshes preserve the workspace controller; leaving the organization route or changing organization replaces it. Workspace data is not available through organization context on public authentication pages.
