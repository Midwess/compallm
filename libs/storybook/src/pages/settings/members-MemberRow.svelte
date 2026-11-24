<script lang="ts">
  import ArrowDownLeftIcon from 'phosphor-svelte/lib/ArrowDownLeftIcon'
  import { Tooltip } from '../../components/feedback'

  import { PermissionRequest } from '../../components/permission'

  import { Table } from '../../components/table'

  import type { SettingsMember } from './types'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { DirectoryDate, MemberRoles, MemberGroups, MemberActions, MemberEmail, MemberAvatar } from './members'

  let {
    member,
  }: {
    member: SettingsMember
  } = $props()
  const __settings = getSettingsController()
  let currentMemberId = $derived(readStore(__settings.store, (s) => s.currentMemberId))
</script>

<Table.Row class="h-14 border-0">
  <Table.Cell column="name" class="pl-0">
    <div class="flex items-center gap-2.5 py-1">
      <MemberAvatar {member}></MemberAvatar><span class="relative min-w-0 pr-3 text-xsm font-medium">
        <span class="block truncate">{member.name}</span>{#if member.id === currentMemberId}<Tooltip.Control
            content="This is you"
          >
            <button
              type="button"
              aria-label="This is you"
              class="absolute -top-1 right-0 rounded text-foreground focus-visible:outline-ring"
            >
              <ArrowDownLeftIcon weight="bold" class="size-2.5" aria-hidden="true"></ArrowDownLeftIcon>
            </button>
          </Tooltip.Control>{:else}{/if}
      </span>
    </div>
  </Table.Cell><Table.Cell column="email" class="text-muted-foreground">
    <MemberEmail email={member.email} name={member.name}></MemberEmail>
  </Table.Cell><Table.Cell column="role">
    <PermissionRequest actions={['role.read']}>
      {#snippet children(allowed)}{#if allowed}<MemberRoles {member}></MemberRoles>{:else}{/if}{/snippet}
    </PermissionRequest>
  </Table.Cell><Table.Cell column="groups"><MemberGroups {member}></MemberGroups></Table.Cell><Table.Cell column="joined">
    <DirectoryDate value={member.joinedAt}></DirectoryDate>
  </Table.Cell><Table.Cell column="actions" class="px-0"><MemberActions {member}></MemberActions></Table.Cell>
</Table.Row>
