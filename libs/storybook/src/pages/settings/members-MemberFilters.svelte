<script lang="ts">
  import { PermissionRequest } from '../../components/permission'

  import FunnelIcon from 'phosphor-svelte/lib/FunnelIcon'

  import { Popup } from '../../components/popup'

  import { untrack } from 'svelte'
  import { readStore } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  let roles = $derived(readStore(__settings.store, (s) => s.roles))
  let selected = $derived(readStore(__settings.store, (s) => s.filters.memberRole ?? 'all'))
</script>

<PermissionRequest actions={['role.read', 'group.read']}>
  {#snippet children(allowed)}<Popup.Root>
      <Popup.IconTrigger
        aria-label="Filter members"
        title="Filter members"
        class={selected !== 'all' ? 'border-primary text-primary' : undefined}
      >
        <FunnelIcon class="size-3.5" aria-hidden="true"></FunnelIcon>
      </Popup.IconTrigger><Popup.Content aria-label="Member filters">
        {#if allowed}<Popup.Sub>
            <Popup.SubTrigger>Role</Popup.SubTrigger><Popup.SubContent aria-label="Filter by role">
              <Popup.RadioGroup
                value={selected}
                onValueChange={(value: string) => c.pages.search.setFilter('memberRole', value)}
              >
                <Popup.RadioItem value="all">All roles</Popup.RadioItem>{#each roles as role (role.id)}<Popup.RadioItem
                    value={role.id}
                  >
                    {role.name}
                  </Popup.RadioItem>{/each}
              </Popup.RadioGroup>
            </Popup.SubContent>
          </Popup.Sub>{:else}<p class="px-2 py-2 text-xs text-muted-foreground">
            You do not have permission to filter by role.
          </p>{/if}<Popup.Separator></Popup.Separator><Popup.Item
          onSelect={() => c.pages.search.setFilter('memberRole', 'all')}
        >
          Clear all filters
        </Popup.Item>
      </Popup.Content>
    </Popup.Root>{/snippet}
</PermissionRequest>
