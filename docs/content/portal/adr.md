---
title: "Portal architecture"
available: false
---

# Portal architecture

The portal is a React application built with Vite. Axum serves it and the API from one origin, proxying Vite in development and serving built assets in production. API calls use relative URLs.

## UI library boundary

`libs/storybook/` owns reusable React components, page compositions, fonts, and compiled styles. It publishes ESM JavaScript, TypeScript declarations, and stylesheet exports. React and React DOM are peer dependencies. The portal links `@compallm/storybook` through the npm file dependency `file:../libs/storybook` and consumes its compiled package exports. Build the library with `npm --prefix libs/storybook run build` before resolving its JavaScript, declarations, or styles in the portal. React and React DOM resolve from the portal in Vite to keep one runtime across the local link. The library does not import portal source.

Storybook renders the same implementations with sample data and caller-supplied handlers. Previews run without Rust, PostgreSQL, authentication, or generated API clients. Library packaging, previews, and portal email authentication integration are implemented.

The portal renders the library signup page at `/` and signin page at `/login`. A portal-owned controller calls the generated email authentication client using cookie delivery; credentials remain in HttpOnly cookies. Successful verification displays the library completion state until an authenticated destination is defined. React Router Data Mode matches `/` and `/login` through lazy route modules. Vite splits those imports into route chunks; shared authentication code is reused. Internal account links navigate without reloading the document. Unknown routes show a not-found page. A shared route boundary handles loading failures, and pending navigation is announced while the current page remains visible. Direct visits still load the application HTML through the Axum frontend fallback. `npm run lint` applies ESLint fixes and Prettier formatting; `npm run lint:check` validates without editing. Generated API contracts are excluded from linting and formatting.

## Application integration

The portal owns routing, authenticated requests, and application state. Consumers wrap each library page in `AuthControllerContext` with one stable controller implementing the library's interface. The library supplies a vanilla Zustand store factory with default state and optional initial overrides. A scoped React Context carries the controller; individual UI parts subscribe to the state values they display. UI events call controller methods, which update the store. Preview controllers do not send mail, persist credentials, or call provider APIs. The portal supplies email authentication controllers.

Organization membership and backend authorization determine access to workspace resources. Component state and navigation do not establish permission.

See [component architecture](components.md), [provider configuration](add-provider.md).
