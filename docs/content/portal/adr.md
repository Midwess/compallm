---
title: "Portal architecture"
available: false
---

# Portal architecture

The component library implementation is maintained in the separate UI change; this page describes its target architecture and integration contract.

The portal is a React application built with Vite. Axum serves it and the API from one origin, proxying Vite in development and serving built assets in production. API calls use relative URLs.

## UI library boundary

`libs/storybook/` owns reusable React components, page compositions, fonts, and compiled styles. It publishes ESM JavaScript, TypeScript declarations, and stylesheet exports. React and React DOM are peer dependencies. The portal consumes package exports; the library does not import portal source.

Storybook renders the same implementations with sample data and caller-supplied handlers. Previews run without Rust, PostgreSQL, authentication, or generated API clients. Portal/API integration remains pending.

## Application integration

The portal owns routing, authenticated requests, and application state. Library pages expose callbacks and navigation inputs so consumers can supply those dependencies. Preview handlers do not send mail, persist credentials, or call provider APIs.

Organization membership and backend authorization determine access to workspace resources. Component state and navigation do not establish permission.

See [component architecture](components.md), [provider configuration](add-provider.md).
