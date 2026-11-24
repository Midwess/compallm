import type { ProviderCatalog } from '../../api-types'
import type {
  ProviderApiProtocol$options as ProviderApiProtocol,
  TextModelProtocol$options as ContractTextModelProtocol,
  ProviderModelCapability$options as ProviderModelCapability,
  ProviderEffortMapping,
  ProviderCatalogRef,
} from '@compallm/contract/houdini'
import type { NavigationRequest } from '../../../url'
import type { FormStore } from '../../../components/form'

export type EndpointMode = 'chat_completions' | 'responses' | 'messages' | 'video' | 'realtime'
export type ApiProtocol = Lowercase<ProviderApiProtocol>
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
export type ProviderPreset = Pick<ProviderCatalog, 'id' | 'name'> & {
  protocol?: ApiProtocol
  baseUrl?: ProviderCatalog['baseUrl']
  logoPath?: NonNullable<ProviderCatalog['logoUrl']>
}
export interface ModelDraft {
  origin?: 'GLOBAL' | 'OVERRIDE' | 'CUSTOM'
  catalogPriceLabels?: string[]

  /** Persisted API identity; UI draft IDs and catalog IDs never substitute for it. */
  persistedId?: string
  catalogRef?: ProviderCatalogRef
  id: string
  name: string
  maxContextTokens: string
  providerContextTokens?: string
  maxOutputTokens?: string
  maxInputTokens?: string
  contextLimitEnabled?: boolean
  inputLimitEnabled?: boolean
  outputLimitEnabled?: boolean
  headers?: HeaderEntry[]
  bodyDefaults?: HeaderEntry[]
  protocol?: TextModelProtocol
  path?: string
  effortMapping?: EffortMapping
  capabilities?: ModelCapability[]
  thinkingLevels: ThinkingLevel[]
  inputPrice: string
  outputPrice: string
  cachedPrice: string
  inputPriceEnabled?: boolean
  outputPriceEnabled?: boolean
  cachedPriceEnabled?: boolean
  catalogPrices?: Pick<ModelDraft, 'inputPrice' | 'outputPrice' | 'cachedPrice'>
  defaults?: Omit<ModelDraft, 'defaults'>
}
export interface ProviderModel extends ModelDraft {
  enabled: boolean
  modes: EndpointMode[]
  description?: string
}
export type ConnectionTest = { status: 'idle' | 'testing' } | { status: 'passed' } | { status: 'failed'; message: string }
export interface ProviderSnapshot {
  id?: string
  name: string
  baseUrl: string
  protocol: ApiProtocol
  mode: EndpointMode
  models: ProviderModel[]
  headers?: HeaderEntry[]
  avatarUrl?: string | null
}
export interface CreateProviderState {
  providersLoaded?: boolean
  providerLoaded?: boolean
  modelLoaded?: boolean
  templatePreview?: boolean
  creationEnabled?: boolean
  readOnly?: boolean
  providersLoading?: boolean
  providersHasMore?: boolean
  providersLoadError?: string | null
  providerLoading?: boolean
  modelLoading?: boolean
  catalogError?: string | null
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
  readonly loading?: {
    providers: import('../../../app').LoadingController
    provider: import('../../../app').LoadingController
    model: import('../../../app').LoadingController
  }
  onProviderQueryChanged(query: string): void
  onProviderSearch?(): void
  onLoadMoreProviders?(): void
  onRetryCatalog?(): void
  providerHref?(id: string): string
  modelHref?(id: string): string
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
  validateModel?(draft: ModelDraft, fields?: string[]): { path: PropertyKey[]; message: string }[] | undefined
  onResetModel?(): Promise<void>
  onDeleteModel?(): Promise<void>
  onSaveModel(draft: ModelDraft, fields?: string[]): ModelDraft | false | Promise<ModelDraft | false>
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
