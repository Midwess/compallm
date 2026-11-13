import type {
  ProviderTemplate,
  TextModelProtocol as ContractTextModelProtocol,
  ProviderModelCapability,
  ProviderEffortMapping,
  ProviderCatalogRef,
} from '@compallm/contract/graphql'
import type { NavigationRequest } from '../../../url'
import type { FormStore } from '../../../components/form'

export type EndpointMode = 'chat_completions' | 'responses' | 'messages' | 'video' | 'realtime'
export type ApiProtocol = Lowercase<ProviderTemplate['protocol']>
export type TextModelProtocol = Lowercase<ContractTextModelProtocol>
export type ModelCapability = Lowercase<ProviderModelCapability>
export type ModelStatusFilter = 'all' | 'enabled' | 'disabled'
export interface HeaderEntry {
  id: string
  key: string
  value: string
}
export type EffortMapping = Partial<Record<keyof ProviderEffortMapping, string>>
export type ThinkingLevel = keyof EffortMapping
export type ProviderPreset = Pick<ProviderTemplate, 'id' | 'name'> & {
  protocol: ApiProtocol
  baseUrl?: ProviderTemplate['baseUrl']
}
export interface ModelDraft {
  /** Persisted API identity; UI draft IDs and catalog IDs never substitute for it. */
  persistedId?: string
  catalogRef?: ProviderCatalogRef
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
  avatarFile: File | null
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
  readonly store: FormStore<CreateProviderState>
  onProviderQueryChanged(query: string): void
  onSelectProvider(id: string): void
  onUseCustomProvider(): void
  onChooseProvider(): void
  onConnectionChanged(field: 'name' | 'baseUrl' | 'apiKey', value: string): void
  onProtocolChanged(protocol: ApiProtocol): void
  onModeChanged(mode: EndpointMode): void
  onModelQueryChanged(query: string): void
  onModelsChanged(models: ProviderModel[]): void
  onAddModel(protocol?: TextModelProtocol): void
  onHeadersChanged?(headers: HeaderEntry[]): void
  onAvatarChanged?(file: File | null): void
  onModelFiltersChanged?(filters: {
    status?: ModelStatusFilter
    protocol?: TextModelProtocol | 'all'
    capabilities?: ModelCapability[]
  }): void
  onProviderFilterChanged?(protocol: TextModelProtocol | 'all'): void
  onClearModelFilters?(): void
  onEditModel(id: string): void
  onModelDraftChanged(patch: Partial<ModelDraft>): void
  onSaveModel(draft: ModelDraft): void
  onCancelModel(): void
  onTestConnection(): void | Promise<void>
  onCreateProvider(
    values: Partial<CreateProviderState>,
  ): void | false | Partial<CreateProviderState> | Promise<void | false | Partial<CreateProviderState>>
  onSaveProvider(
    values: Partial<CreateProviderState>,
  ): void | false | Partial<CreateProviderState> | Promise<void | false | Partial<CreateProviderState>>
  onCancelProvider(): void
  onCloneProvider(): void
}
