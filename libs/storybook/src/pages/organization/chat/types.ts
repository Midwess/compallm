import type { StoreApi } from 'zustand/vanilla'

export interface ChatState {
  draft: string
  modelId: string
  models: { value: string; label: string }[]
  creating: boolean
}

export interface ChatController {
  readonly store: Pick<StoreApi<ChatState>, 'getState' | 'getInitialState' | 'subscribe'>
  onDraftChanged(value: string): void
  onModelChanged(id: string): void
  onCreateChat(): void
  onAttach(): void
}
