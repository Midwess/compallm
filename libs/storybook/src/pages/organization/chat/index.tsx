import { useContext } from 'react'
import { useStore } from 'zustand'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ArrowUpIcon } from '@phosphor-icons/react/dist/csr/ArrowUp'
import { ChatControllerContext } from '../../../context'
import { Button } from '../../../components/buttons'
import { ToolbarAction } from '../../../components/navigation'
import { Select } from '../../../components/dropdowns'
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
      <header className="absolute inset-x-0 top-0 px-8 pt-4">
        <h1 className="text-2xl leading-8 font-medium">New chat</h1>
      </header>
      <div className="flex min-h-0 flex-1 items-center justify-center overflow-auto px-6 py-6">
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
  return (
    <form
      aria-label="Create chat"
      className="w-full max-w-[821px]"
      onSubmit={(event) => {
        event.preventDefault()
        if (canSend) controller.onCreateChat()
      }}
    >
      <div className="rounded-lg border-[0.5px] border-border bg-composer p-3 focus-within:border-input-border">
        <textarea
          aria-label="Message"
          value={draft}
          disabled={creating}
          onChange={(event) => controller.onDraftChanged(event.target.value)}
          placeholder="Start a conversation…"
          className={`
  block h-[72px] w-full resize-none bg-transparent px-1.5 text-xs leading-6
  placeholder:text-muted focus:outline-none disabled:opacity-50
`}
          onKeyDown={(event) => {
            if (event.key === 'Enter' && !event.shiftKey && !event.nativeEvent.isComposing) {
              event.preventDefault()
              if (canSend) controller.onCreateChat()
            }
          }}
        />
        <div className="flex items-center gap-3">
          <Select
            aria-label="Chat model"
            options={models}
            value={modelId}
            onValueChange={controller.onModelChanged}
            className="w-[140px]"
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
    </form>
  )
}
