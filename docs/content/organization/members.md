---
title: "Membership administration"
available: false
---

# Membership administration

Membership records associate users with an organization and a role. Owner membership is the ownership record. Administration operations are planned; their authorization and persistence contracts remain unresolved.

Member queries operate within the current organization. Membership creation times and presence activity are not recorded; do not infer joined dates or last-seen activity from IDs.

Removing membership changes access to organization resources. Permissions, self-removal, owner removal, and the requirement to retain an owner must be defined before exposing removal operations. Invitations, role changes, member visibility, search, and activity tracking also remain unresolved.

Per-member usage budgets are governed by the [Gateway](../llm-gateway/member-limits.md), independently of membership role. See [organization architecture](adr.md).
