import type { NavigationRequest } from '../../../url'
import type { StoreApi } from 'zustand/vanilla'

export type EndpointMode = 'chat_completions' | 'responses' | 'messages' | 'video' | 'realtime'
export type ApiProtocol = 'openai_compatible' | 'anthropic' | 'provider_specific'
export type TextModelProtocol = 'openai_chat_completions' | 'openai_responses' | 'anthropic_messages'
export type ModelCapability =
  | 'text'
  | 'vision'
  | 'pdf'
  | 'audio'
  | 'video'
  | 'image_generation'
  | 'audio_generation'
  | 'search'
  | 'reasoning'
  | 'tool_call'
  | 'forced_tool_choice'
  | 'structured_output'
export type ModelStatusFilter = 'all' | 'enabled' | 'disabled'
export interface HeaderEntry {
  id: string
  key: string
  value: string
}
export type EffortMapping = Partial<Record<ThinkingLevel, string>>
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
  maxOutputTokens?: string
  protocol?: TextModelProtocol
  path?: string
  effortMapping?: EffortMapping
  capabilities?: ModelCapability[]
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
  headers?: HeaderEntry[]
  avatarUrl?: string | null
}
export interface CreateProviderState {
  navigationRequest: NavigationRequest | null
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
  headers: HeaderEntry[]
  avatarUrl: string | null
  modelCapabilityFilter: ModelCapability[]
  modelStatusFilter: ModelStatusFilter
  modelProtocolFilter: TextModelProtocol | 'all'
  providerProtocolFilter: TextModelProtocol | 'all'
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
  onAddModel(protocol?: TextModelProtocol): void
  onHeadersChanged?(headers: HeaderEntry[]): void
  onAvatarChanged?(file: File | null): void
  onModelFiltersChanged?(filters: {
    status?: ModelStatusFilter
    protocol?: TextModelProtocol | 'all'
    capabilities?: ModelCapability[]
  }): void
  onProviderFilterChanged?(protocol: TextModelProtocol | 'all'): void
  onModelsEnabledChanged?(ids: string[], enabled: boolean): void
  onClearModelFilters?(): void
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
