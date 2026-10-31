---
title: "Portal architecture"
available: false
---

# Portal architecture

The portal is a React application built with Vite. Axum serves it and the API from one origin, proxying Vite in development and serving built assets in production. API calls use relative URLs.

## UI library boundary

`libs/storybook/` owns reusable React components, page compositions, fonts, and compiled styles. It publishes ESM JavaScript, TypeScript declarations, and stylesheet exports. React and React DOM are peer dependencies. The portal consumes package exports; the library does not import portal source.

Storybook renders the same implementations with sample data and caller-supplied handlers. Previews run without Rust, PostgreSQL, authentication, or generated API clients. Library packaging and page previews exist; portal/API integration remains pending.

## Application integration

The portal owns routing, authenticated requests, and application state. Each library page accepts one stable controller implementing the library's interface. The library supplies a vanilla Zustand store factory with default state and optional initial overrides. A scoped React Context carries the controller; individual UI parts subscribe to the state values they display. UI events call controller methods, which update the store. Preview controllers do not send mail, persist credentials, or call provider APIs. Portal integration remains pending.

Organization membership and backend authorization determine access to workspace resources. Component state and navigation do not establish permission.

See [component architecture](components.md), [provider configuration](add-provider.md).
