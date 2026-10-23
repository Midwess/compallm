---
title: Organization plugins
tags:
  - features
---

# Organization plugins

## Status

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Release priorities and unresolved rules are tracked in [the roadmap](../roadmap.md) and [business decisions](../decisions.md).

## Scope

Extend what models can do through tools and integrations managed by the organization.

- Add, configure, enable, and disable plugins for the workspace.
- Declare each plugin's available tools, required credentials, and requested permissions.
- Control which members and conversations may use a plugin.
- Distinguish reading information from actions that change external systems.
- Support confirmation before sensitive external actions, according to organization policy.
- Show tool activity, results, failures, and action history.
- Manage plugin versions and revoke access when an integration is removed.

## Behavior examples

### Controlled plugin access
The system should restrict plugin invocation according to organization configuration, member access, and plugin permissions.

#### Example: Disabled plugin
- **WHEN** a conversation attempts to invoke a plugin disabled by its organization
- **THEN** the system rejects the invocation without executing the external tool

## Related pages

- [Data and privacy](data-privacy.md)
- [Product overview](../product.md)
- [Business decisions](../decisions.md)
