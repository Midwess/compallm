<script lang="ts">
  import { getCollectionRow } from '../../context'

  import { PermissionRequest } from '../../components/permission'

  import { Button } from '../../components/buttons'

  import type { SettingsMember } from './types'

  import { untrack } from 'svelte'

  import { MemberGroupPicker } from './members'

  let {
    member,
  }: {
    member: SettingsMember
  } = $props()

  const row = getCollectionRow()
  let initiallyOpen = $state(false)
  let activated = $state(untrack(() => row?.forms.has(`${row.key}:groups`) ?? false))
</script>

{#if activated}<MemberGroupPicker {member} {initiallyOpen}></MemberGroupPicker>{:else}{@const groups =
    member.groups ?? []}<PermissionRequest actions={['group.read']}>
    {#snippet children(allowed)}{#if allowed}<Button
          variant="neutral"
          aria-label={`Groups for ${member.name}`}
          aria-haspopup="listbox"
          class="h-auto min-h-6 justify-start rounded-full px-2 py-0.5 text-left text-xs"
          onclick={() => {
            initiallyOpen = true
            activated = true
          }}
        >
          <span class="line-clamp-2 whitespace-normal text-left">
            {groups[0]?.name ?? 'None'}{#if groups.length > 1}{` +${groups.length - 1}`}{:else}{/if}
          </span>
        </Button>{:else}{/if}{/snippet}
  </PermissionRequest>{/if}
