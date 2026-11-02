---
title: "Authentication page integration"
available: false
---

# Authentication page integration

The React library exports `SignupPage` and `SigninPage` from their page-specific package entry points. Consumers provide the public `AuthControllerContext` from `@compallm/storybook/context` above the pages, supplying one stable controller implementing `SignupController` or `SigninController`. The portal provides this single library context once at the application root for every page. The public controller interface includes current-user state, identity checks, and form lifecycle methods; the portal implements API calls and navigation. Pages consume context; they do not create a provider or accept a controller prop. The library exports `createSignupStore` and `createSigninStore`, which initialize fresh vanilla Zustand stores with defaults and optional initial state overrides. API calls, credential delivery, completion handling, and application routing belong to the consumer. The portal mounts signup at `/` and signin at `/login`, with cookie-delivered credentials through the generated API client. Successful confirmation is followed by `/api/auth/me`; only a confirmed current user is stored in memory and redirected to `/:organizationName/chat`.

Controllers optionally expose `navigate(href)` for client-side account links. Without it, links use ordinary browser navigation. Modified clicks and new-tab navigation keep native anchor behavior.

Controllers can be classes. They expose a stable store for UI subscriptions and methods for email/code changes, code requests, retry, resend disclosure, and returning to email entry. Page components access that instance through scoped Context and subscribe only to the fields needed by each UI part. The page shell does not subscribe to changing state. Form state has one owner: the controller.

## Request state

Request-code admission must succeed before entering code verification. Failed admission preserves the submitted email. Signup and signin use distinct endpoints; an existing signup email produces a conflict and must not silently trigger signin.

Verification accepts six ASCII digits and allows only one request in flight. A complete code triggers verification. Suppress duplicate verification of an unchanged failed code; changing the code permits another attempt. Temporary failures support explicit retry without requiring code re-entry. Failed verification preserves the code and email context.

Resending explicitly requests a replacement challenge. The portal uses the returned OTP `lifetime.throttleSeconds` to count down the 30-second OTP resend throttle, independently of challenge expiry; it does not invent a fixed delay. The SMTP cooldown remains independent. Failed or throttled resend preserves the current challenge. Successful admission clears the entered code. Queue admission does not guarantee inbox delivery.

The controller owner manages timers and ignores responses from cancelled or superseded operations. Constructors and store factories have no request or timer side effects. Replacing or unmounting a demo controller stops its timer and invalidates pending responses. Initial factory overrides seed state once; later changes use controller methods.

## Preview isolation

Storybook provides class-based demo controllers and route previews at `/signup` and `/signin`. Preview controllers never send mail, persist credentials, or call APIs. The example verification code is `123456`.

Authentication policy, token lifetimes, code reuse, and organization creation are owned by the [email flow](email.md) and [login architecture](adr.md).

## Request failures

Authentication form controllers preserve the response `message` as an inline operation error, without translating HTTP status codes into UI labels. Email request errors appear after Continue; confirmation and resend errors stay with the code form. Failed requests preserve input and clear pending state. Editing input clears stale errors. Cancelled or superseded requests are silent. Failed verification supports explicit retry. Other operations, such as signout, may use toasts.

API messages can contain links and basic inline formatting. The shared message renderer sanitizes an explicit markup allowlist before display, retaining the server wording and permitted links. Event handlers, scripts, images, unsafe URLs, and arbitrary styles are removed. Same-origin links use consumer navigation; external HTTP(S) links use normal anchor behavior. Sanitization never grants authentication or replaces API wording with a client status-code message.

The portal's root `AuthController` implements both `SignupController` and `SigninController`. These interfaces share the same contract; the controller's mode selects the signup or signin endpoints. Entering either authentication page resets form state without replacing the root controller or current-user state.

All portal pages access the root instance through `useAuthController()`. Its store holds current-user and form state, with granular subscriptions. `refreshCurrentUser()` requests fresh identity and redirects to `/login` when the API confirms an anonymous session or returns no user. Initial checks on public authentication pages allow anonymous users without redirecting. Network and server failures remain retryable errors rather than evidence of signout. If confirmation succeeds but the identity check fails temporarily, retry checks identity again without repeating confirmation. Controller cancellation suppresses stale identity updates and navigation.

The shared auth contract includes `signOut()`. Organization signout delegates to the root auth controller, which calls `POST /api/auth/signout`. Success cancels pending authentication work, clears in-memory identity and form state, and soft-navigates to `/login`. Duplicate clicks share one operation. API failures show an error toast and leave state available for retry.


## Confirmation addresses

Successful code requests navigate to `/login/confirmation` or
`/signup/confirmation`. Query parameters contain `email`, `resend_at`, and
`expires_at`; deadlines use Unix milliseconds for the browser countdown.
Reload restores the recipient, code-entry step, and remaining throttle from
these values. The OTP code and credential never enter the URL or browser
storage; the credential remains in the HttpOnly cookie. URL metadata grants
no authentication and cannot prove a challenge is valid. Missing or malformed
recipient metadata returns to the corresponding email-entry route.

Confirmation routes and returns to email entry during an active OTP flow do not probe `/api/auth/me`: that endpoint requires a user
access token and its authentication middleware can clear OTP cookies. An
already-known authenticated identity may redirect to its organization without
a request. A successful confirmation still verifies current-user identity
before workspace navigation. An expired display deadline shows the expired-code
message and allows resend once the throttle ends; the API enforces actual token
validity. Using a different email returns to email entry and clears the code.
