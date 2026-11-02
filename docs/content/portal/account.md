---
title: Account settings design
available: false
---

# Account settings design

Decision status: Accepted Penpot design. Implementation status: Canvas prototype only; application actions remain pending.

Personal settings includes Preferences, Profile, and Account. All three compose the shared Settings sidebar and layout from the Preferences design page. Profile retains its existing fields and Save changes action, with a separate Sign out action below the profile form. Sign out follows the planned session revocation and cookie-clearing behavior in [Login](../login/adr.md).

Account has a Danger zone section outside a rounded settings card. The card exposes Delete account with a destructive button. Clicking it opens a confirmation with Cancel and Delete account actions; opening the dialog performs no deletion. The confirmation identifies the signed-in account and requires the exact phrase `I want to delete <account email>` in a text field. Delete account remains disabled until the phrase matches. The dialog describes the request without promising unconfirmed retention or workspace-deletion behavior. The canvas makes no authentication or deletion API calls.

Before implementing deletion, resolve owned-workspace handling, last-owner restrictions, retained billing/usage records, external-provider data, recovery or grace periods, and any identity-verification requirement. A user account deletion request must not implicitly establish a policy for deleting shared organization data.
