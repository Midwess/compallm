import { createStore } from 'zustand/vanilla'
import type { CreateProviderState } from './types'

export function createCreateProviderStore(initial: Partial<CreateProviderState> = {}) {
  return createStore<CreateProviderState>()(() => ({
    intent: 'create',
    savedProvider: null,
    step: 'choose',
    providers: [],
    providerQuery: '',
    selectedProviderId: null,
    name: '',
    baseUrl: '',
    apiKey: '',
    protocol: 'openai_compatible',
    mode: 'chat_completions',
    endpointUrl: '',
    models: [],
    modelQuery: '',
    modelDraft: null,
    editingModelId: null,
    test: { status: 'idle' },
    submitting: false,
    error: null,
    ...initial,
  }))
}
