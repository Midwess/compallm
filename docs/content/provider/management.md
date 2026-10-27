---
title: Managing LLM providers
---

# Managing LLM providers

LLM provider connections are organization-owned resources. They hold the
endpoint, protocol, and protected credentials Compallm needs to call an
upstream provider without exposing those credentials to organization members.

## Who can manage providers

An authenticated organization **owner** or **administrator** can:

- list the organization's provider connections;
- add an OpenAI, Anthropic, or OpenAI-compatible connection;
- change a connection's non-secret configuration or replace its credential;
- enable or disable a connection;
- remove a connection from the organization.

An ordinary **member** cannot add, change, or remove provider connections. A
caller must hold the required role in the organization that owns the provider;
being an owner or administrator in another organization does not grant access.

Authorization uses organization membership and role.

## Credential boundaries

Provider credentials are write-only from the management interface. Responses
may identify a connection and show its safe configuration, but must not return
the API key or another reusable secret. Logs and client-visible errors must not
contain credentials.

Replacing a credential accepts a new protected value without revealing the
old value. Removing a provider also removes its model routes; it does not expose
the removed credential.

Credential encryption, key rotation, and provider-specific verification remain
unresolved. Until those decisions are implemented, the existence of a provider
record does not imply that Compallm has verified the endpoint or credential.

Disabling a provider preserves its configuration, credential, and model routes
for management, but makes all of those routes unavailable. Enabling the
provider does not override a disabled model or route; routing requires all
three availability flags to be enabled.

## Expected behavior

### Authorized management

- **WHEN** an owner or administrator manages a provider owned by their current
  organization
- **THEN** Compallm performs the requested management operation without
  returning the provider credential

### Member denied

- **WHEN** an ordinary member attempts to add, change, or remove a provider
- **THEN** Compallm denies the operation before changing provider data

### Cross-organization access denied

- **WHEN** a caller attempts to manage a provider owned by another organization
- **THEN** Compallm denies the operation even if the caller is an owner or
  administrator elsewhere

### Disabled provider retained for management

- **WHEN** an owner or administrator disables a provider connection
- **THEN** Compallm retains it in management listings without returning its
  credential
- **AND THEN** routing does not use any route associated with that provider

## Related pages

- [Provider management decisions](adr.md)
- [Model management](../model/adr.md)
- [Organization and membership](../organization/adr.md)
- [LLM Router](../llm-router/adr.md)
