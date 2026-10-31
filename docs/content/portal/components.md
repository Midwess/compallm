---
title: "Component library"
available: false
---

# Component library

The npm package in `libs/storybook/` supplies reusable React components and pages to the portal. Components are defined in `src/components/`; page compositions in `src/pages/` reuse those components. Stories import the production implementations and provide sample data.

## Package contract

Publish ESM JavaScript, TypeScript declarations, and compiled styles. React and React DOM are peer dependencies. Keep the library build separate from the static Storybook build. Consumers import supported package entry points rather than private source paths.

Components and pages use standard Tailwind utilities directly in React, including the default spacing scale. Do not define custom spacing tokens or override the default spacing unit. Keep theme configuration and font imports in the Tailwind entry files; do not add component, page, or story stylesheets. The library compiles those utilities into its exported stylesheet so consumers do not need Tailwind to use the package.

`npm run dev` runs Storybook through Vite with Tailwind compilation and CSS hot updates. Preview scanning includes components, pages, stories, and Storybook configuration; no separate CSS build is required when editing utilities. `npm start` runs the same preview server.

`npm run build` produces minified, tree-shakeable ESM, declarations, and minified Tailwind output. Scan production components and pages for the package stylesheet; preview-only utilities and Storybook code remain outside the published library. `npm run build:storybook` separately builds preview demos and the route examples.

Pages accept a single controller implementing an exported page interface. The controller exposes a stable store and explicit event/action methods. Export a factory function that initializes a fresh vanilla Zustand store from library defaults and optional caller overrides. The interface does not require inheritance; a consumer class can implement it.

Pass the stable controller through scoped React Context. Context carries the instance, not a copy of its changing state. UI parts use selectors to subscribe to individual values; avoid whole-store subscriptions at page or form roots. Subscribe to primitive or stable derived values, and use shallow comparison when combining values into a new object. Changing a field or countdown must not rerender unrelated page sections. Local focus and disclosure state can remain in base components.

UI events call controller methods; UI code does not mutate the store. Controller methods read the current snapshot when invoked and own asynchronous requests and state transitions. Keep API clients, authentication storage, application routing, and provider credentials outside the library. Consumer-created controllers are initialized once per page instance. Preview classes implement the same interfaces without a backend, and their owner starts and cleans up timers and pending-operation guards.

## Component behavior

Form controls expose values, changes, validation, pending state, and disabled state through their public APIs. Preserve native semantics and accessible names. Floating option lists support keyboard selection, dismissal, and focus restoration without changing the surrounding document flow.

Shared styles and assets are packaged with the library. Import only required Phosphor SVG components; avoid a runtime registry that bundles the entire icon catalog. Decorative icons are hidden from assistive technology; icon-only actions have accessible names.

See [authentication page callbacks](../login/interface.md).
