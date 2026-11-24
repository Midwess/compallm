<script lang="ts">
  import { Form } from '../../../components/form'
  import { RoleUpdateInput } from '@compallm/contract/validation'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getSettingsController } from '../../../context'
  import { RoleEditorContent } from './index'

  const __settings = getSettingsController()
  const c = untrack(() => getSettingsController().pages.role)
  let id = $derived(readStore(__settings.pages.role.store, (state) => state.draft?.id, ['draft.id']))
  let form = $derived(c.store.form)
  const settings = untrack(() => getSettingsController())
  $effect(() => {
    void [c, form, id, settings]
    return untrack(() => {
      if (!c.store.getState().draft) return
      form.reset(form.values)
      form.configure({
        mode: id ? 'blur' : 'button',
        names: ['draft.name', 'draft.description', 'draft.source'],
        command: !id,
        fields: {
          'draft.name': { group: 'metadata' },
          'draft.description': { group: 'metadata' },
          'draft.source': { group: 'policy' },
        },
        changed: (name, value) => {
          if (name === 'draft.source') c.setPolicySource(String(value))
        },
        parse: (values, names) => {
          const draft = values.draft
          if (!draft || draft.builtin) return { issues: [] }
          const patch: Record<string, unknown> = {}
          for (const name of names) {
            if (name === 'draft.name') patch.name = draft.name
            if (name === 'draft.description') patch.description = draft.description
            if (name === 'draft.source') patch.cedarSource = draft.source
          }
          const result = RoleUpdateInput.safeParse(patch)
          if (!result.success)
            return {
              issues: result.error.issues.map((issue) => ({
                ...issue,
                path: ['draft', issue.path[0] === 'cedarSource' ? 'source' : issue.path[0]],
              })),
            }
          return {
            values: {
              ...values,
              draft: {
                ...draft,
                ...('name' in result.data ? { name: result.data.name! } : {}),
                ...('description' in result.data ? { description: result.data.description! } : {}),
                ...('cedarSource' in result.data ? { source: result.data.cedarSource! } : {}),
              },
            },
          }
        },
        submit: async (values, _patch, signal, names) => {
          const draft = values.draft
          if (!draft) return false
          if (!draft.id) {
            c.saveRole()
            return false
          }
          let result = { ...draft }
          const metadata: Partial<Pick<typeof draft, 'name' | 'description'>> = {}
          if (names.includes('draft.name')) metadata.name = draft.name
          if (names.includes('draft.description')) metadata.description = draft.description
          if (Object.keys(metadata).length) {
            if (!c.updateMetadata) return false
            result = { ...result, ...(await c.updateMetadata(metadata, signal)) }
          }
          if (names.includes('draft.source')) {
            if ((await c.commitPolicySource?.(draft.source)) === false) return false
            if (c.store.getState().draft?.diagnostics.length) return false
          }
          return { draft: result }
        },
        error: (error, retry) => settings.apiError(error, retry),
      })
    })
  })
</script>

<Form.Root model={form}><RoleEditorContent></RoleEditorContent></Form.Root>
