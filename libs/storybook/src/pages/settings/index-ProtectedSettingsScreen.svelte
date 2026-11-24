<script lang="ts">
  import { PermissionRequest } from '../../components/permission'
  import type { PermissionAction } from '../../permission'

  import { Empty, SettingsContent, SettingsDetailLoading } from './shared'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { SettingsScreen } from './index'

  const __settings = getSettingsController()
  let page = $derived(readStore(__settings.store, (state) => state.location.page))
  let action: PermissionAction | undefined = $derived(
    (
      {
        organization: 'organization.settings.read',
        'organization-onboarding': 'organization.onboarding.read',
        members: 'member.read',
        role: 'role.read',
        'new-role': 'role.create',
        group: 'group.read',
        'new-group': 'group.create',
      } as Partial<Record<string, PermissionAction>>
    )[page],
  )
  let pendingTitle = $derived(
    (
      {
        organization: 'Organization',
        'organization-onboarding': 'Onboarding',
        members: 'Members',
        role: 'Role',
        'new-role': 'New role',
        group: 'Group',
        'new-group': 'New group',
      } as Record<string, string>
    )[page] ?? 'Settings',
  )
</script>

{#if !action}<SettingsScreen></SettingsScreen>{:else}<PermissionRequest actions={[action]}>
    {#snippet pending()}<SettingsDetailLoading title={pendingTitle} sections={[pendingTitle]} wide={page === 'members'}
      ></SettingsDetailLoading>{/snippet}{#snippet children(allowed)}{#if allowed}<SettingsScreen
        ></SettingsScreen>{:else}<SettingsContent>
          <Empty title="You do not have access">Return to an available page using the sidebar.</Empty>
        </SettingsContent>{/if}{/snippet}
  </PermissionRequest>{/if}
