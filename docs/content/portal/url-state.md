---
title: "URL state and controller scope"
available: false
---

# URL state and controller scope

Implementation scope: Root URL navigation and restoration of non-secret provider
creation drafts. Saved-provider backend loading and server-stored large drafts
remain planned. Controllers receive explicit constructor dependencies;
components supply the existing instances from context.

## Ownership

One root `UrlController` owns application navigation and the current location
snapshot. React Router remains responsible for route matching, lazy loading,
loaders, history, and rendering. The URL controller adapts that concrete router;
it does not introduce another router or history stack. Components and feature
controllers do not call browser history or React Router navigation directly.

The pathname selects the organization, resource, and workflow step. Search
parameters contain restorable non-secret state. Feature controllers decode the
location into their existing observable stores. Those stores are projections
for granular subscriptions, not independently persisted copies of URL state.

Server records remain authoritative for saved providers and permissions. URL
values are untrusted input and cannot establish organization access, a saved
baseline, a successful connection test, or permission to save.

## Controller dependencies

Controllers declare only the dependencies they need in their constructors.
Components obtain instances from context and inject them when creating a child
controller. There is no `.parent` property, ancestor traversal, service registry,
or dependency inheritance. For example, the provider route injects the existing
URL and organization controllers into its provider controller; the organization
route injects the URL and auth controllers into its organization controller.

The library's public context module defines the URL, auth, organization, and
feature contexts once. A root URL context wraps the application alongside auth;
the organization and provider workflow contexts wrap their own route subtrees.
Contexts provide component access, and constructor parameters provide
controller-to-controller access. Feature controllers never call React hooks.
Constructors have no subscription or request side effects. Explicit start/stop
methods own subscription lifetimes.

## Provider addresses

For an organization named `Research team`:

```text
/Research%20team/providers/new/choose-provider
/Research%20team/providers/new/configure-provider?provider=openai
/Research%20team/providers/new/configure-provider?provider=custom&name=Internal
/Research%20team/providers/123/configure-provider
/Research%20team/providers/new/configure-provider?clone_from=123
```

The organization name remains the first path segment. `/providers/new` redirects
with replacement to `new/choose-provider`. Selecting a provider pushes the
configuration URL. A saved provider is addressed by its stable ID. A clone reads
its source through an authorized API, prepares a new draft, and never copies
credentials or a prior test result. Saved read/update/clone APIs remain planned.

Provider query parameters are owned by a feature codec:

| Parameter | Meaning |
| --- | --- |
| `provider` | Selected catalog identity or `custom`. |
| `q`, `model_q` | Provider and model searches. |
| `name`, `base_url`, `protocol`, `mode` | Non-secret connection draft fields. |
| `models` | Versioned JSON array of model draft IDs, identifiers, context limits, thinking levels, enabled states, and three explicit decimal-string prices. |
| `dialog`, `model` | Open model action (`add-model` or `edit-model`) and edited model draft ID. |
| `model_name`, `context_tokens`, `thinking`, `input_price`, `output_price`, `cached_price` | Unsaved fields in the open model dialog. |
| `clone_from` | Source saved-provider ID; never permission to read it. |
| `v` | Query-schema version when serialized model data is present. |

Examples show decoded values for readability; `URLSearchParams` performs actual
encoding. Model identity uses stable draft IDs, not names that change while
editing. Prices remain decimal strings. Explicit zero, missing values, and
empty values retain their distinct meanings. Incomplete input is restored as
input and validated before saving; typing must not be silently replaced by
valid defaults.

## Update flow

1. The user invokes a feature-controller action.
2. The controller validates the requested transition and builds a typed patch
   for its feature's URL codec.
3. The injected URL controller applies that patch to the latest intended location,
   preserving parameters owned by other scopes, and asks React Router to navigate.
4. On the committed location, feature controllers decode their owned values and
   update their observable projections. Reload, deep links, and Back/Forward use
   the same decoder.

URL application never calls URL-writing actions again. Codecs distinguish
initial hydration from user intent, avoiding feedback loops. Identical targets
produce no navigation. Functional patches and serialized writes prevent rapid
field updates from overwriting one another. Stale transitions and requests are
guarded by location revision and cancellation.

Use history push for workflow/entity changes and opening a model dialog. Use
replacement for typing, filtering, and canonical corrections; do not create a
history entry per keystroke. Input remains responsive while navigation settles. Writes are immediate;
there is no debounce window that drops the final draft edit on reload. Query-only updates do not remount the
workflow controller or refetch identity/catalogs unnecessarily. Authorization
still runs when identity, organization, or protected resource scope changes.

Keep the provider controller mounted at the `providers/new` workflow boundary,
with chooser and configuration as child routes. Leaving that workflow disposes
its subscriptions, clears credentials, and cancels pending operations. Changing
organizations replaces the scoped controllers. Root URL and auth instances
survive ordinary navigation.

## Reload and dismissal

Reload restores the step, selected provider, searches, connection fields,
configured models, and open model draft from the URL. Saved-provider pages load
the authorized server baseline before comparing edits. Editing a URL does not
change the saved baseline. Changes to tested configuration invalidate the test
result. Pending operations, notifications, hover/focus, discard confirmations,
and connection-test outcomes are transient and are not restored as facts.

API keys, passwords, OTP codes, tokens, and current-user data never enter URL
parameters or fragments. An unsaved API key is memory-only and must be entered
again after reload. The existing saved credential stays server-side. A URL can
still disclose non-secret configuration and names through history and sharing;
portable URLs are not a credential-storage mechanism.

Cancel and Close use the URL controller to leave the model-dialog address.
Modified drafts require the existing `Dialog.Discard` confirmation before a
navigation is committed, including Back/Forward or leaving the workflow.
Authentication redirects to signin bypass draft guards after session loss.
Keeping changes retains both the current URL and draft. Confirming discard
commits the pending navigation and removes the dialog's draft parameters without
changing configured models. Saving applies the draft to `models` and removes
its dialog parameters in one transition. A native unload warning can protect
memory-only credentials on full reload; it cannot preserve those credentials.

## Limits and remaining decisions

- Query serialization uses version 1 and an initial 16 KiB URL budget. Oversized
  updates are rejected with feedback; data is never silently truncated. Deployment
  URL limits may require a smaller budget. Large model drafts may
  require an authenticated draft record addressed by an opaque ID; that needs
  backend support and a retention policy. Do not silently truncate data or
  replace portable state with tab-local storage.
- Decide which non-secret configuration is acceptable in shareable URLs. Private
  endpoint names and model identifiers can still be sensitive in some deployments.
- Writes are immediate. Functional patches merge against the latest intended
  URL so rapid input cannot overwrite another field.
- Secrets must be re-entered after reload. A native unload warning is used
  while a credential or modified model draft would be lost.

Implementation validation should cover direct entry and reload, rapid input,
step navigation, Back/Forward, invalid queries, saved-baseline dirty comparison,
discarded/retained model edits, organization changes, and absence of credentials
from every generated URL. Provider read/update APIs and backend draft persistence remain outside this implementation.
