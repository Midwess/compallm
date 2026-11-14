import { PermissionRequest } from '../../../components/permission'
import { ChatCreateInput } from '@compallm/contract/validation'
import { FormError } from '../../../components/form'
import { Form, contractResult } from '../../../components/form'
import { MultilineField } from '../../../components/fields'
import { useContext, useEffect } from 'react'
import { useFormStore } from '../../../components/form'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ArrowUpIcon } from '@phosphor-icons/react/dist/csr/ArrowUp'
import { ChatControllerContext } from '../../../context'
import { Button } from '../../../components/buttons'
import { ToolbarAction } from '../../../components/navigation'
import { Select } from '../../../components/dropdowns'
import { PageHeader, Title } from '../../../components/page-header'
import type { ChatState } from './types'

export { createChatStore } from './store'
export type { ChatController, ChatState } from './types'

function useController() {
  const controller = useContext(ChatControllerContext)
  if (!controller) throw new Error('ChatPage requires a ChatControllerContext value.')
  return controller
}
function useChat<T>(selector: (state: ChatState) => T, names: readonly string[] = []) {
  return useFormStore(useController().store, selector, names)
}

export function ChatPage() {
  return (
    <PermissionRequest actions={['MODEL_READ']} restricted="fallback">
      <section aria-label="Create chat" className="relative flex h-full min-h-0 flex-col">
        <PageHeader className="absolute inset-x-0 top-0">
          <Title>New chat</Title>
        </PageHeader>
        <div className="flex min-h-0 flex-1 items-center justify-center overflow-auto px-3 py-6">
          <ChatInput />
        </div>
      </section>
    </PermissionRequest>
  )
}

function ChatInput() {
  const controller = useController()
  const creating = useChat((state) => state.creating)
  const models = useChat((state) => state.models)
  const form = controller.store.form
  useEffect(
    () =>
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
    [controller, form],
  )
  return (
    <Form.Root model={form} aria-label="Create chat" className="w-full max-w-205.5">
      <div className="rounded-lg border-(length:--border-hairline) border-border bg-composer p-3 focus-within:border-input">
        <MultilineField
          name="draft"
          aria-label="Message"
          disabled={creating}
          placeholder="Start a conversation…"
          className={`
  block field-sizing-content min-h-18 max-h-64 w-full resize-none overflow-y-auto bg-transparent px-1.5 text-xs leading-6
  placeholder:text-muted-foreground focus:outline-none disabled:opacity-50
`}
        />
        <FormError name="draft" />
        <div className="flex items-center gap-3">
          <Select.Control
            aria-label="Chat model"
            options={models}
            name="modelId"
            className="w-35 [&>button>span]:flex-1 [&>button>span]:text-center"
            disabled={creating || !models.length}
          />
          <FormError name="modelId" />
          <ToolbarAction label="Attach file" onClick={controller.onAttach} disabled={creating} className="ml-auto">
            <PlusIcon size={14} aria-hidden="true" />
          </ToolbarAction>
          <ChatSubmit />
        </div>
      </div>
    </Form.Root>
  )
}

function ChatSubmit() {
  const draft = useChat((state) => state.draft, ['draft'])
  const modelId = useChat((state) => state.modelId, ['modelId'])
  const models = useChat((state) => state.models)
  const creating = useChat((state) => state.creating)
  const canSend =
    ChatCreateInput.safeParse({ message: draft, modelId }).success &&
    models.some((model) => model.value === modelId) &&
    !creating
  return (
    <PermissionRequest
      actions={['MODEL_USE']}
      resource={{ type: 'MODEL', id: modelId || undefined }}
      enabled={!!modelId}
      restricted="disabled"
      asChild
    >
      <Button
        type="submit"
        variant="primary"
        aria-label="Create chat"
        disabled={!canSend}
        loading={creating}
        className="size-6 p-0"
      >
        {!creating ? <ArrowUpIcon size={14} aria-hidden="true" /> : null}
      </Button>
    </PermissionRequest>
  )
}
