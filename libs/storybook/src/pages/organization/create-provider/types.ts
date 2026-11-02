import type { StoreApi } from 'zustand/vanilla'

export type EndpointMode = 'chat_completions' | 'responses' | 'messages' | 'video' | 'realtime'
export type ApiProtocol = 'openai_compatible' | 'anthropic' | 'provider_specific'
export type ThinkingLevel = 'low' | 'medium' | 'high' | 'xhigh' | 'max'
export interface ProviderPreset {
  id: string
  name: string
  protocol: ApiProtocol
  baseUrl: string
}
export interface ModelDraft {
  id: string
  name: string
  maxContextTokens: string
  thinkingLevels: ThinkingLevel[]
  inputPrice: string
  outputPrice: string
  cachedPrice: string
}
export interface ProviderModel extends ModelDraft {
  enabled: boolean
  modes: EndpointMode[]
  description?: string
}
export type ConnectionTest = { status: 'idle' | 'testing' } | { status: 'passed' } | { status: 'failed'; message: string }
export interface ProviderSnapshot {
  name: string
  baseUrl: string
  protocol: ApiProtocol
  mode: EndpointMode
  models: ProviderModel[]
}
export interface CreateProviderState {
  intent: 'create' | 'edit' | 'clone'
  savedProvider: ProviderSnapshot | null

  step: 'choose' | 'configure'
  providers: ProviderPreset[]
  providerQuery: string
  selectedProviderId: string | null
  name: string
  baseUrl: string
  apiKey: string
  protocol: ApiProtocol
  mode: EndpointMode
  endpointUrl: string
  models: ProviderModel[]
  modelQuery: string
  modelDraft: ModelDraft | null
  editingModelId: string | null
  test: ConnectionTest
  submitting: boolean
  error: string | null
}
export interface CreateProviderController {
  readonly store: Pick<StoreApi<CreateProviderState>, 'getState' | 'getInitialState' | 'subscribe'>
  onProviderQueryChanged(query: string): void
  onSelectProvider(id: string): void
  onUseCustomProvider(): void
  onChooseProvider(): void
  onConnectionChanged(field: 'name' | 'baseUrl' | 'apiKey', value: string): void
  onProtocolChanged(protocol: ApiProtocol): void
  onModeChanged(mode: EndpointMode): void
  onModelQueryChanged(query: string): void
  onModelEnabledChanged(id: string, enabled: boolean): void
  onAddModel(): void
  onEditModel(id: string): void
  onModelDraftChanged(patch: Partial<ModelDraft>): void
  onSaveModel(): void
  onCancelModel(): void
  onTestConnection(): void | Promise<void>
  onCreateProvider(): void | Promise<void>
  onSaveProvider(): void | Promise<void>
  onCancelProvider(): void
  onCloneProvider(): void
}
