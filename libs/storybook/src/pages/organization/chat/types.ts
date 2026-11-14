import type { FormStore } from '../../../components/form'

export interface ChatState {
  draft: string
  modelId: string
  models: { value: string; label: string; enabled?: boolean }[]
  creating: boolean
}

export interface ChatController {
  readonly store: FormStore<ChatState>
  onCreateChat(values: Pick<ChatState, 'draft' | 'modelId'>): void | false | Promise<void | false>
  onAttach(): void
}
