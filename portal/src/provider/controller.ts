import type { AppController, AppLoadingTask } from '@compallm/storybook'
import {
  createCreateProviderStore,
  type CreateProviderController as Controller,
  type ModelDraft,
  type ProviderModel,
  templateCustomization,
} from '@compallm/storybook/pages/organization/create-provider'
import { processInSlices } from '@compallm/storybook'
import type { OrganizationController } from '@compallm/storybook/pages/organization'
import type { UrlController } from '../url/controller'
import { readProviderUrl, writeProviderUrl } from './url-state'
import { listProviders, getProvider, getModel, getGlobalModel, modelDetail } from './catalog'
import { providerRow, modelRow, modelDraft } from './presentation'
import { graphql, ApiError } from '../graphql-client'
import { notify } from '../notifications/toast'
import {
  ModelTemplateCustomizeInput,
  ModelTemplateCreateInput,
  ModelTemplateEffectiveLimitsInput,
  ProviderCreateFromTemplateInput,
  TextModelProtocol,
} from '@compallm/contract/validation'
import type { MutationGenqlSelection } from '@compallm/contract/graphql'

export class CreateProviderController implements Controller {
  readonly store = createCreateProviderStore({ templatePreview: true, creationEnabled: true, protocol: 'provider_specific' })
  readonly loading: NonNullable<Controller['loading']>
  private loadingTasks = new Map<string, AppLoadingTask>()
  private owner?: AbortController
  private listRequest?: AbortController
  private providerRequest?: AbortController
  private modelRequest?: AbortController
  private unsubscribe?: () => void
  private query?: string
  private provider?: string | null
  private model?: string | null
  private departingModel?: string | null
  private confirmedModels = new Map<string, ModelDraft>()
  private localModelIds = new Map<string, string>()
  private writes = new Map<string, Promise<unknown>>()
  private uploadedLogo?: { file: File; id: string }
  private cursor?: string
  private appendFailed = false
  private loadedProvider?: string
  private readonly url: Pick<UrlController, 'store' | 'update' | 'back'>
  private readonly organization: Pick<OrganizationController, 'getCurrentOrganization' | 'onProviderCreated'>
  constructor(
    url: Pick<UrlController, 'store' | 'update' | 'back'>,
    organization: Pick<OrganizationController, 'getCurrentOrganization' | 'onProviderCreated'>,
    app: Pick<AppController, 'createLoadingScope'>,
  ) {
    this.loading = {
      providers: app.createLoadingScope(),
      provider: app.createLoadingScope(),
      model: app.createLoadingScope(),
    }
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
    for (const task of this.loadingTasks.values()) task.cancel()
    this.loadingTasks.clear()
    this.owner?.abort()
    this.listRequest?.abort()
    this.providerRequest?.abort()
    this.modelRequest?.abort()
    this.unsubscribe?.()
    this.unsubscribe = undefined
    this.owner = undefined
  }
  startSaved = (id: string) => {
    this.stop()
    const owner = new AbortController()
    this.owner = owner
    this.store.setState({
      intent: 'edit',
      templatePreview: true,
      creationEnabled: false,
      readOnly: true,
      step: 'configure',
      providerLoaded: false,
      providerLoading: true,
    })
    const loading = this.loading.provider.beginLoading('saved-provider')
    const selection = {
      provider: {
        __args: { id },
        node: {
          id: true,
          name: true,
          description: true,
          logoUrl: true,
          connection: { baseUrl: true },
          models: {
            id: true,
            name: true,
            enabled: true,
            capabilities: true,
            inputPrice: true,
            outputPrice: true,
            cachedPrice: true,
            protocol: true,
            connection: { path: true },
            contextTokens: true,
            text: { input: { configuration: { maxTokens: true } }, output: { configuration: { maxTokens: true } } },
          },
        },
      },
    } satisfies import('@compallm/contract/graphql').QueryGenqlSelection
    void graphql(owner.signal)
      .query(selection)
      .then((result) => {
        const value = result.provider?.node
        if (!value || owner.signal.aborted) return
        const models: ProviderModel[] = value.models.map((v) => ({
          id: v.id,
          name: v.name,
          enabled: v.enabled,
          protocol: (
            {
              OPENAI: 'openai_chat_completions',
              OPENAI_RESPONSES: 'openai_responses',
              ANTHROPIC: 'anthropic_messages',
            } as Partial<Record<string, ModelDraft['protocol']>>
          )[v.protocol],
          path: v.connection.path ?? undefined,
          modes: [],
          maxContextTokens: v.contextTokens?.toString() ?? '',
          maxInputTokens: v.text.input.configuration?.maxTokens?.toString(),
          maxOutputTokens: v.text.output.configuration?.maxTokens?.toString(),
          thinkingLevels: [],
          capabilities: v.capabilities.map((c) => c.toLowerCase()) as ModelDraft['capabilities'],
          inputPrice: v.inputPrice ?? '',
          outputPrice: v.outputPrice ?? '',
          cachedPrice: v.cachedPrice ?? '',
        }))
        const saved = {
          id: value.id,
          name: value.name,
          baseUrl: value.connection.baseUrl ?? '',
          models,
          protocol: 'provider_specific' as const,
          mode: 'chat_completions' as const,
          avatarUrl: value.logoUrl,
        }
        this.store.setState({ ...saved, savedProvider: saved, providerLoaded: true })
      })
      .catch((error) => {
        if (!owner.signal.aborted) notify.error(error)
      })
      .finally(() => {
        loading.complete()
        if (!owner.signal.aborted) this.store.setState({ providerLoading: false })
      })
    return this.stop
  }
  private apply = () => {
    const next = readProviderUrl(this.url.store.getState())
    const provider = next.step === 'configure' ? next.provider : null
    this.store.setState({
      step: next.step,
      selectedProviderId: provider,
      ...(this.query !== next.query ? { providerQuery: next.query } : {}),
      modelQuery: next.modelQuery,
    })
    if (
      next.step === 'choose' &&
      (this.query !== next.query || (!this.store.getState().providers.length && !this.listRequest))
    ) {
      this.query = next.query
      void this.loadProviders(false)
    }
    if (this.provider !== provider) {
      this.confirmedModels.clear()
      this.localModelIds.clear()
      this.uploadedLogo = undefined
      this.provider = provider
      this.loadedProvider = undefined
      this.providerRequest?.abort()
      this.modelRequest?.abort()
      this.loadingTasks.get('provider')?.cancel()
      this.loadingTasks.get('model')?.cancel()
      this.model = undefined
      this.store.setState({
        providerLoaded: false,
        modelLoaded: false,
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
    const model = provider && next.model ? (this.localModelIds.get(next.model) ?? next.model) : null
    if (this.model !== model) {
      this.model = model
      this.modelRequest?.abort()
      this.loadingTasks.get('model')?.cancel()
      if (model && this.loadedProvider !== provider) {
        this.store.setState({
          editingModelId: model,
          modelDraft: {
            id: model,
            name: model,
            maxContextTokens: '',
            thinkingLevels: [],
            inputPrice: '',
            outputPrice: '',
            cachedPrice: '',
          },
        })
      }
      if (!model)
        this.store.setState({
          modelDraft: null,
          editingModelId: null,
          modelLoading: false,
          modelLoaded: false,
          catalogError: null,
        })
      else if (this.loadedProvider === provider) {
        if (model.startsWith('draft:')) {
          const saved = this.confirmedModels.get(model)
          if (saved)
            this.store.setState({
              modelDraft: structuredClone(saved),
              editingModelId: model,
              modelLoaded: true,
              modelLoading: false,
            })
          else if (this.store.getState().modelDraft?.id !== model) this.navigate('configure', provider, null, true)
        } else void this.loadModel(model)
      }
    }
  }
  private navigate(step: 'choose' | 'configure', provider: string | null, model: string | null = null, replace = false) {
    const org = this.organization.getCurrentOrganization()
    if (!org) return
    const page = step === 'choose' ? 'choose-provider' : 'configure-provider'
    this.url.update(
      writeProviderUrl({
        ...this.store.getState(),
        step,
        selectedProviderId: provider,
        editingModelId: model ? (this.confirmedModels.get(model)?.catalogRef?.model ?? model) : null,
        providerQuery: this.query ?? '',
      }),
      {
        pathname: `/${encodeURIComponent(org.name)}/providers/new/${page}`,
        replace,
      },
    )
  }
  onProviderQueryChanged = (providerQuery: string) => {
    this.store.setState({ providerQuery })
    if (!providerQuery.trim() && this.query) this.onProviderSearch()
  }
  onProviderSearch = () => {
    const query = this.store.getState().providerQuery.trim()
    if (query === this.query) void this.loadProviders(false)
    else this.url.update({ q: query || null }, { replace: true })
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
    this.loadingTasks.get('providers')?.cancel()
    const loading = this.loading.providers.beginLoading('providers')
    this.loadingTasks.set('providers', loading)
    const query = this.query ?? ''
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
        return {
          providersLoaded: true,
          providers: [...rows.values()],
          providersHasMore: data.pageInfo.hasNextPage,
          providersLoadError: null,
        }
      })
    } catch {
      if (!owner.signal.aborted && !request.signal.aborted && this.listRequest === request)
        this.store.setState({ providersLoadError: 'Could not load provider templates. Try again.' })
    } finally {
      loading.complete()

      if (this.listRequest === request) this.store.setState({ providersLoading: false })
    }
  }
  onSelectProvider = (id: string) => this.navigate('configure', id)
  private back(step: 'choose' | 'configure', provider: string | null) {
    const org = this.organization.getCurrentOrganization()
    if (!org) return
    const page = step === 'choose' ? 'choose-provider' : 'configure-provider'
    const search = new URLSearchParams(this.url.store.getState().search)
    const patch = writeProviderUrl({ ...this.store.getState(), step, selectedProviderId: provider, editingModelId: null })
    for (const [key, value] of Object.entries(patch)) {
      if (value === null) search.delete(key)
      else search.set(key, value)
    }
    this.url.back(`/${encodeURIComponent(org.name)}/providers/new/${page}${search.size ? '?' + search : ''}`, {
      matchTarget: true,
    })
  }
  onChooseProvider = () => {
    this.departingModel = this.model
    this.back('choose', null)
  }
  private async loadProvider(id: string) {
    const owner = this.owner
    if (!owner) return
    this.providerRequest?.abort()
    const request = new AbortController()
    this.providerRequest = request
    this.loadingTasks.get('provider')?.cancel()
    const loading = this.loading.provider.beginLoading('provider')
    this.loadingTasks.set('provider', loading)
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
        providerLoaded: true,
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
      loading.complete()

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
    this.departingModel = undefined
    if (!this.provider) return
    this.navigate('configure', this.provider, id)
    if (id.startsWith('draft:')) {
      const draft = this.confirmedModels.get(id)
      if (draft)
        this.store.setState({ modelDraft: structuredClone(draft), editingModelId: id, modelLoaded: true, modelLoading: false })
      else this.onCancelModel()
    }
  }
  private async loadModel(id: string) {
    const owner = this.owner,
      provider = this.provider
    if (!owner || !provider) return
    this.modelRequest?.abort()
    const request = new AbortController()
    this.modelRequest = request
    this.loadingTasks.get('model')?.cancel()
    const loading = this.loading.model.beginLoading('model')
    this.loadingTasks.set('model', loading)
    const state = this.store.getState()
    const retained = state.modelLoaded && state.modelDraft?.id === id ? state.modelDraft : null
    const summary = state.models.find((v) => v.id === id)
    const pending: ModelDraft = retained ??
      summary ?? {
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
      modelLoaded: !!retained,
      modelLoading: true,
      catalogError: null,
    })
    try {
      const signal = AbortSignal.any([owner.signal, request.signal])
      const reference = this.confirmedModels.get(id)?.catalogRef?.model ?? id
      const [value, global] = await Promise.all([
        getModel(provider, reference, signal),
        reference.startsWith('custom:') ? Promise.resolve(null) : getGlobalModel(provider, reference, signal),
      ])
      if (signal.aborted || this.modelRequest !== request || this.provider !== provider || this.model !== id) return
      if (!value) throw new Error('missing model')
      const draft = { ...modelDraft(value, global), id }
      this.confirmedModels.set(id, structuredClone(draft))
      this.store.setState(retained ? { catalogError: null } : { modelDraft: draft, modelLoaded: true, catalogError: null })
    } catch {
      if (!owner.signal.aborted && !request.signal.aborted && this.modelRequest === request) {
        request.abort()
        this.store.setState({ catalogError: 'Could not load this model template. It may no longer be available. Try again.' })
      }
    } finally {
      loading.complete()

      if (this.modelRequest === request) this.store.setState({ modelLoading: false })
    }
  }
  onCancelModel = () => {
    this.departingModel = this.model
    if (this.provider) this.back('configure', this.provider)
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
  onAddModel: Controller['onAddModel'] = (protocol = 'openai_chat_completions') => {
    if (!this.provider) return
    const id = `draft:${crypto.randomUUID()}`
    const path = {
      openai_chat_completions: '/chat/completions',
      openai_responses: '/responses',
      anthropic_messages: '/messages',
    }[protocol]
    const draft: ModelDraft = {
      id,
      name: '',
      protocol,
      path,
      origin: 'CUSTOM',
      maxContextTokens: '',
      contextLimitEnabled: true,
      maxInputTokens: '',
      inputLimitEnabled: false,
      maxOutputTokens: '',
      outputLimitEnabled: false,
      inputPrice: '',
      outputPrice: '',
      cachedPrice: '',
      inputPriceEnabled: true,
      outputPriceEnabled: true,
      cachedPriceEnabled: false,
      capabilities: ['text'],
      thinkingLevels: [],
      effortMapping: {},
      headers: [],
      bodyDefaults: [],
    }
    this.navigate('configure', this.provider, id)
    this.model = id
    this.store.setState({ modelDraft: draft, editingModelId: id, modelLoaded: true, modelLoading: false })
  }
  onModelDraftChanged: Controller['onModelDraftChanged'] = (patch) => {
    const draft = this.store.getState().modelDraft
    if (draft && this.model === draft.id) this.store.setState({ modelDraft: { ...draft, ...patch } })
  }
  private modelIssuePath(path: PropertyKey[]): PropertyKey[] {
    if (path[0] === 'limits')
      return [
        'modelDraft',
        ({ context: 'maxContextTokens', input: 'maxInputTokens', output: 'maxOutputTokens' } as Record<string, string>)[
          String(path[1])
        ] ?? 'maxContextTokens',
      ]
    if (['inputPrice', 'outputPrice', 'cachedPrice'].includes(String(path[0]))) return ['modelDraft', path[0]]
    return ['modelDraft', ...path]
  }
  private commitSnapshot(draft: ModelDraft, fields?: string[]): ModelDraft {
    const baseline = this.confirmedModels.get(draft.id)
    if (!baseline || !fields || fields.includes('modelDraft')) return draft
    const names = new Set(fields.map((name) => name.replace(/^modelDraft\./, '').split('.')[0]))
    const limits = [
      'maxContextTokens',
      'maxInputTokens',
      'maxOutputTokens',
      'contextLimitEnabled',
      'inputLimitEnabled',
      'outputLimitEnabled',
    ]
    if (limits.some((key) => names.has(key))) for (const key of limits) names.add(key)
    for (const key of ['inputPrice', 'outputPrice', 'cachedPrice'])
      if (names.has(key) || names.has(`${key}Enabled`)) {
        names.add(key)
        names.add(`${key}Enabled`)
      }
    if (names.has('path')) names.add('protocol')
    if (names.has('effortMapping')) names.add('thinkingLevels')
    return { ...baseline, ...Object.fromEntries([...names].map((name) => [name, draft[name as keyof ModelDraft]])) }
  }
  validateModel: NonNullable<Controller['validateModel']> = (draft, fields) => {
    draft = this.commitSnapshot(draft, fields)
    const baseline = this.confirmedModels.get(draft.id)
    const patch = templateCustomization(draft, baseline)
    const result = baseline
      ? ModelTemplateCustomizeInput.safeParse(patch)
      : ModelTemplateCreateInput.safeParse({ configuration: patch })
    const issues: { path: PropertyKey[]; message: string }[] = result.success
      ? []
      : result.error.issues.map((issue) => ({
          ...issue,
          path: this.modelIssuePath(issue.path[0] === 'configuration' ? issue.path.slice(1) : issue.path),
        }))
    const defaults = draft.defaults
    const effective = {
      context: draft.contextLimitEnabled ? Number(draft.maxContextTokens) : Number(defaults?.maxContextTokens) || null,
      input: draft.inputLimitEnabled ? Number(draft.maxInputTokens) : Number(defaults?.maxInputTokens) || null,
      output: draft.outputLimitEnabled ? Number(draft.maxOutputTokens) : Number(defaults?.maxOutputTokens) || null,
    }
    if (patch.limits) {
      // Inherited values retain their catalog range; only overrides use editable bounds.
      const check = ModelTemplateEffectiveLimitsInput.safeParse(effective)
      if (!check.success)
        for (const issue of check.error.issues) {
          const field = String(issue.path[0]) as 'context' | 'input' | 'output'
          const enabled =
            field === 'context'
              ? draft.contextLimitEnabled
              : field === 'input'
                ? draft.inputLimitEnabled
                : draft.outputLimitEnabled
          if (enabled || issue.code === 'custom')
            issues.push({
              ...issue,
              path: [
                'modelDraft',
                !enabled && draft.contextLimitEnabled
                  ? 'maxContextTokens'
                  : ({ context: 'maxContextTokens', input: 'maxInputTokens', output: 'maxOutputTokens' }[field] ??
                    'maxContextTokens'),
              ],
            })
        }
    }
    return issues.length ? issues : undefined
  }
  onSaveModel: Controller['onSaveModel'] = async (draft, fields) => {
    draft = this.commitSnapshot(draft, fields)
    const provider = this.provider,
      owner = this.owner
    if (!provider || !owner || this.model !== draft.id) return false
    const baseline = this.confirmedModels.get(draft.id)
    const input = ModelTemplateCustomizeInput.parse(templateCustomization(draft, baseline))
    if (!Object.keys(input).length) return draft
    const reference = baseline?.catalogRef?.model
    const write = (async () => {
      const response = reference
        ? await graphql(owner.signal)
            .mutation({
              modelTemplateCustomize: { __args: { providerTemplateId: provider, id: reference, input }, node: modelDetail },
            } satisfies MutationGenqlSelection)
            .then((v) => v.modelTemplateCustomize.node)
        : await graphql(owner.signal)
            .mutation({
              modelTemplateCreate: {
                __args: { providerTemplateId: provider, input: { configuration: input } },
                node: modelDetail,
              },
            } satisfies MutationGenqlSelection)
            .then((v) => v.modelTemplateCreate.node)
      const saved = { ...draft, ...modelDraft(response), id: draft.id, defaults: draft.defaults ?? baseline?.defaults }
      for (const key of [
        'contextLimitEnabled',
        'inputLimitEnabled',
        'outputLimitEnabled',
        'inputPriceEnabled',
        'outputPriceEnabled',
        'cachedPriceEnabled',
      ] as const)
        saved[key] = draft[key]
      if (owner.signal.aborted || this.provider !== provider) return saved
      this.confirmedModels.set(draft.id, structuredClone(saved))
      this.store.setState((state) => ({
        models: state.models.some((v) => v.id === draft.id)
          ? state.models.map((v) => (v.id === draft.id ? { ...v, ...saved } : v))
          : [...state.models, { ...saved, enabled: true, modes: [] }],
      }))
      if (!reference) {
        this.localModelIds.set(response.id, draft.id)
        if (this.model === draft.id && this.departingModel !== draft.id)
          this.url.update({ model: response.id }, { replace: true })
      }
      return saved
    })()
    this.writes.set(draft.id, write)
    try {
      return await write
    } catch (error) {
      if (error instanceof ApiError && this.model === draft.id)
        for (const issue of error.validation) {
          let path = issue.path
          if (path[0] === 'limits')
            path = [
              { context: 'maxContextTokens', input: 'maxInputTokens', output: 'maxOutputTokens' }[path[1] as 'context'] ??
                'maxContextTokens',
            ]
          if (['inputPrice', 'outputPrice', 'cachedPrice'].includes(path[0])) path = [path[0]]
          this.store.form.api.setError(this.modelIssuePath(path).join('.'), { type: 'server', message: issue.message })
        }
      if (!owner.signal.aborted) notify.error(error)
      throw error
    } finally {
      if (this.writes.get(draft.id) === write) this.writes.delete(draft.id)
    }
  }
  onResetModel = async () => {
    try {
      const draft = this.store.getState().modelDraft,
        provider = this.provider
      if (!draft || !provider) return
      await this.writes.get(draft.id)?.catch(() => undefined)
      const reference = this.confirmedModels.get(draft.id)?.catalogRef?.model ?? draft.id
      const value = (
        await graphql(this.owner?.signal).mutation({
          modelTemplateReset: { __args: { providerTemplateId: provider, id: reference }, node: modelDetail },
        } satisfies MutationGenqlSelection)
      ).modelTemplateReset.node
      const next = { ...modelDraft(value), id: draft.id }
      this.confirmedModels.set(draft.id, structuredClone(next))
      this.store.form.set('modelDraft', next)
      this.store.setState((state) => ({ models: state.models.map((v) => (v.id === draft.id ? { ...v, ...next } : v)) }))
    } catch (error) {
      notify.error(error)
      throw error
    }
  }
  onDeleteModel = async () => {
    try {
      const draft = this.store.getState().modelDraft,
        provider = this.provider
      if (!draft || !provider) return
      await this.writes.get(draft.id)?.catch(() => undefined)
      const reference = this.confirmedModels.get(draft.id)?.catalogRef?.model
      if (reference)
        await graphql(this.owner?.signal).mutation({
          modelTemplateDelete: { __args: { providerTemplateId: provider, id: reference }, node: true },
        } satisfies MutationGenqlSelection)
      this.confirmedModels.delete(draft.id)
      this.store.setState((state) => ({ models: state.models.filter((v) => v.id !== draft.id) }))
      this.onCancelModel()
    } catch (error) {
      notify.error(error)
      throw error
    }
  }
  onTestConnection = () => {}
  onCreateProvider: Controller['onCreateProvider'] = async (values) => {
    const state = { ...this.store.getState(), ...values }
    if (state.submitting || !this.provider) return false
    if (state.avatarFile && this.uploadedLogo?.file !== state.avatarFile) {
      notify.error(new ApiError('Wait for the logo upload to finish or choose the provider logo.', 400, 'BAD_USER_INPUT'))
      return false
    }
    const input = ProviderCreateFromTemplateInput.parse({
      name: state.name,
      providerTemplateId: this.provider,
      baseUrl: state.baseUrl,
      apiKey: state.apiKey,
      headers: state.headers.map(({ key, value }) => ({ key, value })),
      models: state.models
        .filter((v) => v.enabled)
        .map((v) => ({
          id: this.confirmedModels.get(v.id)?.catalogRef?.model ?? v.catalogRef?.model ?? v.id,
          protocol: TextModelProtocol.parse((v.protocol ?? 'openai_chat_completions').toUpperCase()),
        })),
      logoUrl: state.avatarUrl,
      logoUploadId: this.uploadedLogo?.file === state.avatarFile ? this.uploadedLogo?.id : undefined,
    })
    this.store.setState({ submitting: true })
    try {
      const result = await graphql(this.owner?.signal).mutation({
        providerCreate: { __args: { input }, node: { id: true, name: true, logoUrl: true } },
      } satisfies MutationGenqlSelection)
      this.store.setState({ apiKey: '', avatarFile: null })
      this.organization.onProviderCreated?.(result.providerCreate.node)
      return { apiKey: '' }
    } catch (error) {
      notify.error(error)
      return false
    } finally {
      this.store.setState({ submitting: false })
    }
  }
  onAvatarChanged: NonNullable<Controller['onAvatarChanged']> = (file) => {
    if (!file) return
    void (async () => {
      const response = await graphql(this.owner?.signal).mutation({
        providerLogoUpload: {
          __args: {
            input: {
              size: file.size,
              mediaType: ({ 'image/png': 'PNG', 'image/jpeg': 'JPEG', 'image/webp': 'WEBP' } as const)[
                file.type as 'image/png'
              ],
            },
          },
          node: { id: true, upload: { url: true, fields: { name: true, value: true } } },
        },
      } satisfies MutationGenqlSelection)
      const grant = response.providerLogoUpload.node,
        form = new FormData()
      for (const { name, value } of grant.upload.fields) form.append(name, value)
      form.append('file', file)
      const uploaded = await fetch(grant.upload.url, { method: 'POST', body: form, signal: this.owner?.signal })
      if (!uploaded.ok) throw new Error('The logo upload failed.')
      this.uploadedLogo = { file, id: grant.id }
    })().catch((error) => notify.error(error))
  }
  onSaveProvider = () => false as const
  onCancelProvider = () => {
    this.store.setState({ apiKey: '', headers: [], avatarFile: null })
    this.onChooseProvider()
  }
  onCloneProvider = () => {}
}
