<script lang="ts">
  import { SidebarItem } from '../../components/sidebar'
  import type { OrganizationDestination } from './types'
  import { untrack } from 'svelte'
  import type { Content } from '../../components/shared'
  import { readStore } from '../page-state.svelte'
  import { getOrganizationController } from '../../context'

  let {
    item,
    icon,
  }: {
    item: OrganizationDestination
    icon: Content
  } = $props()
  const __organization = getOrganizationController()
  const controller = untrack(() => getOrganizationController())
  let selected = $derived(readStore(__organization.store, (state) => state.selectedDestination === item.id))
</script>

<SidebarItem
  label={item.name}
  href={item.href}
  {icon}
  {selected}
  preload={item.href.includes('/providers/') || item.href.endsWith('/settings/members') || item.href.endsWith('/models')}
  onNavigate={() => controller.onOpenDestination(item.id)}
></SidebarItem>
