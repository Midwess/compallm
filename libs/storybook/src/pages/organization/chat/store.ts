import { createFormStore } from '../../../components/form'
import type { ChatState } from './types'

export function createChatStore(initial: Partial<ChatState> = {}) {
  return createFormStore<ChatState>(
    {
      draft: '',
      modelId: '',
      models: [],
      creating: false,
      ...initial,
    },
    ['draft', 'modelId'],
  )
}
