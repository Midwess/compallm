import { createStore } from 'zustand/vanilla'
import type { ChatState } from './types'

export function createChatStore(initial: Partial<ChatState> = {}) {
  return createStore<ChatState>()(() => ({
    draft: '',
    modelId: '',
    models: [],
    creating: false,
    ...initial,
  }))
}
