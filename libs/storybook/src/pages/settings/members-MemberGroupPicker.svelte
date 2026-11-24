<script lang="ts">
  import { ChoicePicker } from '../../components/choice-picker'
  import { Searchbox } from '../../components/fields'
  import { PermissionRequest, usePermissionChecks } from '../../components/permission'
  import { Form, managedForm } from '../../components/form'

  import type { SettingsMember } from './types'

  import { untrack } from 'svelte'
  import { readStore, readForm } from '../page-state.svelte'
  import { getSettingsController } from '../../context'

  let {
    member,
    initiallyOpen,
  }: {
    member: SettingsMember
    initiallyOpen: boolean
  } = $props()
  const __settings = getSettingsController()
  let open = $state(untrack(() => initiallyOpen))
  const c = untrack(() => getSettingsController())
  let catalog = $derived(readStore(__settings.store, (s) => s.groups))
  let confirmed = $derived(member.groups ?? catalog.filter((group) => group.memberIds.includes(member.id)))
  let ids = $derived(confirmed.map((group) => group.id))
  let options = $derived([...new Map([...catalog, ...confirmed].map((group) => [group.id, group])).values()])
  const checksState = usePermissionChecks(() =>
    (open ? options : []).map((group) => ({
      action: ids.includes(group.id) ? ('group.member.remove' as const) : ('group.member.add' as const),
      resourceType: 'Group',
      resourceId: group.id,
      memberId: member.id,
    })),
  )
  let checks = $derived(checksState.current)
  const form = untrack(() =>
    managedForm(
      { groupIds: ids },
      {
        mode: 'change',
        submit: async (values) => {
          const result = await c.pages.members.setMemberGroups?.(member.id, values.groupIds)
          return result === false || !result ? false : { groupIds: result }
        },
      },
      'groups',
    ),
  )
  let selected = $derived(readForm<string[]>(form, 'groupIds') ?? [])
  let selectedGroups = $derived(options.filter((group) => selected.includes(group.id)))
  let editable = $derived(Boolean(c.pages.members.setMemberGroups))
  let pickerOptions = $derived(
    options.map((group, index) => ({
      id: group.id,
      label: group.name,
      description: group.description,
      disabled: !editable || checks[index] !== 'allowed',
    })),
  )
</script>

<PermissionRequest actions={['group.read']}>
  {#snippet children(allowed)}{#if allowed}<Form.Root model={form}>
        <ChoicePicker.Root
          {open}
          onOpenChange={(value) => (open = value)}
          options={pickerOptions}
          value={selected}
          onValueChange={(next) => form.change('groupIds', next)}
        >
          <ChoicePicker.Trigger aria-label={`Groups for ${member.name}`}>
            {selectedGroups[0]?.name ?? 'None'}{#if selectedGroups.length > 1}{` +${selectedGroups.length - 1}`}{:else}{/if}
          </ChoicePicker.Trigger>{#if open}<ChoicePicker.Content align="start">
              <div class="p-2 pb-0">
                <Searchbox.Root>
                  <ChoicePicker.Input aria-label={`Search groups for ${member.name}`} placeholder="Search groups…"
                  ></ChoicePicker.Input>
                </Searchbox.Root>
              </div>
              <ChoicePicker.Empty>Empty.</ChoicePicker.Empty><ChoicePicker.List>
                {#snippet children(option)}<ChoicePicker.Item value={option}></ChoicePicker.Item>{/snippet}
              </ChoicePicker.List>
            </ChoicePicker.Content>{:else}{/if}
        </ChoicePicker.Root>
      </Form.Root>{:else}{/if}{/snippet}
</PermissionRequest>
