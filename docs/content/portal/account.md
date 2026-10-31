---
title: "Account lifecycle"
available: false
---

# Account lifecycle

Account settings apply to the signed-in identity. Account deletion remains planned. Signout follows the session revocation and cookie-clearing behavior in [Login](../login/adr.md).

Account deletion requires explicit confirmation identifying the account. The confirmation phrase is `I want to delete <account email>`; no deletion request is submitted before it matches.

Before implementation, resolve owned-workspace handling, last-owner restrictions, retained billing/usage records, external-provider data, recovery or grace periods, and identity verification. Deleting an account must not implicitly delete shared organization data.
