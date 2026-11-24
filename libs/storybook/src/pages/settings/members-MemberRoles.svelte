<script lang="ts">
  import { getCollectionRow } from '../../context'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'

  import { Button } from '../../components/buttons'

  import type { SettingsMember } from './types'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { MemberRolePicker } from './members'

  let {
    member,
  }: {
    member: SettingsMember
  } = $props()
  const __settings = getSettingsController()
  const row = getCollectionRow()
  let initiallyOpen = $state(false)
  let activated = $state(untrack(() => row?.forms.has(`${row.key}:role`) ?? false))
  let current = $derived(readStore(__settings.store, (state) => state.currentMemberId))
  let owner = $derived(readStore(__settings.store, (state) => state.actorIsOwner))
  let owners = $derived(readStore(__settings.store, (state) => state.ownerMemberIds ?? []))
</script>

{#if member.id === current || (member.role?.owner && (!owner || owners.length <= 1))}<span
    class="line-clamp-2 whitespace-normal wrap-break-word text-left text-xs"
  >
    {member.role?.name ?? 'Unavailable'}
  </span>{:else}{#if activated}<MemberRolePicker {member} {initiallyOpen}></MemberRolePicker>{:else}<Button
      variant="neutral"
      aria-label={`Roles for ${member.name}`}
      aria-haspopup="listbox"
      class="h-auto min-h-6 w-fit max-w-full justify-between rounded-lg px-2 py-0.5 text-left text-xs"
      onclick={() => {
        initiallyOpen = true
        activated = true
      }}
    >
      <span class="line-clamp-2 min-w-0 flex-1 whitespace-normal text-left">
        {member.role?.name ?? 'Unavailable'}
      </span><CaretDownIcon size={10} aria-hidden="true"></CaretDownIcon>
    </Button>{/if}{/if}
