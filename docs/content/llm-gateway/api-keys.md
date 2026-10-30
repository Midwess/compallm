---
title: API keys
available: false
---

# API keys

## Purpose and status

This page defines the proposed identity and lifecycle of Compallm API keys used
to call the LLM Gateway. It is not an implemented API or a description of
provider credentials.

Decision status: Membership ownership, multiple keys per member, digest-only
storage, expiration, and revocation are proposed. Key format, creation and
management authorization, and endpoint contracts remain unresolved.

Implementation status: Design only. The table exists in `dbdiagram.dbml`, but
no migration, service, or API implements it.

## Ownership

An API key belongs to one organization membership, identified by
`(organization_id, user_id)`. A member may own several keys so separate clients
or environments do not need to share one credential. The composite foreign key
prevents a key from naming a user who is not a member of that organization.

The key identity is retained on logical-model usage when the Gateway
authenticates through that key. Composite usage constraints ensure the key,
member, and usage record have the same organization.

## Secret storage

`api_keys.value` stores a unique SHA-256 digest of the generated
secret. Compallm must return the plaintext only when the key is created and
must never persist or log it. Authentication hashes the supplied credential,
looks up the digest, and then checks the key lifecycle and membership state.

The digest is an authentication value, not a provider API key. Provider
credentials remain on provider connections and must never be returned through
Gateway key-management or usage APIs.

## Lifecycle

Each key has a member-defined name and four relevant timestamps:

- `created_at` records issuance;
- optional `expires_at` makes the key invalid after a configured time;
- optional `revoked_at` disables the key without deleting its identity;
- optional `last_used_at` records the most recent accepted authentication.

A revoked, expired, disabled-member, or disabled-organization key must not
authorize a provider call. Revocation is preferable to deletion when retained
usage still refers to the key. The current diagram restricts deletion through
usage attribution, but the final retention behavior remains a policy decision.

## Open questions

- What plaintext key format and secret entropy are required?
- Which roles may create, list, revoke, and delete keys for another member?
- Is `last_used_at` updated synchronously, asynchronously, or approximately?
- How are key expiration and revocation represented in public API errors?
- May a revoked key be deleted after its usage retention period ends?

## Related decisions

- [LLM Gateway](adr.md)
- [Usage accounting](../usage-administration/accounting.md)
- [Organization and membership](../organization/adr.md)
- [Data and privacy](../data-privacy/adr.md)
