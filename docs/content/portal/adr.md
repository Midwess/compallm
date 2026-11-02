---
title: "Portal architecture"
available: false
---

# Portal architecture

The portal is a React application built with Vite. Axum serves it and the API from one origin, proxying Vite in development and serving built assets in production. API calls use relative URLs.

## UI library boundary

`libs/storybook/` owns reusable React components, page compositions, fonts, and compiled styles. It publishes ESM JavaScript, TypeScript declarations, and stylesheet exports. React and React DOM are peer dependencies. The portal links `@compallm/storybook` through the npm file dependency `file:../libs/storybook` and consumes its compiled package exports. Build the library with `npm --prefix libs/storybook run build` before resolving its JavaScript, declarations, or styles in the portal. React and React DOM resolve from the portal in Vite to keep one runtime across the local link. The library does not import portal source.

Storybook renders the same implementations with sample data and caller-supplied handlers. Previews run without Rust, PostgreSQL, authentication, or generated API clients. Library packaging, previews, and portal email authentication integration are implemented.

The portal renders the library signup page at `/` and signin page at `/login`. A portal-owned controller calls the generated email authentication client using cookie delivery; credentials remain in HttpOnly cookies. After signup or signin confirmation, the portal requests `GET /api/auth/me`, stores its response in the root auth controller, and soft-navigates to `/:organizationName/chat` only after that succeeds. React Router Data Mode lazy-loads signup, signin, and the protected organization/chat routes. Vite splits those imports into route chunks; shared authentication code is reused. Internal account links navigate without reloading the document. Unknown routes show a not-found page. A shared route boundary handles loading failures, and pending navigation is announced while the current page remains visible. Direct visits still load the application HTML through the Axum frontend fallback. `npm run lint` applies ESLint fixes and Prettier formatting; `npm run lint:check` validates without editing. Generated API contracts are excluded from linting and formatting.

## Application integration

The portal owns routing, authenticated requests, and application state. The library’s public `AuthControllerContext` wraps the application once at the root with one stable controller. Its interface exposes current-user state, identity checks, and authentication-form actions. All portal pages and library components consume this same context; signup and signin add no providers. The library supplies a vanilla Zustand store factory with default state and optional initial overrides. Individual UI parts subscribe to the state values they display. UI events call controller methods, which update the store. Preview controllers do not send mail, persist credentials, or call provider APIs. The portal's single root `AuthController` owns current-user state and implements both authentication-page contracts. Entering signup or signin resets only its form state.

Organization membership and backend authorization determine access to workspace resources. Component state and navigation do not establish permission.

Portal logic uses `lodash-es` for shared object checks and collection lookups,
with direct function imports to avoid loading the library's barrel. Keep
native operations where they are simpler or preserve validation semantics,
including ASCII-only email normalization, exact OTP validation, and abort
signals. Dependency use is a readability choice; performance claims require
measurement of the relevant behavior and production bundle.

See [component architecture](components.md), [provider configuration](add-provider.md).

## Error notifications

A single Sonner host is mounted outside the router, so notifications survive navigation and route errors. Custom notifications render the library `Toast`; controllers choose contextual inline feedback or the shared notification function. Login/signup form errors remain inline. API JSON `message` wording is preserved; an allowlisted, sanitized renderer supports links and basic inline formatting. Missing or malformed messages use a generic fallback; aborts and stale controller responses are silent. Repeated identical messages share one notification ID. Info, error, and warning notifications automatically close after 15 seconds and can be dismissed earlier. Field validation stays inline. No server error payload change is required.

## Root session state

The library’s `AuthControllerContext` wraps the router and notification host. Pages use `useAuthController()` to access the same stable instance and refresh identity at any time. Its in-memory Zustand store holds the current-user response, with explicit unknown, checking, authenticated, anonymous, and error states. No user data or credentials are persisted in browser storage. A reload restores session state by calling `/api/auth/me` with cookies. Concurrent initial checks share one request, and aborted or stale responses cannot overwrite newer identity state. A refresh that confirms an anonymous session or returns no user clears identity and soft-navigates to `/login`; guest-route checks allow anonymous users to remain on signup or signin. Network and server failures enter the error state without treating the user as signed out.

Each entry to `/` or `/login` rechecks `/api/auth/me` instead of trusting a cached signed-out result. Public authentication routes redirect confirmed users to `/:organizationName/chat`. The organization route waits for a session check before rendering; a 401 redirects to `/login`, while network/server failures show a recoverable check-error state. The `/:organizationName` entry soft-redirects to `/:organizationName/chat`. Organization and chat controllers receive identity from root auth state. The sidebar displays `organizationName` from `/api/auth/me`, which returns the verified session’s signed organization-name snapshot without an organization query. Renames appear after token renewal or a new login. Missing model catalogs and chat APIs are not replaced with fictional production data; their actions report unavailable functionality until supported.

Workspace URLs start with the real organization name, URL-encoded as a single path segment. The portal derives the canonical path from `/api/auth/me`; a route for another name redirects to the signed-in organization instead of changing session scope.

The organization route provides one shared `OrganizationController` through the library’s public `OrganizationControllerContext` around its shell and child outlet. `useOrganizationController()` consumes that context and exposes the library interface, including current-organization information and workspace actions, to all organization child pages. The portal implements the interface and owns API calls and navigation. The controller owns workspace state and derives the current organization from its store. Auth refreshes update verified organization information without clearing workspace resources; child navigation preserves the instance. Leaving the organization route or changing organization ends that scope.

Provider creation is lazy-loaded at `/:organizationName/providers/new` under the
organization shell. A portal-owned controller implements the shared library
contract for choosing presets and editing unsaved configuration. Provider model
catalogs, saving, and upstream testing remain pending backend integration; their
absence is reported without simulating successful operations.


## URL state and controller dependencies

The [URL-state architecture](url-state.md) defines one root URL controller over
React Router and explicit constructor injection. Components pass existing URL,
auth, or organization instances only to controllers that need them; no controller
uses a `.parent` property or ancestor lookup. The provider creation workflow uses
explicit chooser/configuration paths and restores non-secret fields, configured
models, searches, and model-dialog drafts from query parameters. Credentials
remain memory-only. Saved-provider backend loading and large server-stored
drafts remain planned.
