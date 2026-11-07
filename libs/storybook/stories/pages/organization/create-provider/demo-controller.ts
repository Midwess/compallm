import {
  createCreateProviderStore,
  type CreateProviderController,
  type ApiProtocol,
  type EndpointMode,
  type ModelDraft,
  type ProviderModel,
  type TextModelProtocol,
  type HeaderEntry,
  type ModelCapability,
} from '../../../../src/pages/organization/create-provider'
import {
  canCreateProvider,
  canSaveProvider,
  providerSnapshot,
  hasValidModelPrices,
  isValidModelDraft,
} from '../../../../src/pages/organization/create-provider/validation'
import {
  joinModelEndpoint,
  modelPath,
  modelProtocol,
  textModelOptions,
} from '../../../../src/pages/organization/create-provider/model-options'
import { providers, openaiModels } from './fixtures'

type Scenario =
  | 'choose'
  | 'preset'
  | 'custom'
  | 'dialog'
  | 'passed'
  | 'failed'
  | 'testing'
  | 'submitting'
  | 'no-results'
  | 'missing-endpoint'
  | 'saved'
  | 'saved-custom'
  | 'changed'
  | 'clone'
  | 'headers'
  | 'response-dialog'
  | 'anthropic-dialog'
  | 'filtered'
  | 'capabilities'
  | 'long-model-name'
export type { Scenario }
const paths: Record<EndpointMode, string> = {
  chat_completions: '/chat/completions',
  responses: '/responses',
  messages: '/messages',
  video: '/videos',
  realtime: '/realtime',
}
export class DemoCreateProviderController implements CreateProviderController {
  readonly store = createCreateProviderStore({ providers, selectedProviderId: 'openai' })
  private generation = 0
  private nextModelId = 0
  private configuredProviderId: string | null | undefined
  private avatarObjectUrl: string | undefined
  private timer: ReturnType<typeof setTimeout> | undefined
  constructor(
    scenario: Scenario,
    private readonly report: (intent: string) => void,
  ) {
    if (scenario === 'choose' || scenario === 'no-results' || scenario === 'missing-endpoint') {
      if (scenario === 'missing-endpoint')
        this.store.setState({
          providers: [...providers, { id: 'azure', name: 'Azure OpenAI', protocol: 'provider_specific', baseUrl: null }],
        })
      if (scenario === 'no-results') this.store.setState({ providerQuery: 'No matching provider' })
      return
    }
    if (scenario === 'custom' || scenario === 'dialog') this.onUseCustomProvider()
    else {
      this.configureSelectedProvider()
      this.store.setState({
        apiKey: 'preview-only-example',
        models: openaiModels.map((model, index) => ({ ...model, enabled: index < 2 })),
      })
    }
    if (['saved', 'saved-custom', 'changed', 'clone'].includes(scenario)) {
      if (scenario === 'saved-custom')
        this.store.setState({ name: 'Internal inference', baseUrl: 'https://inference.example.com/v1' })
      this.store.setState({ intent: 'edit', apiKey: '' })
      this.store.setState({ savedProvider: providerSnapshot(this.store.getState()) })
      if (scenario === 'changed') this.onConnectionChanged('name', 'OpenAI production updated')
      if (scenario === 'clone') this.onCloneProvider()
    }
    if (scenario === 'dialog') this.onAddModel()
    if (scenario === 'response-dialog') this.onAddModel('openai_responses')
    if (scenario === 'anthropic-dialog') this.onAddModel('anthropic_messages')
    if (scenario === 'headers')
      this.onHeadersChanged([
        { id: 'client', key: 'X-Client', value: 'Compallm' },
        { id: 'version', key: 'X-Version', value: '1' },
      ])
    if (scenario === 'capabilities') this.onModelFiltersChanged({ capabilities: ['vision', 'reasoning'] })
    if (scenario === 'long-model-name')
      this.store.setState((state) => ({
        models: [
          ...state.models,
          {
            ...openaiModels[0],
            id: 'long-name',
            name: 'organization-specialized-reasoning-model-with-an-extremely-long-upstream-name-version-2026',
            protocol: 'openai_responses',
            capabilities: ['text', 'vision', 'reasoning', 'tool_call'],
            enabled: false,
          },
        ],
      }))
    if (scenario === 'filtered') this.onModelFiltersChanged({ status: 'enabled' })
    if (scenario === 'passed') this.store.setState({ test: { status: 'passed' } })
    if (scenario === 'failed')
      this.store.setState({
        test: { status: 'failed', message: 'Authentication failed (401). Check the API key and try again.' },
      })
    if (scenario === 'testing') this.store.setState({ test: { status: 'testing' } })
    if (scenario === 'submitting') this.store.setState({ submitting: true })
  }
  stop = () => {
    this.generation += 1
    if (this.timer) clearTimeout(this.timer)
    if (this.avatarObjectUrl) URL.revokeObjectURL(this.avatarObjectUrl)
  }
  private invalidateTest() {
    this.generation += 1
    if (this.timer) clearTimeout(this.timer)
    this.store.setState({ test: { status: 'idle' } })
  }
  private endpoint() {
    const state = this.store.getState()
    this.store.setState({
      endpointUrl: state.baseUrl
        ? joinModelEndpoint(
            state.baseUrl,
            modelPath(
              state.models.find((model) => model.enabled) ?? { ...this.emptyModel('endpoint'), path: paths[state.mode] },
            ),
          )
        : '',
    })
  }
  onProviderQueryChanged = (providerQuery: string) => this.store.setState({ providerQuery })
  onSelectProvider = (selectedProviderId: string) => {
    this.store.setState({ selectedProviderId })
    this.configureSelectedProvider()
  }
  private configureSelectedProvider() {
    const state = this.store.getState()
    const preset = state.providers.find((provider) => provider.id === state.selectedProviderId)
    if (!preset) return
    if (this.configuredProviderId === state.selectedProviderId) {
      this.store.setState({ step: 'configure' })
      return
    }
    this.invalidateTest()
    this.configuredProviderId = preset.id
    this.store.setState({
      step: 'configure',
      name: `${preset.name} production`,
      baseUrl: preset.baseUrl ?? '',
      protocol: preset.protocol,
      apiKey: '',
      headers: [],
      avatarUrl: null,
      mode: preset.protocol === 'anthropic' ? 'messages' : 'chat_completions',
      models: preset.id === 'openai' ? openaiModels.map((model) => ({ ...model })) : [],
      modelQuery: '',
      error: null,
    })
    this.endpoint()
  }
  onUseCustomProvider = () => {
    this.invalidateTest()
    this.configuredProviderId = null
    this.store.setState({
      step: 'configure',
      selectedProviderId: null,
      name: '',
      baseUrl: '',
      apiKey: '',
      headers: [],
      avatarUrl: null,
      protocol: 'openai_compatible',
      mode: 'chat_completions',
      models: [],
      modelQuery: '',
      error: null,
    })
    this.endpoint()
  }
  onChooseProvider = () => this.store.setState({ step: 'choose' })
  onConnectionChanged = (field: 'name' | 'baseUrl' | 'apiKey', value: string) => {
    if (this.store.getState().submitting) return
    this.invalidateTest()
    this.store.setState({ [field]: value })
    this.endpoint()
  }
  onProtocolChanged = (protocol: ApiProtocol) => {
    this.invalidateTest()
    this.store.setState({ protocol })
    this.endpoint()
  }
  onModeChanged = (mode: EndpointMode) => {
    this.invalidateTest()
    this.store.setState((state) => ({
      mode,
      models: state.models.map((model) => ({ ...model, enabled: model.enabled && model.modes.includes(mode) })),
    }))
    this.endpoint()
  }
  onModelQueryChanged = (modelQuery: string) => this.store.setState({ modelQuery })
  onModelEnabledChanged = (id: string, enabled: boolean) => {
    this.invalidateTest()
    this.store.setState((state) => ({
      models: state.models.map((model) =>
        model.id === id &&
        (model.protocol || model.modes.some((mode) => ['chat_completions', 'responses', 'messages'].includes(mode))) &&
        hasValidModelPrices(model)
          ? { ...model, enabled }
          : model,
      ),
    }))
  }
  private emptyModel(id: string, protocol: TextModelProtocol = 'openai_chat_completions'): ModelDraft {
    return {
      id,
      name: '',
      maxContextTokens: '250000',
      maxOutputTokens: '',
      thinkingLevels: [],
      effortMapping: {},
      protocol,
      path: textModelOptions.find((option) => option.value === protocol)!.path,
      inputPrice: '',
      outputPrice: '',
      cachedPrice: '',
    }
  }
  onAddModel = (protocol: TextModelProtocol = 'openai_chat_completions') =>
    this.store.setState({ editingModelId: null, modelDraft: this.emptyModel(`custom-${++this.nextModelId}`, protocol) })
  onHeadersChanged = (headers: HeaderEntry[]) => {
    this.invalidateTest()
    this.store.setState({ headers })
  }
  onAvatarChanged = (file: File | null) => {
    if (this.avatarObjectUrl) URL.revokeObjectURL(this.avatarObjectUrl)
    this.avatarObjectUrl = file ? URL.createObjectURL(file) : undefined
    this.store.setState({ avatarUrl: this.avatarObjectUrl ?? null })
  }
  onModelFiltersChanged = (filters: {
    status?: import('../../../../src/pages/organization/create-provider').ModelStatusFilter
    protocol?: TextModelProtocol | 'all'
    capabilities?: ModelCapability[]
  }) =>
    this.store.setState({
      ...(filters.status ? { modelStatusFilter: filters.status } : {}),
      ...(filters.protocol ? { modelProtocolFilter: filters.protocol } : {}),
      ...(filters.capabilities ? { modelCapabilityFilter: filters.capabilities } : {}),
    })
  onProviderFilterChanged = (providerProtocolFilter: TextModelProtocol | 'all') =>
    this.store.setState({ providerProtocolFilter })
  onClearModelFilters = () =>
    this.store.setState({ modelStatusFilter: 'all', modelProtocolFilter: 'all', modelCapabilityFilter: [] })
  onModelsEnabledChanged = (ids: string[], enabled: boolean) => {
    this.invalidateTest()
    this.store.setState((state) => ({
      models: state.models.map((model) =>
        ids.includes(model.id) && hasValidModelPrices(model) ? { ...model, enabled } : model,
      ),
    }))
  }
  onEditModel = (id: string) => {
    const model = this.store.getState().models.find((model) => model.id === id)
    if (model) this.store.setState({ editingModelId: id, modelDraft: { ...model, thinkingLevels: [...model.thinkingLevels] } })
  }
  onModelDraftChanged = (patch: Partial<ModelDraft>) =>
    this.store.setState((state) => ({ modelDraft: state.modelDraft ? { ...state.modelDraft, ...patch } : null }))
  onCancelModel = () => this.store.setState({ modelDraft: null, editingModelId: null })
  onSaveModel = () => {
    const { modelDraft: draft, models, editingModelId } = this.store.getState()
    if (
      !draft ||
      !isValidModelDraft(draft) ||
      models.some((model) => model.id !== editingModelId && model.name.trim() === draft.name.trim())
    )
      return
    this.invalidateTest()
    const previous = models.find((model) => model.id === editingModelId)
    const model: ProviderModel = {
      ...draft,
      name: draft.name.trim(),
      modes: previous?.modes ?? [
        modelProtocol(draft) === 'anthropic_messages'
          ? 'messages'
          : modelProtocol(draft) === 'openai_responses'
            ? 'responses'
            : 'chat_completions',
      ],
      enabled: previous?.enabled ?? true,
      description: previous?.description,
    }
    this.store.setState({
      models: previous ? models.map((item) => (item.id === editingModelId ? model : item)) : [...models, model],
      modelDraft: null,
      editingModelId: null,
    })
    this.endpoint()
    this.report('Model saved in preview')
  }
  onTestConnection = () => {
    const state = this.store.getState()
    if (state.test.status === 'testing' || !canCreateProvider(state)) return
    const generation = ++this.generation
    this.store.setState({ test: { status: 'testing' } })
    this.timer = setTimeout(() => {
      if (generation !== this.generation) return
      this.store.setState({
        test:
          state.apiKey === 'fail'
            ? { status: 'failed', message: 'Authentication failed (401). Check the API key and try again.' }
            : { status: 'passed' },
      })
    }, 400)
  }
  onSaveProvider = () => {
    const state = this.store.getState()
    if (state.submitting || !canSaveProvider(state)) return
    this.store.setState({ savedProvider: providerSnapshot(state), apiKey: '' })
    this.report(`Save provider: ${state.name}`)
  }
  onCancelProvider = () => {
    const saved = this.store.getState().savedProvider
    if (!saved) return
    this.invalidateTest()
    this.store.setState({ ...structuredClone(saved), intent: 'edit', apiKey: '', modelDraft: null, editingModelId: null })
    this.endpoint()
  }
  onCloneProvider = () => {
    const state = this.store.getState()
    this.invalidateTest()
    const source = state.savedProvider ?? providerSnapshot(state)
    this.store.setState({
      ...structuredClone(source),
      name: `${source.name} clone`,
      intent: 'clone',
      apiKey: '',
      modelDraft: null,
      editingModelId: null,
      modelQuery: '',
      error: null,
    })
    this.endpoint()
    this.report('Clone provider draft')
  }
  onCreateProvider = () => {
    const state = this.store.getState()
    if (state.submitting || !canCreateProvider(state)) return
    this.report(`Create provider: ${state.name} · ${state.models.filter((model) => model.enabled).length} models`)
  }
}
