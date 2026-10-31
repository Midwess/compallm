---
title: "Component library"
available: false
---

# Component library

The component library implementation is maintained in the separate UI change; this page describes its target architecture and integration contract.

The npm package in `libs/storybook/` supplies reusable React components and pages to the portal. Components are defined in `src/components/`; page compositions in `src/pages/` reuse those components. Stories import the production implementations and provide sample data.

## Package contract

Publish ESM JavaScript, TypeScript declarations, and compiled styles. React and React DOM are peer dependencies. Keep the library build separate from the static Storybook build. Consumers import supported package entry points rather than private source paths.

Pages accept caller-supplied actions, completion callbacks, and navigation inputs. Keep API clients, authentication storage, application routing, and provider credentials outside the library. Previews substitute local handlers and require no backend.

## Component behavior

Form controls expose values, changes, validation, pending state, and disabled state through their public APIs. Preserve native semantics and accessible names. Floating option lists support keyboard selection, dismissal, and focus restoration without changing the surrounding document flow.

Shared styles and assets are packaged with the library. Import only required Phosphor SVG components; avoid a runtime registry that bundles the entire icon catalog. Decorative icons are hidden from assistive technology; icon-only actions have accessible names.

See [authentication page callbacks](../login/interface.md).
