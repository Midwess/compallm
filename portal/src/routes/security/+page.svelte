<script lang="ts">
  import { onMount } from 'svelte'
  import { SecurityEnhancementPage } from '@compallm/storybook/pages/passkey'
  import { getPortalControllers } from '../../context'
  import { organizationRoot } from '../../auth/controller'
  import { errorMessage } from '../../graphql-client'
  import { AppLoadingOverlay } from '@compallm/storybook/components/app-loading-overlay'
  const { auth, url, passkeys } = getPortalControllers()
  let ready = $state(auth.store.getState().status === 'authenticated')
  let failure = $state<string | undefined>()
  function next() {
    const user = auth.store.getState().currentUser
    auth.securityOfferSession = undefined
    if (user) void url.navigate(organizationRoot(user), { replace: true })
  }
  onMount(() => {
    let active = true
    let stop = () => {}
    void auth
      .ensureCurrentUser()
      .then((user) => {
        if (!active) return
        if (!user) {
          void url.navigate('/login', { replace: true })
          return
        }
        auth.securityOfferSession = undefined
        ready = true
        const close = passkeys.startView()
        const unsubscribe = passkeys.store.subscribe((state, before) => {
          if (before.dialog?.kind === 'add' && !state.dialog && state.records !== before.records) next()
        })
        stop = () => {
          unsubscribe()
          close()
        }
      })
      .catch((error) => {
        if (active) failure = errorMessage(error)
      })
    return () => {
      active = false
      stop()
    }
  })
</script>

{#if ready}<SecurityEnhancementPage onSkip={next} />{:else}<AppLoadingOverlay
    active
    error={failure}
    onRetry={() => window.location.reload()}
  />{/if}
