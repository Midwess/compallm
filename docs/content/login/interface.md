---
title: "Authentication page integration"
available: false
---

# Authentication page integration

The component library implementation is maintained in the separate UI change; this page describes its target architecture and integration contract.

The React library exports `SignupPage` and `SigninPage` from their page-specific package entry points. Each page accepts request-code and verify-code callbacks; successful verification invokes a completion callback. API calls, credential delivery, and application routing belong to the consumer. Portal/API integration remains pending.

## Request state

Request-code admission must succeed before entering code verification. Failed admission preserves the submitted email. Signup and signin use distinct endpoints; an existing signup email produces a conflict and must not silently trigger signin.

Verification accepts six ASCII digits and allows only one request in flight. A complete code triggers verification. Suppress duplicate verification of an unchanged failed code; changing the code permits another attempt. Temporary failures support explicit retry without requiring code re-entry. Failed verification preserves the code and email context.

Resending explicitly requests a replacement challenge. Respect server-provided retry delays; do not invent a fixed cooldown. Failed or throttled resend preserves the current challenge. Successful admission clears the entered code. Queue admission does not guarantee inbox delivery.

## Preview isolation

Storybook provides local handlers and route previews at `/signup` and `/signin`. Preview handlers never send mail, persist credentials, or call APIs. The example verification code is `123456`.

Authentication policy, token lifetimes, code reuse, and organization creation are owned by the [email flow](email.md) and [login architecture](adr.md).
