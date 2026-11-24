<script lang="ts">
  import { SidebarHeader } from '../../components/sidebar'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getOrganizationController } from '../../context'

  const __organization = getOrganizationController()
  const controller = untrack(() => getOrganizationController())
  let organizations = $derived(readStore(__organization.store, (state) => state.organizations))
  let organizationId = $derived(readStore(__organization.store, (state) => state.organizationId))
</script>

<SidebarHeader
  workspace={organizations.find((item) => item.id === organizationId)?.name ?? 'Organization'}
  {organizations}
  selectedOrganizationId={organizationId}
  onOrganizationChange={controller.onSelectOrganization}
  onWorkspaceClick={() => {}}
  onSearch={controller.onSearch}
  onSettings={controller.onOpenSettings}
  onSignOut={controller.onSignOut}
></SidebarHeader>
