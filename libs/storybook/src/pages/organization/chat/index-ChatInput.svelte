<script lang="ts">
  import { getChatController } from '../../../context'
  import { readStore } from '../../page-state.svelte'
  const chatController = getChatController()

  import { ChatCreateInput } from '@compallm/contract/validation'
  import { FormError } from '../../../components/form'
  import { Form, contractResult } from '../../../components/form'
  import { MultilineField } from '../../../components/fields'

  import PlusIcon from 'phosphor-svelte/lib/PlusIcon'

  import { ToolbarAction } from '../../../components/navigation'
  import { Select } from '../../../components/dropdowns'

  import { untrack } from 'svelte'

  import { ChatSubmit } from './index'

  const controller = untrack(() => getChatController())
  let creating = $derived(readStore(chatController.store, (state) => state.creating))
  let models = $derived(readStore(chatController.store, (state) => state.models))
  let form = $derived(controller.store.form)
  $effect(() => {
    void [controller, form]
    return untrack(() =>
      form.configure({
        mode: 'button',
        command: true,
        parse: (values) => {
          const result = ChatCreateInput.safeParse({ message: values.draft, modelId: values.modelId })
          const parsed = contractResult(
            result,
            result.success ? { ...values, draft: result.data.message, modelId: result.data.modelId } : values,
          )
          return parsed.issues
            ? {
                issues: parsed.issues.map((issue) => ({ ...issue, path: [issue.path[0] === 'message' ? 'draft' : 'modelId'] })),
              }
            : parsed
        },
        validate: (values) => controller.store.getState().models.some((m) => m.value === values.modelId),
        submit: async (values) => {
          await controller.onCreateChat({ draft: values.draft ?? '', modelId: values.modelId ?? '' })
          return false as const
        },
      }),
    )
  })
</script>

<Form.Root model={form} aria-label="Create chat" class="w-full max-w-205.5">
  <div class="rounded-lg border-(length:--border-hairline) border-border bg-composer p-3 focus-within:border-input">
    <MultilineField
      name="draft"
      aria-label="Message"
      disabled={creating}
      placeholder="Start a conversation…"
      class={`
  block field-sizing-content min-h-18 max-h-64 w-full resize-none overflow-y-auto bg-transparent px-1.5 text-xs leading-6
  placeholder:text-muted-foreground focus:outline-none disabled:opacity-50
`}
    ></MultilineField><FormError name="draft"></FormError>
    <div class="flex items-center gap-3">
      <Select.Control
        aria-label="Chat model"
        options={models}
        name="modelId"
        class="w-35 [&>button>span]:flex-1 [&>button>span]:text-center"
        disabled={creating || !models.length}
      ></Select.Control><FormError name="modelId"></FormError><ToolbarAction
        label="Attach file"
        onclick={controller.onAttach}
        disabled={creating}
        class="ml-auto"
      >
        <PlusIcon size={14} aria-hidden="true"></PlusIcon>
      </ToolbarAction><ChatSubmit></ChatSubmit>
    </div>
  </div>
</Form.Root>
