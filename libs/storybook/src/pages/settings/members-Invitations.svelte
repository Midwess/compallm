<script lang="ts">
  import DotsThreeIcon from 'phosphor-svelte/lib/DotsThreeIcon'

  import { PermissionRequest } from '../../components/permission'

  import { Table } from '../../components/table'

  import { Popup } from '../../components/popup'

  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'
  import { DirectoryDate, MemberEmail } from './members'

  const __settings = getSettingsController()
  let open = $state(true)
  const c = untrack(() => getSettingsController())
  let confirmed = $derived(readStore(__settings.store, (s) => s.invitations))
  let invitations = $derived((() => confirmed.filter((invitation) => invitation.status === 'pending'))())
  let loading = $derived(readStore(__settings.store, (s) => s.invitationsLoading))
  let ready = $derived(readStore(__settings.store, (s) => s.invitationsLoaded !== false))
  let more = $derived(readStore(__settings.store, (s) => s.invitationsHasNextPage))
  let failed = $derived(readStore(__settings.store, (s) => s.invitationsLoadError ?? false))
  let busy = $derived(readStore(__settings.store, (s) => s.invitationBusy))
</script>

{#if !invitations.length && ready}{:else}<Table.Category
    initialLoading={!ready}
    variant="directory"
    label="Invited"
    count={ready ? `${invitations.length}${more ? ' shown' : ''}` : undefined}
    columns={6}
    {open}
    onOpenChange={(value) => (open = value)}
    items={invitations}
    getKey={(row) => row.id}
    paging={{
      hasMore: more,
      loading: loading || busy,
      error: failed,
      onLoadMore: () => c.pages.members.loadMoreInvitations?.(),
    }}
  >
    {#snippet children(row)}<Table.Row class="h-14 border-0">
        <Table.Cell column="name" class="pl-0">
          <div class="flex items-center gap-2.5 py-1">
            <span
              class="grid size-6 shrink-0 place-items-center rounded-full border border-dashed
border-border text-ui-11 text-muted-foreground"
            >
              @
            </span><span class="truncate text-xsm font-medium">Invited member</span>
          </div>
        </Table.Cell><Table.Cell column="email" class="text-muted-foreground">
          <MemberEmail email={row.email} name={`invitation ${row.id}`}></MemberEmail>
        </Table.Cell><Table.Cell column="role" class="px-2 text-muted-foreground">Member (Invited)</Table.Cell><Table.Cell
          column="groups"
          class="px-2 text-muted-foreground"
        >
          —
        </Table.Cell><Table.Cell column="joined">
          <DirectoryDate value={row.issuedAt ?? row.expiresAt - 604800}></DirectoryDate>
        </Table.Cell><Table.Cell column="actions" class="px-0">
          <PermissionRequest actions={['member.invite']}>
            {#snippet children(allowed)}{#if allowed && (row.status === 'pending' || row.status === 'expired')}<Popup.Root>
                  <Popup.Trigger aria-label={`Actions for invitation ${row.id}`} class="size-8 p-0">
                    <DotsThreeIcon class="size-4 text-muted-foreground" aria-hidden="true"></DotsThreeIcon>
                  </Popup.Trigger><Popup.Content aria-label="Invitation actions">
                    <Popup.Item onSelect={() => void c.pages.members.resend?.(row.id)}>Resend invitation</Popup.Item><Popup.Item
                      class="text-destructive"
                      onSelect={() => void c.pages.members.revoke?.(row.id)}
                    >
                      Revoke invitation
                    </Popup.Item>
                  </Popup.Content>
                </Popup.Root>{:else}{/if}{/snippet}
          </PermissionRequest>
        </Table.Cell>
      </Table.Row>{/snippet}
  </Table.Category>{/if}
