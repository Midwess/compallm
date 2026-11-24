<script lang="ts">
  import { untrack } from 'svelte'
  import { onDestroy } from 'svelte'
  import { browser } from '$app/env'
  import { page } from '$app/state'
  import type { Snippet } from 'svelte'
  import { getPortalControllers } from './context'
  import { organizationRoot } from './auth/controller'
  import { errorMessage } from './graphql-client'
  import type { AuthMode } from '@compallm/storybook/pages/auth'
  import { AppLoadingOverlay } from '@compallm/storybook/components/app-loading-overlay'
  let { mode, organization, children }: { mode?: AuthMode; organization?: string; children?: Snippet } = $props()
  const { auth, url } = getPortalControllers()
  const stopInitial = untrack(() => (mode ? auth.start(mode) : () => {}))
  onDestroy(stopInitial)
  let ready = $state(untrack(() => auth.store.getState().status === 'authenticated' || !!mode))
  let failure = $state<string | undefined>()
  let attempt = $state(0)
  let accessMode = $derived(mode)
  let organizationName = $derived(organization)
  $effect(() => {
    if (!browser) return
    const verifiedUser = page.data.user
    const activeMode = accessMode,
      expectedOrganization = organizationName
    void attempt
    return untrack(() => {
      ready = auth.store.getState().status === 'authenticated' || !!activeMode
      failure = undefined
      let active = true
      auth.seed(verifiedUser ?? null)
      const stop = activeMode ? auth.start(activeMode) : () => {}
      const check = activeMode ? auth.ensureGuestAccess() : auth.ensureCurrentUser()
      void check
        .then(async (user) => {
          if (!active) return
          if (activeMode && user) {
            await url.navigate(organizationRoot(user), { replace: true })
            return
          }
          if (!activeMode && !user) {
            const location = url.store.getState()
            const search = new URLSearchParams({ return_to: location.pathname + location.search + location.hash })
            await url.navigate('/login?' + search, { replace: true })
            return
          }
          if (!activeMode && user && expectedOrganization !== user.organizationName) {
            await url.navigate(organizationRoot(user), { replace: true })
            return
          }
          ready = true
        })
        .catch((error) => {
          if (active) failure = errorMessage(error)
        })
      return () => {
        active = false
        stop()
      }
    })
  })
</script>

{#if ready}{@render children?.()}{:else}<AppLoadingOverlay active error={failure} onRetry={() => attempt++} />{/if}
