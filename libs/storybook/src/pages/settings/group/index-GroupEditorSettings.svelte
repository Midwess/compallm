<script lang="ts">
  import { GroupCreateInput, GroupUpdateInput } from '@compallm/contract/validation'

  import { Form, contractResult } from '../../../components/form'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'
  import { GroupEditorContent } from './index'

  const __settings = getSettingsController()
  const group = untrack(() => getSettingsController().pages.group)
  let id = $derived(readStore(__settings.pages.group.store, (state) => state.draft?.id, ['draft.id']))
  let members = $derived(
    readStore(__settings.pages.group.store, (state) => state.members?.map((member) => member.id).join('|')),
  )
  let fallbackMembers = $derived(readStore(__settings.store, (state) => state.members.map((member) => member.id).join('|')))
  let memberIds = $derived(members ?? fallbackMembers)
  let form = $derived(group.store.form)
  const settings = untrack(() => getSettingsController())
  $effect(() => {
    void [group, form, id, settings, memberIds]
    return untrack(() => {
      if (!group.store.getState().draft) return
      form.configure({
        mode: 'blur',
        names: ['draft.name', 'draft.description', 'draft.roleIds', 'draft.memberSelection'],
        changed: (name, value) => {
          if (name === 'draft.roleIds') group.rolesChanged?.(value as string[])
        },
        fields: {
          'draft.name': { group: 'metadata' },
          'draft.description': { group: 'metadata' },
          'draft.roleIds': { mode: 'change', group: 'roles' },
          ...Object.fromEntries(
            memberIds
              .split('|')
              .filter(Boolean)
              .map((id) => [`draft.memberSelection.${id}`, { mode: 'change' as const, group: 'members' }]),
          ),
        },
        validationNames: (names) => (id ? names : ['draft']),
        error: (error, retry) =>
          settings.apiError(error, () => {
            if (group.store.getState().draft?.id === id) retry()
          }),
        parse: (values, names) => {
          const draft = values.draft
          if (id && draft) {
            if (names.includes('draft.roleIds')) {
              const roles = GroupCreateInput.shape.roleIds.safeParse(draft.roleIds)
              if (!roles.success)
                return { issues: roles.error.issues.map((issue) => ({ ...issue, path: ['draft', 'roleIds', ...issue.path] })) }
            }
            const patch: Record<string, unknown> = {}
            if (names.includes('draft.name')) patch.name = draft.name
            if (names.includes('draft.description')) patch.description = draft.description
            const result = GroupUpdateInput.safeParse(patch)
            return contractResult(
              result,
              result.success
                ? {
                    ...values,
                    draft: {
                      ...draft,
                      name: result.data.name ?? draft.name,
                      description: result.data.description ?? draft.description,
                    },
                  }
                : values,
              'draft',
            )
          }
          return { issues: [] }
        },
        submit: async (values, _patch, signal, names) => {
          if (!values.draft?.id) return false
          const result = await group.updateGroup(values.draft, names, signal)
          return result && typeof result === 'object' ? { draft: result } : false
        },
      })
    })
  })
</script>

<Form.Root model={form}><GroupEditorContent></GroupEditorContent></Form.Root>
