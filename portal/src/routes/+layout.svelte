<script lang="ts">
  import '../app.css'
  import { onMount, untrack } from 'svelte'
  import { page, navigating } from '$app/state'
  import { browser } from '$app/env'
  import { configureTransport } from '@compallm/contract/houdini/client'
  import { AuthController } from '../auth/controller'
  import { UrlController } from '../url/controller'
  import { PasskeyController } from '../passkey/controller'
  import { PermissionController } from '../permission/controller'
  import { UserPreferenceController } from '../preference/user/controller'
  import { resolve } from '$app/paths'
  import type { PathnameWithSearchOrHash } from '$app/types'
  import { goto, beforeNavigate, onNavigate } from '$app/navigation'
  import {
    setAuthController,
    setPasskeyController,
    setUserPreferenceController,
    setUrlController,
  } from '@compallm/storybook/context'
  import { PageProgress, AppLoadingOverlay } from '@compallm/storybook'
  import { graphqlClient, GraphqlClient } from '../graphql-client'
  import { setPortalControllers } from '../context'
  import { appController } from '../app/controller'
  import { appearanceController } from '../appearance/controller'
  import { PasskeyDialogs } from '@compallm/storybook/pages/passkey'
  import { AuthVerificationHost } from '@compallm/storybook/pages/auth'
  import Notifications from '../notifications/Notifications.svelte'
  import { clearNotices } from '../notifications/toast'
  let { children, data } = $props()
  const client = browser ? graphqlClient : new GraphqlClient()
  const url = new UrlController()
  const auth = new AuthController(url, client)
  const passkeys = new PasskeyController(auth)
  const preferences = new UserPreferenceController(appController)
  const permissions = new PermissionController()
  untrack(() => {
    url.synchronize(new URL(page.url.href), page.state)
    auth.seed(data.user)
    if (data.preferences) preferences.seed(data.preferences.userPreferences.node)
    if (data.user && data.authority) permissions.seed(data.user, data.authority.me.node)
  })
  const disconnectTransport = browser ? configureTransport(client.request) : () => {}
  setPortalControllers({ auth, url, passkeys, preferences, permissions })
  setPasskeyController(passkeys)
  setUserPreferenceController(preferences)
  setUrlController(url)
  setAuthController(auth)
  let mounted = $state(false)
  onMount(() => {
    client.publishPreparationVersion()
    const navigation = url.connect((href, replace, state) =>
      goto(resolve(href.replace(/^\//, '') as PathnameWithSearchOrHash), {
        replace,
        reset: false,
        state: state ?? {},
        invalidateAll: auth.store.getState().currentUser?.id !== data.user?.id,
      }),
    )
    const stopPasskeys = passkeys.start()
    const stopPreferences = preferences.start(auth)
    const stopAppearance = appearanceController.start()
    const invalid = client.onInvalidAuthorization(auth.invalidAuthorization)
    const stopNotices = auth.store.subscribe((state, before) => {
      if (state.currentUser?.id !== before.currentUser?.id) clearNotices()
    })
    url.synchronize(new URL(page.url.href), page.state)
    mounted = true
    return () => {
      stopPreferences()
      stopPasskeys()
      stopAppearance()
      clearNotices()
      stopNotices()
      auth.dispose()
      invalid()
      navigation()
      disconnectTransport()
    }
  })
  beforeNavigate((navigation) => {
    if (navigation.to?.url) {
      url.beforeNavigation(navigation.to.url, navigation.cancel, () => {
        if (navigation.type === 'popstate' && navigation.delta) history.go(navigation.delta)
        else
          void goto(
            resolve(
              (navigation.to!.url.pathname.slice(1) +
                navigation.to!.url.search +
                navigation.to!.url.hash) as PathnameWithSearchOrHash,
            ),
            {
              reset: false,
            },
          )
      })
    }
  })
  let navigationLoading = $state(false)
  let navigationTimer: ReturnType<typeof setTimeout> | undefined
  let preparedNavigation: Promise<void> | null = null
  onNavigate((navigation) => {
    preparedNavigation = navigation.complete
    clearTimeout(navigationTimer)
    navigationLoading = false
  })
  $effect(() => {
    const current = navigating.complete
    clearTimeout(navigationTimer)
    navigationLoading = false
    if (current) {
      navigationTimer = setTimeout(() => {
        if (navigating.complete === current && preparedNavigation !== current) navigationLoading = true
      }, 100)
    }
    return () => clearTimeout(navigationTimer)
  })
  let adopted = untrack(() => data.user)
  $effect(() => {
    const next = data.user
    if (next === adopted) return
    untrack(() => {
      adopted = next
      const current = auth.store.getState().currentUser
      if (current?.id !== next?.id || current?.organizationId !== next?.organizationId) {
        client.resetScope()
        auth.seed(next)
      }
    })
  })
  $effect(() => {
    if (mounted) url.synchronize(new URL(page.url.href), page.state)
  })
</script>

<svelte:head>
  <title>Compallm</title>
  <meta name="description" content="One workspace for your team’s AI models, providers, and conversations." />
  {#if data.user || page.url.pathname.includes('confirmation') || page.url.pathname === '/invitation' || page.url.pathname === '/security'}<meta
      name="robots"
      content="noindex, nofollow"
    />{/if}
</svelte:head>
<PageProgress active={client.activity.getState().pending > 0 || navigationLoading} />
{@render children()}
<PasskeyDialogs /><AuthVerificationHost /><Notifications />
<AppLoadingOverlay
  active={mounted && appController.store.getState().loading}
  error={appController.store.getState().error ?? undefined}
  onRetry={() => void appController.retry()}
/>
