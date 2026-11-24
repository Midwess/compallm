<script lang="ts">
  import { usePermissionChecks } from '../../components/permission'
  import { Form, managedForm } from '../../components/form'

  import { Button } from '../../components/buttons'

  import { Select } from '../../components/dropdowns'
  import { Dialog } from '../../components/dialog'

  import type { PermissionCheck } from '../../permission'
  import type { SettingsMember } from './types'

  import { untrack } from 'svelte'
  import { readStore, readForm, toStyle } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  import ContentValue from '../../components/Content.svelte'

  let {
    member,
    initiallyOpen,
  }: {
    member: SettingsMember
    initiallyOpen: boolean
  } = $props()
  const __settings = getSettingsController()
  let open = $state(untrack(() => initiallyOpen))
  let roles = $derived(readStore(__settings.store, (s) => s.roles))
  let actorIsOwner = $derived(readStore(__settings.store, (s) => s.actorIsOwner))
  let currentMemberId = $derived(readStore(__settings.store, (s) => s.currentMemberId))
  let ownerIds = $derived(readStore(__settings.store, (s) => s.ownerMemberIds ?? []))
  let confirmation: string | null = $state(null)
  let first = $derived(member.role)
  let known = $derived(Boolean(first))
  let label = $derived(first?.name ?? 'Unavailable')
  const c = untrack(() => getSettingsController())
  const checksFor = (id: string): PermissionCheck[] => [
    ...(id
      ? [
          { action: 'member.role.assign' as const, resourceType: 'Member' as const, resourceId: member.id, roleId: id },
          { action: 'role.assign' as const, resourceType: 'Role' as const, resourceId: id },
        ]
      : []),
    ...(first && first.id !== id
      ? [
          { action: 'member.role.unassign' as const, resourceType: 'Member' as const, resourceId: member.id, roleId: first.id },
          { action: 'role.unassign' as const, resourceType: 'Role' as const, resourceId: first.id },
        ]
      : []),
  ]
  let protectedOwner = $derived(first?.owner)
  let options = $derived(roles.filter((r) => !r.owner || actorIsOwner))
  let optionChecks = $derived(open ? options.map((role) => checksFor(role.id)) : [])
  const statusesState = usePermissionChecks(() => optionChecks.flat())
  let statuses = $derived(statusesState.current)
  let allowed = $derived(
    optionChecks.map((checks, index) => {
      const offset = optionChecks.slice(0, index).reduce((total, previous) => total + previous.length, 0)
      return statuses.slice(offset, offset + checks.length).every((status) => status === 'allowed')
    }),
  )
  const form = untrack(() =>
    managedForm(
      { roleId: first?.id ?? '' },
      {
        mode: 'change',
        submit: async (values) => {
          const result = await c.pages.members.setMemberRole(member.id, values.roleId)
          if (result === false) return false
          confirmation = null
          return values
        },
      },
      'role',
    ),
  )
  let selected = $derived(readForm<string>(form, 'roleId'))
  let pending = $derived((() => form.pending)())
  const cancelConfirmation = () => {
    if (pending) return
    form.escape('roleId')
    confirmation = null
  }
</script>

{#if member.id === currentMemberId || (protectedOwner && (!actorIsOwner || ownerIds.length <= 1))}<span
    class="line-clamp-2 whitespace-normal wrap-break-word text-left text-xs"
  >
    <ContentValue value={label} />
  </span>{:else}<Form.Root model={form}>
    <Select.Root
      {open}
      onOpenChange={(value) => (open = value)}
      value={selected}
      disabled={pending || confirmation !== null}
      onValueChange={(value) => {
        if (value === null || value === selected || pending) return
        if (Boolean(protectedOwner) !== Boolean(roles.find((r) => r.id === value)?.owner)) confirmation = value
        else form.change('roleId', value)
      }}
    >
      <Select.Trigger
        aria-label={`Roles for ${member.name}`}
        class="h-auto min-h-6 w-fit max-w-full rounded-lg px-2 py-0.5 text-left text-xs"
      >
        <Select.Value>
          {#if selected === (first?.id ?? '')}<ContentValue value={label} />{:else}{roles.find((r) => r.id === selected)
              ?.name ?? 'Unavailable'}{/if}
        </Select.Value>
      </Select.Trigger>{#if open}<Select.Content aria-label={`Roles for ${member.name}`} class="w-64">
          {#each options as role, index (index)}{@const unavailable =
              !known ||
              !allowed[index] ||
              (protectedOwner &&
                (!actorIsOwner || ownerIds.length <= 1) &&
                role.id !== first?.id)}{#if unavailable}{:else}<Select.Item value={role.id}>
                {#if role.color}<span
                    aria-hidden="true"
                    class="mr-1 size-1.5 shrink-0 rounded-full"
                    style={toStyle({ backgroundColor: `color-mix(in oklab, ${role.color} 70%, var(--foreground))` })}
                  ></span>{:else}{/if}<span class="min-w-0">
                  <span class="block truncate">{role.name}</span>{#if role.description}<span
                      class="block truncate text-xs text-muted-foreground"
                    >
                      {role.description}
                    </span>{:else}{/if}
                </span>
              </Select.Item>{/if}{/each}
        </Select.Content>{:else}{/if}
    </Select.Root><Form.Error name="roleId"></Form.Error>{#if confirmation !== null}<Dialog.Root
        open
        onOpenChange={(open) => {
          if (!open) cancelConfirmation()
        }}
      >
        <Dialog.Content class="w-100">
          <Dialog.Header>
            <Dialog.Title>
              {#if roles.find((r) => r.id === confirmation)?.owner}{`Make ${member.name} an owner?`}{:else}{`Change ${member.name}’s role?`}{/if}
            </Dialog.Title>{#if !pending}<Dialog.Close></Dialog.Close>{:else}{/if}
          </Dialog.Header><Dialog.Footer>
            <Button disabled={pending} onclick={cancelConfirmation}>Cancel</Button><Button
              variant="primary"
              loading={pending}
              onclick={() => {
                form.change('roleId', confirmation)
                void form.submit()
              }}
            >
              Confirm
            </Button>
          </Dialog.Footer>
        </Dialog.Content>
      </Dialog.Root>{:else}{/if}
  </Form.Root>{/if}
