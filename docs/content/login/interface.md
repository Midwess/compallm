---
title: "Authentication page integration"
available: false
---

# Authentication page integration

The React library exports `SignupPage` and `SigninPage` from their page-specific package entry points. Each page accepts one `controller` implementing `SignupController` or `SigninController`. The library exports `createSignupStore` and `createSigninStore`, which initialize fresh vanilla Zustand stores with defaults and optional initial state overrides. API calls, credential delivery, completion handling, and application routing belong to the consumer. Portal/API integration remains pending.

Controllers can be classes. They expose a stable store for UI subscriptions and methods for email/code changes, code requests, retry, resend disclosure, and returning to email entry. Page components access that instance through scoped Context and subscribe only to the fields needed by each UI part. The page shell does not subscribe to changing state. Form state has one owner: the controller.

## Request state

Request-code admission must succeed before entering code verification. Failed admission preserves the submitted email. Signup and signin use distinct endpoints; an existing signup email produces a conflict and must not silently trigger signin.

Verification accepts six ASCII digits and allows only one request in flight. A complete code triggers verification. Suppress duplicate verification of an unchanged failed code; changing the code permits another attempt. Temporary failures support explicit retry without requiring code re-entry. Failed verification preserves the code and email context.

Resending explicitly requests a replacement challenge. Respect server-provided retry delays; do not invent a fixed cooldown. Failed or throttled resend preserves the current challenge. Successful admission clears the entered code. Queue admission does not guarantee inbox delivery.

The controller owner manages timers and ignores responses from cancelled or superseded operations. Constructors and store factories have no request or timer side effects. Replacing or unmounting a demo controller stops its timer and invalidates pending responses. Initial factory overrides seed state once; later changes use controller methods.

## Preview isolation

Storybook provides class-based demo controllers and route previews at `/signup` and `/signin`. Preview controllers never send mail, persist credentials, or call APIs. The example verification code is `123456`.

Authentication policy, token lifetimes, code reuse, and organization creation are owned by the [email flow](email.md) and [login architecture](adr.md).
