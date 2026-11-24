<script lang="ts">
  import { onMount } from 'svelte'
  import { getPortalControllers } from '../../../../../context'
  const { url } = getPortalControllers()
  onMount(() => {
    const current = url.store.getState()
    void url.navigate(current.pathname.replace(/\/$/, '') + '/choose-provider' + current.search, { replace: true })
  })
</script>
