# Compallm UI library

`@compallm/storybook` owns reusable React components, page compositions, styles, and Storybook demos. Build and preview UI here
first; the portal consumes the package later.

```sh
npm ci
npm start
```

Storybook runs at `http://localhost:6006`. Base previews live under **Components**; page compositions belong in `src/pages/`
with demos under **Pages**.

```sh
npm run lint            # Fix lint issues and format source; maximum line length 128
npm run lint:check      # Check without editing
npm run check          # Type-check library, stories, and tests
npm run build          # Optimized ESM UI library, declarations, Tailwind output, and assets in dist/
npm run build:storybook # Standalone preview site in storybook-static/
npm test               # Component browser checks
npm pack --dry-run     # Review npm package contents without publishing
```

Consumers import the library and its compiled stylesheet:

```tsx
import { Button } from '@compallm/storybook/components/buttons'
import '@compallm/storybook/styles.css'

export function Example() {
  return <Button variant="primary">Continue</Button>
}
```

The root export also exposes the components. React and React DOM are peer dependencies. The package includes Inter and uses
Phosphor SVG icons. Tailwind consumers can import `@compallm/storybook/theme.css` after Tailwind to reuse the semantic color and
spacing tokens.

Components, pages, and preview demos use Tailwind utilities directly. There are no custom component or page stylesheets.
Tailwind entry files contain imports and theme configuration only. The library build scans production source, minifies ESM and
generated CSS, and keeps font files separate for caching. Consumers can use the compiled package without installing Tailwind.
React, React DOM, Phosphor, and `tailwind-merge` remain external package dependencies.

Penpot's **Design foundations** is the design reference. Tokens are translated explicitly; canvas changes do not automatically
update this package. Demos use sample data and never require the backend.

The package is source available under Elastic License 2.0; see `LICENSE`.

## Account pages

Preview [signup](http://localhost:6006/signup) and [signin](http://localhost:6006/signin) directly, or open **Pages/signup** and
**Pages/signin** in Storybook. The demo code is `123456`; `registered@example.com` demonstrates an existing signup email. Demo
callbacks never send email or call the backend.

The package exports `@compallm/storybook/pages/signup` and `@compallm/storybook/pages/signin`. Consumers wrap pages in
`AuthControllerContext` with a stable controller. Implement the exported `SignupController` or `SigninController` interface in
the consuming application; classes are supported without inheritance. Page source follows URL folders under `src/pages/`. Static
previews include `/signup/` and `/signin/`.

```tsx
import { SignupPage, createSignupStore, type SignupController } from '@compallm/storybook/pages/signup'
import { AuthControllerContext } from '@compallm/storybook/context'

// In the consumer controller: one independent store, with library defaults and optional overrides.
const store = createSignupStore({ email: 'alex@example.com' })

export function SignupRoute({ controller }: { controller: SignupController }) {
  return (
    <AuthControllerContext value={controller}>
      <SignupPage />
    </AuthControllerContext>
  )
}
```

Create the controller once per page instance. It owns the store and implements methods such as `onEmailChanged`,
`onCodeChanged`, `requestCode`, `resendCode`, and `retry`. Methods read `store.getState()` and update state through
`store.setState()`. The UI receives a read-only subscription contract and calls controller methods for changes.

A scoped Context distributes the stable controller. Leaf components select only the values they need from Zustand; the page
shell never subscribes to the whole state. `createSigninStore` follows the same initialization pattern. Initial values are
applied once, and later changes go through controller methods. For server rendering, pass initial values into the factory so its
initial snapshot matches the rendered page. Optional `links` provide signup/signin destinations.

Storybook's `DemoAuthController` is a separate class implementing this interface. Its owner starts countdowns after mounting and
stops them on cleanup; constructors have no side effects. The **Pages/signup → Subscriptions** demo shows actual commit counts
for independent UI parts. Application API calls, credentials, and navigation belong to the consumer.
