import { createFormStore } from '../../../components/form'
import type { CreateProviderState } from './types'

export function createCreateProviderStore(initial: Partial<CreateProviderState> = {}) {
  return createFormStore<CreateProviderState>(
    {
      navigationRequest: null,
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
      headers: [],
      avatarUrl: null,
      avatarFile: null,
      modelCapabilityFilter: [],
      modelStatusFilter: 'all',
      modelProtocolFilter: 'all',
      providerProtocolFilter: 'all',
      modelDraft: null,
      editingModelId: null,
      test: { status: 'idle' },
      submitting: false,
      error: null,
      ...initial,
    },
    ['name', 'baseUrl', 'apiKey', 'protocol', 'mode', 'models', 'headers', 'avatarUrl', 'avatarFile', 'modelDraft'],
  )
}
