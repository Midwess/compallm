<script lang="ts">
  import { PermissionRequest, usePermissionChecks } from '../../../components/permission'

  import { FormError } from '../../../components/form'
  import { DisabledTooltip } from '../../../components/feedback'

  import { Button } from '../../../components/buttons'
  import TrashIcon from 'phosphor-svelte/lib/TrashIcon'
  import { Select } from '../../../components/dropdowns'

  import { SectionSurface } from '../../../components/surfaces'
  import { SettingsSectionTitle } from '../../../components/settings'

  import { Indicator } from '../shared'
  import { untrack } from 'svelte'
  import { readStore, toStyle } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController())
  const group = untrack(() => getSettingsController().pages.group)
  let draft = $derived(
    readStore(__settings.pages.group.store, (s) => (s.draft ? { id: s.draft.id, roleIds: s.draft.roleIds } : null), [
      'draft.id',
      'draft.roleIds',
    ]),
  )
  let fallbackRoles = $derived(readStore(__settings.store, (s) => s.roles))
  let roleOptions = $derived(readStore(__settings.pages.group.store, (s) => s.roles))
  let readonly = $derived(readStore(__settings.store, (s) => s.readonly))
  let busy = $derived(readStore(__settings.pages.group.store, (s) => s.busy))
  let roles = $derived((roleOptions ?? fallbackRoles).filter((role) => !role.owner))
  const optionPermissionsState = usePermissionChecks(() =>
    draft?.id
      ? roles.map((role) => ({
          action: draft.roleIds.includes(role.id) ? ('group.role.unassign' as const) : ('group.role.assign' as const),
          resourceType: 'Group',
          resourceId: draft.id,
          roleId: role.id,
        }))
      : [],
  )
  let optionPermissions = $derived(optionPermissionsState.current)
</script>

{#if !draft}{:else}{@const selected = roles.filter((role) => draft.roleIds.includes(role.id))}{@const missing =
    draft.roleIds.filter((id) => !roles.some((role) => role.id === id))}
  <section class="flex min-w-0 flex-col gap-4" aria-label="Assigned roles">
    <div class="flex items-center justify-between gap-2 px-4">
      <SettingsSectionTitle>Assigned roles</SettingsSectionTitle><DisabledTooltip
        disabled={readonly || busy}
        reason={readonly ? 'This group is read-only.' : 'Wait for the current update to finish.'}
      >
        <Select.Root
          value=""
          disabled={readonly || busy}
          onValueChange={(roleId) => {
            if (roleId === 'create-role') c.navigate('new-role')
            else if (roleId) group.store.form.change('draft.roleIds', [...new Set([...draft.roleIds, roleId])])
          }}
        >
          <Select.Trigger aria-label="Add role" class="h-6 w-auto gap-2 rounded-full px-2 text-xs">
            <Select.Value placeholder="Add role"></Select.Value>
          </Select.Trigger><Select.Content aria-label="Add role" class="w-64">
            {#each roles as role, index (index)}{#if role.owner}{:else}<Select.Item
                  value={role.id}
                  disabled={draft.roleIds.includes(role.id) || optionPermissions[index] !== 'allowed'}
                >
                  <span
                    aria-hidden="true"
                    class="mr-1 size-1.5 shrink-0 rounded-full"
                    style={toStyle({ backgroundColor: `color-mix(in oklab, ${role.color} 70%, var(--foreground))` })}
                  ></span><span class="min-w-0">
                    <span class="block truncate">{role.name}</span>{#if role.description}<span
                        class="block truncate text-xs text-muted-foreground"
                      >
                        {role.description}
                      </span>{:else}{/if}
                  </span>
                </Select.Item>{/if}{/each}{#if !roles.length}<Select.Item value="create-role">
                Create role
              </Select.Item>{:else}{/if}
          </Select.Content>
        </Select.Root>
      </DisabledTooltip><FormError name="draft.roleIds"></FormError>
    </div>
    <SectionSurface class="p-0">
      {#if draft.roleIds.length}<ul aria-label="Roles assigned to this group">
          {#each missing as id (id)}<li class="px-4 py-3 text-xs text-muted-foreground">
              Role unavailable: {id}
            </li>{/each}{#each selected as role (role.id)}<li
              class="group/assigned-role flex min-h-14 min-w-0 items-center gap-3
                border-b border-divider px-4 py-3 last:border-0"
            >
              <Indicator color={role.color}></Indicator>
              <div class="flex min-w-0 flex-1 items-baseline gap-3">
                <span class="line-clamp-2 min-w-0 max-w-60 whitespace-normal text-left text-xsm">
                  {role.name}
                </span>{#if role.description}<span
                    class="min-w-0 max-w-80 flex-1 whitespace-normal wrap-break-word
                      text-xs leading-4.5 text-muted-foreground"
                  >
                    {role.description}
                  </span>{:else}{/if}
              </div>
              <DisabledTooltip
                disabled={readonly || busy}
                reason={readonly ? 'This group is read-only.' : 'Wait for the current update to finish.'}
              >
                <PermissionRequest
                  actions={['group.role.unassign']}
                  resource={{ type: 'Group', id: draft.id }}
                  context={{ roleId: role.id }}
                >
                  {#snippet children(allowed)}<DisabledTooltip
                      disabled={!allowed}
                      reason="You do not have permission to perform this action."
                    >
                      <Button
                        variant="ghost"
                        aria-label={`Remove ${role.name} from group`}
                        class="pointer-events-none size-6 shrink-0 rounded-full p-0 text-sm opacity-0
                    group-hover/assigned-role:pointer-events-auto group-hover/assigned-role:opacity-100
                    group-focus-within/assigned-role:pointer-events-auto group-focus-within/assigned-role:opacity-100"
                        disabled={!allowed || readonly || busy}
                        onclick={() =>
                          group.store.form.change(
                            'draft.roleIds',
                            draft.roleIds.filter((id) => id !== role.id),
                          )}
                      >
                        <TrashIcon aria-hidden="true" class="size-3.5"></TrashIcon>
                      </Button>
                    </DisabledTooltip>{/snippet}
                </PermissionRequest>
              </DisabledTooltip>
            </li>{/each}
        </ul>{:else}<p class="px-4 py-5 text-xs text-muted-foreground">No roles assigned.</p>{/if}
    </SectionSurface>
  </section>{/if}
