import type { ChatState } from './types'
export { createChatStore } from './store'
export type { ChatController, ChatState } from './types'
export const useController = getChatController
export function useChat<T>(selector: (state: ChatState) => T, _names: readonly string[] = []) {
  return selector(getChatController().store.getState())
}
export { default as ChatPage } from './index-ChatPage.svelte'
export { default as ChatInput } from './index-ChatInput.svelte'
export { default as ChatSubmit } from './index-ChatSubmit.svelte'

import { getChatController } from '../../../context'
