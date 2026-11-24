<script lang="ts">
  import { onMount } from 'svelte'
  import { getPortalControllers, getWorkspaceControllers } from '../../../context'
  const { preferences } = getPortalControllers()
  const { organization } = getWorkspaceControllers()
  onMount(() => {
    const apply = () => organization.onOpenHome(preferences.store.getState().saved?.navigation.home ?? 'CHAT')
    const stop = preferences.store.subscribe(apply)
    apply()
    return stop
  })
</script>
