---
title: Organization plugins
available: false
---

# Organization plugins

## Status

Decision status: Proposed. Implementation status: Under development.

Planned scope for discussion. The examples describe intended behavior, not completed implementation. Open questions are recorded below.

## Context and proposed scope

Extend what models can do through tools and integrations managed by the organization.

- Add, configure, enable, and disable plugins for the workspace.
- Declare each plugin's available tools, required credentials, and requested permissions.
- Control which members and conversations may use a plugin.
- Distinguish reading information from actions that change external systems.
- Support confirmation before sensitive external actions, according to organization policy.
- Show tool activity, results, failures, and action history.
- Manage plugin versions and revoke access when an integration is removed.

## Consequences

Enabling a tool does not grant every member permission to invoke it. External actions require the organization's permission and confirmation policies.

## Behavior examples

### Controlled plugin access
The system should restrict plugin invocation according to organization configuration, member access, and plugin permissions.

#### Example: Disabled plugin
- **WHEN** a conversation attempts to invoke a plugin disabled by its organization
- **THEN** the system rejects the invocation without executing the external tool

## Open questions

- Which plugin types are supported first?
- Which external actions require confirmation, and who defines the policy?
- Which plugin capabilities belong in the first release?

## Related pages

- [Data and privacy](../data-privacy/adr.md)
- [Product overview](../overview.md)
