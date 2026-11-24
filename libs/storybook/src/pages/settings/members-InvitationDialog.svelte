<script lang="ts">
  import UserPlusIcon from 'phosphor-svelte/lib/UserPlusIcon'

  import { PermissionRequest } from '../../components/permission'

  import { Dialog } from '../../components/dialog'

  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { InviteMember } from './members'

  const __settings = getSettingsController()
  let open = $derived(readStore(__settings.store, (s) => s.invitationsOpen ?? false))
</script>

<Dialog.Root defaultOpen={open}>
  <Dialog.Trigger variant="neutral" class="ml-auto h-8 shrink-0 gap-1.5 px-3">
    <UserPlusIcon class="size-3.5" aria-hidden="true"></UserPlusIcon>Invite
  </Dialog.Trigger><Dialog.Content class="top-[22vh] bottom-auto max-h-[76dvh] w-140 rounded-xl bg-inset md:left-61">
    <Dialog.Header class="h-14 border-0">
      <Dialog.Title class="flex items-center gap-2 text-ui-15 font-medium">Send invitations</Dialog.Title><Dialog.Close
      ></Dialog.Close>
    </Dialog.Header><Dialog.Body class="pt-3 pb-4">
      <PermissionRequest actions={['member.invite']}>
        {#snippet children(allowed)}{#if allowed}<InviteMember></InviteMember>{:else}{/if}{/snippet}
      </PermissionRequest>
    </Dialog.Body>
  </Dialog.Content>
</Dialog.Root>
