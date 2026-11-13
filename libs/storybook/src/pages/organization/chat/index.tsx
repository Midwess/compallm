import { Form } from '../../../components/form'
import { MultilineField } from '../../../components/fields'
import { useContext, useEffect } from 'react'
import { useStore } from 'zustand'
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
function useChat<T>(selector: (state: ChatState) => T) {
  return useStore(useController().store, selector)
}

export function ChatPage() {
  return (
    <section aria-label="Create chat" className="relative flex h-full min-h-0 flex-col">
      <PageHeader className="absolute inset-x-0 top-0">
        <Title>New chat</Title>
      </PageHeader>
      <div className="flex min-h-0 flex-1 items-center justify-center overflow-auto px-3 py-6">
        <ChatInput />
      </div>
    </section>
  )
}

function ChatInput() {
  const controller = useController()
  const draft = useChat((state) => state.draft)
  const creating = useChat((state) => state.creating)
  const models = useChat((state) => state.models)
  const modelId = useChat((state) => state.modelId)
  const canSend = draft.trim().length > 0 && models.some((model) => model.value === modelId) && !creating
  const form = controller.store.form
  useEffect(
    () =>
      form.configure({
        mode: 'button',
        command: true,
        validate: (values) =>
          Boolean(values.draft?.trim() && controller.store.getState().models.some((m) => m.value === values.modelId)),
        submit: async (values) => {
          await controller.onCreateChat({ draft: values.draft ?? '', modelId: values.modelId ?? '' })
          return false as const
        },
      }),
    [controller, form],
  )
  return (
    <Form model={form} aria-label="Create chat" className="w-full max-w-205.5">
      <div className="rounded-lg border-[0.5px] border-border bg-composer p-3 focus-within:border-input-border">
        <MultilineField
          name="draft"
          aria-label="Message"
          disabled={creating}
          placeholder="Start a conversation…"
          className={`
  block field-sizing-content min-h-18 max-h-64 w-full resize-none overflow-y-auto bg-transparent px-1.5 text-xs leading-6
  placeholder:text-muted focus:outline-none disabled:opacity-50
`}
        />
        <div className="flex items-center gap-3">
          <Select
            aria-label="Chat model"
            options={models}
            name="modelId"
            className="w-35 [&>button>span]:flex-1 [&>button>span]:text-center"
            disabled={creating || !models.length}
          />
          <ToolbarAction label="Attach file" onClick={controller.onAttach} disabled={creating} className="ml-auto">
            <PlusIcon size={14} aria-hidden="true" />
          </ToolbarAction>
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
        </div>
      </div>
    </Form>
  )
}
