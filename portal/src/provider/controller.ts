import {
  createCreateProviderStore,
  isValidModelDraft,
  providerSnapshot,
  type CreateProviderController as Controller,
  type ApiProtocol,
  type EndpointMode,
  type ModelDraft,
  type ProviderModel,
  type TextModelProtocol,
  type HeaderEntry,
  type ModelCapability,
  type ModelStatusFilter,
} from '@compallm/storybook/pages/organization/create-provider'
import { notify } from '../notifications/toast'
import type { OrganizationController } from '@compallm/storybook/pages/organization'
import { UrlController } from '../url/controller'
import { readProviderUrl, writeProviderUrl, emptyModel, modelFingerprint } from './url-state'
import { providers } from './presets'

const paths: Record<EndpointMode, string> = {
  chat_completions: '/chat/completions',
  responses: '/responses',
  messages: '/messages',
  video: '/videos',
  realtime: '/realtime',
}
export class CreateProviderController implements Controller {
  readonly store = createCreateProviderStore({ providers, selectedProviderId: 'openai' })
  private readonly url: UrlController
  private readonly organization: OrganizationController
  private unsubscribe: (() => void) | undefined
  private unblock: (() => void) | undefined
  private avatarObjectUrl: string | undefined
  private applying = false
  private permitNavigation = false
  constructor(url: UrlController, organization: OrganizationController) {
    this.url = url
    this.organization = organization
  }
  start = () => {
    const apply = () => {
      this.applying = true
      const projection = readProviderUrl(this.url.store.getState())
      const previous = this.store.getState()
      if (JSON.stringify(projection.models) === JSON.stringify(previous.models)) projection.models = previous.models
      if (
        projection.modelDraft &&
        previous.modelDraft &&
        modelFingerprint(projection.modelDraft) === modelFingerprint(previous.modelDraft)
      )
        projection.modelDraft = previous.modelDraft
      this.store.setState({ ...projection, navigationRequest: null, test: { status: 'idle' } })
      this.configuredProviderId = this.store.getState().selectedProviderId
      this.endpoint()
      this.applying = false
    }
    apply()
    this.unsubscribe = this.url.store.subscribe(apply)
    this.unblock = this.url.block(
      (target) => {
        if (this.permitNavigation) return false
        const state = this.store.getState(),
          draft = state.modelDraft
        if (!draft) return false
        const original = state.models.find((model) => model.id === state.editingModelId) ?? emptyModel(draft.id, draft.protocol)
        if (modelFingerprint(draft) === modelFingerprint(original)) return false
        const current = new URLSearchParams(this.url.store.getState().search)
        return (
          target.pathname !== this.url.store.getState().pathname ||
          target.searchParams.get('dialog') !== current.get('dialog') ||
          target.searchParams.get('model') !== current.get('model')
        )
      },
      (request) =>
        this.store.setState({
          navigationRequest: {
            discard: () => {
              this.store.setState({ navigationRequest: null })
              request.discard()
            },
            keep: () => {
              this.store.setState({ navigationRequest: null })
              request.keep()
            },
          },
        }),
    )
    window.addEventListener('beforeunload', this.beforeUnload)
    return this.stop
  }
  stop = () => {
    this.unsubscribe?.()
    this.unblock?.()
    window.removeEventListener('beforeunload', this.beforeUnload)
    this.store.setState({ apiKey: '', headers: [], avatarUrl: null, avatarFile: null })
    if (this.avatarObjectUrl) URL.revokeObjectURL(this.avatarObjectUrl)
  }
  private beforeUnload = (event: BeforeUnloadEvent) => {
    const state = this.store.getState()
    if (
      state.apiKey ||
      state.headers.some((header) => header.key || header.value) ||
      (state.modelDraft &&
        modelFingerprint(state.modelDraft) !==
          modelFingerprint(
            state.models.find((m) => m.id === state.editingModelId) ??
              emptyModel(state.modelDraft.id, state.modelDraft.protocol),
          ))
    ) {
      event.preventDefault()
      event.returnValue = ''
    }
  }
  private persist(replace = true, allow = false) {
    if (this.applying) return
    const state = this.store.getState(),
      org = this.organization.getCurrentOrganization()
    if (!org) return
    const step = state.step === 'choose' ? 'choose-provider' : 'configure-provider'
    const pathname = `/${encodeURIComponent(org.name)}/providers/new/${step}`
    this.permitNavigation = allow
    const accepted = this.url.update(writeProviderUrl(state), { pathname, replace })
    if (accepted === false) {
      this.store.setState(readProviderUrl(this.url.store.getState()))
      this.endpoint()
    }
    this.permitNavigation = false
  }
  private configuredProviderId: string | null | undefined
  private invalidateTest() {
    this.store.setState({ test: { status: 'idle' } })
  }
  private endpoint() {
    const state = this.store.getState()
    const model = state.models.find((item) => item.enabled)
    const path = model?.path ?? paths[state.mode]
    this.store.setState({
      endpointUrl: state.baseUrl ? `${state.baseUrl.replace(/\/+$/, '')}/${path.replace(/^\/+/, '')}` : '',
    })
  }
  onProviderQueryChanged = (providerQuery: string) => {
    this.store.setState({ providerQuery })
    this.persist()
  }
  onSelectProvider = (selectedProviderId: string) => {
    this.store.setState({ selectedProviderId })
    this.configureSelectedProvider()
    this.persist(false)
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
      models: [],
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
    this.persist(false)
  }
  onChooseProvider = () => {
    this.store.setState({ step: 'choose' })
    this.persist(false)
  }
  onConnectionChanged = (field: 'name' | 'baseUrl' | 'apiKey', value: string) => {
    if (this.store.getState().submitting) return
    this.invalidateTest()
    this.store.setState({ [field]: value })
    this.endpoint()
    if (field !== 'apiKey') this.persist()
  }
  onProtocolChanged = (protocol: ApiProtocol) => {
    this.invalidateTest()
    this.store.setState({ protocol })
    this.endpoint()
    this.persist()
  }
  onModeChanged = (mode: EndpointMode) => {
    this.invalidateTest()
    this.store.setState((state) => ({
      mode,
      models: state.models.map((model) => ({ ...model, enabled: model.enabled && model.modes.includes(mode) })),
    }))
    this.endpoint()
    this.persist()
  }
  onModelQueryChanged = (modelQuery: string) => {
    this.store.setState({ modelQuery })
    this.persist()
  }
  onModelsChanged = (models: ProviderModel[]) => {
    this.invalidateTest()
    this.store.setState({ models })
    this.endpoint()
    this.persist()
  }
  onAddModel = (protocol: TextModelProtocol = 'openai_chat_completions') => {
    this.store.setState({ editingModelId: null, modelDraft: emptyModel(crypto.randomUUID(), protocol) })
    this.persist(false)
  }
  onEditModel = (id: string) => {
    const model = this.store.getState().models.find((model) => model.id === id)
    if (!model) return
    this.store.setState({ editingModelId: id, modelDraft: { ...model, thinkingLevels: [...model.thinkingLevels] } })
    this.persist(false)
  }
  onModelDraftChanged = (patch: Partial<ModelDraft>) => {
    this.store.setState((state) => ({ modelDraft: state.modelDraft ? { ...state.modelDraft, ...patch } : null }))
    this.persist()
  }
  onCancelModel = () => {
    this.store.setState({ modelDraft: null, editingModelId: null })
    this.persist(true, true)
  }
  onSaveModel = (draft: ModelDraft) => {
    const { models, editingModelId } = this.store.getState()
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
        draft.protocol === 'anthropic_messages'
          ? 'messages'
          : draft.protocol === 'openai_responses'
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
    this.persist(true, true)
  }
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
    status?: ModelStatusFilter
    protocol?: TextModelProtocol | 'all'
    capabilities?: ModelCapability[]
  }) => {
    this.store.setState({
      ...(filters.status ? { modelStatusFilter: filters.status } : {}),
      ...(filters.protocol ? { modelProtocolFilter: filters.protocol } : {}),
      ...(filters.capabilities ? { modelCapabilityFilter: filters.capabilities } : {}),
    })
    this.persist()
  }
  onProviderFilterChanged = (providerProtocolFilter: TextModelProtocol | 'all') => {
    this.store.setState({ providerProtocolFilter })
    this.persist()
  }
  onClearModelFilters = () => {
    this.store.setState({ modelStatusFilter: 'all', modelProtocolFilter: 'all', modelCapabilityFilter: [] })
    this.persist()
  }
  onTestConnection = () => {
    notify.info('Connection testing is not available yet.')
  }
  onSaveProvider = () => {
    notify.info('Saving provider changes is not available yet.')
    return false as const
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
    this.persist(false)
  }
  onCreateProvider = () => {
    notify.info('Provider creation is not available yet. Your configuration has not been saved.')
    return false as const
  }
}
