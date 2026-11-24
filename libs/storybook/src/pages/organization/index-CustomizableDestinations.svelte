<script lang="ts">
  import { LoadingFallback } from '../../components/loading'
  import { usePermission } from '../../components/permission'

  import { SidebarLoading } from '../../components/sidebar'

  import { LoadedDestinations } from './index'

  const checks = usePermission(() => ({ actions: ['provider.read', 'model.read', 'member.read'] }))
</script>

<LoadingFallback pending={checks.pending}>
  {#snippet fallback()}<SidebarLoading rows={10}></SidebarLoading>{/snippet}<LoadedDestinations></LoadedDestinations>
</LoadingFallback>
