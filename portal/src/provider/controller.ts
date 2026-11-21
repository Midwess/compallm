import {
  createCreateProviderStore,
  type CreateProviderController as Controller,
  type ModelDraft,
  type ProviderModel,
} from '@compallm/storybook/pages/organization/create-provider'
import { processInSlices } from '@compallm/storybook'
import type { OrganizationController } from '@compallm/storybook/pages/organization'
import type { UrlController } from '../url/controller'
import { readProviderUrl, writeProviderUrl } from './url-state'
import { listProviders, getProvider, getModel } from './catalog'
import { providerRow, modelRow, modelDraft } from './presentation'

export class CreateProviderController implements Controller {
  readonly store = createCreateProviderStore({ templatePreview: true, protocol: 'provider_specific' })
  private owner?: AbortController
  private listRequest?: AbortController
  private providerRequest?: AbortController
  private modelRequest?: AbortController
  private unsubscribe?: () => void
  private query?: string
  private provider?: string | null
  private model?: string | null
  private cursor?: string
  private appendFailed = false
  private loadedProvider?: string
  private readonly url: Pick<UrlController, 'store' | 'update'>
  private readonly organization: Pick<OrganizationController, 'getCurrentOrganization'>
  constructor(
    url: Pick<UrlController, 'store' | 'update'>,
    organization: Pick<OrganizationController, 'getCurrentOrganization'>,
  ) {
    this.url = url
    this.organization = organization
  }
  start = () => {
    this.stop()
    this.owner = new AbortController()
    this.query = undefined
    this.provider = undefined
    this.model = undefined
    this.loadedProvider = undefined
    this.unsubscribe = this.url.store.subscribe(this.apply)
    this.apply()
    return this.stop
  }
  stop = () => {
    this.owner?.abort()
    this.listRequest?.abort()
    this.providerRequest?.abort()
    this.modelRequest?.abort()
    this.unsubscribe?.()
    this.unsubscribe = undefined
    this.owner = undefined
  }
  private apply = () => {
    const next = readProviderUrl(this.url.store.getState())
    const provider = next.step === 'configure' ? next.provider : null
    this.store.setState({
      step: next.step,
      selectedProviderId: provider,
      providerQuery: next.query,
      modelQuery: next.modelQuery,
    })
    if (next.step === 'choose' && (this.query !== next.query || (!this.store.getState().providers.length && !this.listRequest)))
      void this.loadProviders(false)
    if (this.provider !== provider) {
      this.provider = provider
      this.loadedProvider = undefined
      this.providerRequest?.abort()
      this.modelRequest?.abort()
      this.model = undefined
      this.store.setState({
        models: [],
        modelDraft: null,
        editingModelId: null,
        name: '',
        baseUrl: '',
        apiKey: '',
        avatarUrl: null,
        avatarFile: null,
        headers: [],
        catalogError: null,
        providerLoading: false,
        modelLoading: false,
      })
      if (provider) void this.loadProvider(provider)
    }
    const model = provider ? next.model : null
    if (this.model !== model) {
      this.model = model
      this.modelRequest?.abort()
      if (!model) this.store.setState({ modelDraft: null, editingModelId: null, modelLoading: false, catalogError: null })
      else if (this.loadedProvider === provider) void this.loadModel(model)
    }
  }
  private navigate(step: 'choose' | 'configure', provider: string | null, model: string | null = null, replace = false) {
    const org = this.organization.getCurrentOrganization()
    if (!org) return
    this.store.setState({ step, selectedProviderId: provider, editingModelId: model })
    const page = step === 'choose' ? 'choose-provider' : 'configure-provider'
    this.url.update(writeProviderUrl(this.store.getState()), {
      pathname: `/${encodeURIComponent(org.name)}/providers/new/${page}`,
      replace,
    })
  }
  onProviderQueryChanged = (providerQuery: string) => {
    this.store.setState({ providerQuery })
    void this.loadProviders(false)
    this.url.update({ q: providerQuery || null }, { replace: true })
  }
  onLoadMoreProviders = () => {
    const s = this.store.getState()
    if (s.providersHasMore && !s.providersLoading) void this.loadProviders(true)
  }
  private async loadProviders(append: boolean) {
    const owner = this.owner
    if (!owner) return
    this.listRequest?.abort()
    const request = new AbortController()
    this.listRequest = request
    const query = this.store.getState().providerQuery.trim()
    this.query = this.store.getState().providerQuery
    if (!append) this.cursor = undefined
    this.appendFailed = append
    this.store.setState({ providersLoading: true, providersLoadError: null, ...(!append ? { providersHasMore: false } : {}) })
    try {
      const data = await listProviders(query, append ? this.cursor : undefined, AbortSignal.any([owner.signal, request.signal]))
      if (owner.signal.aborted || request.signal.aborted || this.listRequest !== request) return
      this.cursor = data.pageInfo.endCursor ?? undefined
      this.store.setState((state) => {
        const rows = new Map((append ? state.providers : []).map((row) => [row.id, row]))
        for (const { node } of data.edges) {
          if (!rows.has(node.id)) rows.set(node.id, providerRow(node))
        }
        return { providers: [...rows.values()], providersHasMore: data.pageInfo.hasNextPage, providersLoadError: null }
      })
    } catch {
      if (!owner.signal.aborted && !request.signal.aborted && this.listRequest === request)
        this.store.setState({ providersLoadError: 'Could not load provider templates. Try again.' })
    } finally {
      if (this.listRequest === request) this.store.setState({ providersLoading: false })
    }
  }
  onSelectProvider = (id: string) => this.navigate('configure', id)
  onChooseProvider = () => this.navigate('choose', null)
  private async loadProvider(id: string) {
    const owner = this.owner
    if (!owner) return
    this.providerRequest?.abort()
    const request = new AbortController()
    this.providerRequest = request
    this.store.setState({ providerLoading: true, catalogError: null })
    try {
      const signal = AbortSignal.any([owner.signal, request.signal])
      const value = await getProvider(id, signal)
      if (signal.aborted || this.providerRequest !== request || this.provider !== id) return
      if (!value) throw new Error('missing provider')
      const models: ProviderModel[] = []
      await processInSlices(value.models.edges, ({ node }) => models.push(modelRow(node, id)), signal)
      if (signal.aborted || this.providerRequest !== request || this.provider !== id) return
      this.loadedProvider = id
      this.store.setState({
        name: value.name,
        baseUrl: value.connection.baseUrl ?? '',
        avatarUrl: value.logoUrl ?? null,
        headers: value.headers.map((v, index) => ({ id: String(index), ...v })),
        models,
        catalogError: null,
      })
      const selected = readProviderUrl(this.url.store.getState()).model
      if (selected) {
        this.model = selected
        void this.loadModel(selected)
      }
    } catch {
      if (!owner.signal.aborted && !request.signal.aborted && this.providerRequest === request)
        this.store.setState({ catalogError: 'Could not load this provider template. Try again.' })
    } finally {
      if (this.providerRequest === request) {
        this.store.setState({ providerLoading: false })
      }
    }
  }
  onModelQueryChanged = (modelQuery: string) => {
    this.store.setState({ modelQuery })
    this.url.update({ model_q: modelQuery || null }, { replace: true })
  }
  onEditModel = (id: string) => {
    if (this.provider) this.navigate('configure', this.provider, id)
  }
  private async loadModel(id: string) {
    const owner = this.owner,
      provider = this.provider
    if (!owner || !provider) return
    this.modelRequest?.abort()
    const request = new AbortController()
    this.modelRequest = request
    const summary = this.store.getState().models.find((v) => v.id === id)
    const pending: ModelDraft = summary ?? {
      id,
      name: id,
      maxContextTokens: '',
      thinkingLevels: [],
      inputPrice: '',
      outputPrice: '',
      cachedPrice: '',
    }
    this.store.setState({
      modelDraft: { ...pending },
      editingModelId: id,
      modelLoading: true,
      catalogError: null,
    })
    try {
      const signal = AbortSignal.any([owner.signal, request.signal])
      const value = await getModel(provider, id, signal)
      if (signal.aborted || this.modelRequest !== request || this.provider !== provider || this.model !== id) return
      if (!value) throw new Error('missing model')
      this.store.setState({ modelDraft: modelDraft(value), catalogError: null })
    } catch {
      if (!owner.signal.aborted && !request.signal.aborted && this.modelRequest === request) {
        request.abort()
        this.store.setState({ catalogError: 'Could not load this model template. It may no longer be available. Try again.' })
      }
    } finally {
      if (this.modelRequest === request) this.store.setState({ modelLoading: false })
    }
  }
  onCancelModel = () => {
    if (this.provider) this.navigate('configure', this.provider)
  }
  onRetryCatalog = () => {
    const s = this.store.getState()
    if (s.step === 'choose') void this.loadProviders(this.appendFailed)
    else if (this.provider && this.loadedProvider !== this.provider) void this.loadProvider(this.provider)
    else if (this.model) void this.loadModel(this.model)
  }
  onModelFiltersChanged: Controller['onModelFiltersChanged'] = (filters) =>
    this.store.setState({
      ...(filters?.status ? { modelStatusFilter: filters.status } : {}),
      ...(filters?.protocol ? { modelProtocolFilter: filters.protocol } : {}),
      ...(filters?.capabilities ? { modelCapabilityFilter: filters.capabilities } : {}),
    })
  onClearModelFilters = () =>
    this.store.setState({ modelStatusFilter: 'all', modelProtocolFilter: 'all', modelCapabilityFilter: [] })
  onUseCustomProvider = () => {}
  onConnectionChanged: Controller['onConnectionChanged'] = (field, value) => this.store.setState({ [field]: value })
  onHeadersChanged: Controller['onHeadersChanged'] = (headers) => this.store.setState({ headers })
  onProtocolChanged = () => {}
  onModeChanged = () => {}
  onModelsChanged: Controller['onModelsChanged'] = (models) => this.store.setState({ models })
  onAddModel = () => {}
  onModelDraftChanged: Controller['onModelDraftChanged'] = (patch) => {
    const draft = this.store.getState().modelDraft
    if (draft) this.store.setState({ modelDraft: { ...draft, ...patch } })
  }
  onSaveModel: Controller['onSaveModel'] = (draft) => {
    this.store.setState({ modelDraft: draft })
    return draft
  }
  onTestConnection = () => {}
  onCreateProvider = () => false as const
  onSaveProvider = () => false as const
  onCancelProvider = this.onChooseProvider
  onCloneProvider = () => {}
}
