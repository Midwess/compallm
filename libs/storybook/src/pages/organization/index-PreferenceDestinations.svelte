<script lang="ts">
  import { LoadingFallback } from '../../components/loading'
  import { usePermission } from '../../components/permission'

  import { SidebarLoading } from '../../components/sidebar'

  import { readStore } from '../page-state.svelte'
  import { LoadedDestinations } from './index'

  let {
    controller,
  }: {
    controller: import('../settings/preferences/user/controller').UserPreferenceController
  } = $props()

  const checks = usePermission(() => ({ actions: ['provider.read', 'model.read', 'member.read'] }))
  let ready = $derived(
    readStore(controller.store, (state) => !!state.saved || state.status === 'error' || state.status === 'denied'),
  )
</script>

<LoadingFallback pending={!ready || checks.pending}>
  {#snippet fallback()}<SidebarLoading rows={10}></SidebarLoading>{/snippet}<LoadedDestinations></LoadedDestinations>
</LoadingFallback>
