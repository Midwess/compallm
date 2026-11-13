import {
  createUserPreferenceStore,
  initialPreferenceDraft,
  loadedPreferenceDraft,
  confirmedPreferenceDraft,
  parsePreferencePatch,
  preferenceSelection,
  type UserPreferenceController as Contract,
  type PreferenceDraft,
} from '@compallm/storybook/pages/settings'
import { appController } from '../../app/controller'
import { appearanceController } from '../../appearance/controller'
import type { AppController, AppLoadingTask } from '@compallm/storybook'
import type { OrganizationController } from '../../organization/controller'
import type { UrlController } from '@compallm/storybook'
import type { AuthController } from '@compallm/storybook'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { ApiError, graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'

export class UserPreferenceController implements Contract {
  readonly store = createUserPreferenceStore()
  private scope = new AbortController()
  private read?: AbortController
  private loading?: AppLoadingTask
  private navigation?: Pick<OrganizationController, 'onOpenHome' | 'setSidebarOrder' | 'setSidebarLayout'>
  private readonly app?: Pick<AppController, 'beginLoading'>
  constructor(app?: Pick<AppController, 'beginLoading'>) {
    this.app = app
  }

  connectOrganization = (
    navigation: Pick<OrganizationController, 'onOpenHome' | 'setSidebarOrder' | 'setSidebarLayout'>,
    url: Pick<UrlController, 'store'>,
  ) => {
    this.navigation = navigation
    const unsubscribe = url.store.subscribe(this.applyHomeView)
    this.applySettings()
    return () => {
      unsubscribe()
      if (this.navigation === navigation) this.navigation = undefined
    }
  }

  private applySettings = () => {
    const { saved, status } = this.store.getState()
    if (!saved || status !== 'ready') return
    this.applyHomeView()
    this.applySidebarOrder()
    appearanceController.setTextSize(saved.typography.size)
  }

  private applyHomeView = () => {
    const { saved, status } = this.store.getState()
    if (saved && status === 'ready') this.navigation?.onOpenHome(saved.navigation.home)
  }

  private applySidebarOrder = () => {
    const { saved, status } = this.store.getState()
    if (saved && status === 'ready') this.navigation?.setSidebarOrder(saved.sidebar.entries.map(({ item }) => item))
    if (saved && status === 'ready') this.navigation?.setSidebarLayout(saved.sidebar.layout)
  }

  start = (auth: Pick<AuthController, 'store'>) => {
    let userId: string | null | undefined
    const synchronize = () => {
      const { currentUser, status } = auth.store.getState()
      if (status === 'checking' || status === 'unknown') return
      const next = currentUser?.userId ?? null
      if (next === userId) return
      userId = next
      this.cancel()
      this.store.form.reset({ draft: initialPreferenceDraft() })
      this.store.setState({ saved: null, status: 'idle' })
      appearanceController.setTextSize('DEFAULT')
      if (next) void this.load()
    }
    const unsubscribe = auth.store.subscribe(synchronize)
    synchronize()
    return () => {
      unsubscribe()
      this.cancel()
    }
  }

  cancel = () => {
    this.scope.abort()
    this.read?.abort()
    this.loading?.cancel()
    this.read = undefined
    this.store.form.stop()
    this.store.setState({ status: this.store.getState().saved ? 'ready' : 'idle' })
  }

  load = async () => {
    if (this.scope.signal.aborted) this.scope = new AbortController()
    if (this.store.form.dirty || this.store.form.pending) return
    const scope = this.scope
    this.read?.abort()
    this.loading?.cancel()
    const request = new AbortController()
    this.read = request
    const loading = this.app?.beginLoading('user-preferences')
    this.loading = loading
    this.store.setState({ status: 'loading' })
    try {
      const selection = { userPreferences: { node: preferenceSelection } } satisfies QueryGenqlSelection
      const {
        userPreferences: { node },
      } = await graphql(AbortSignal.any([scope.signal, request.signal])).query(selection)
      if (scope.signal.aborted || this.read !== request) {
        loading?.cancel()
        return
      }
      const draft = loadedPreferenceDraft(node, this.store.form.values.draft!.codeLanguage)
      this.store.form.reset({ draft })
      this.store.setState({ saved: node, status: 'ready' })
      this.applySettings()
      loading?.complete()
    } catch (error) {
      if (scope.signal.aborted || this.read !== request) {
        loading?.cancel()
        return
      }
      this.store.setState({ status: error instanceof ApiError && [401, 403].includes(error.status) ? 'denied' : 'error' })
      loading?.fail(error, () => this.load())
      if (!this.app)
        notify.apiError(error, () => {
          if (!scope.signal.aborted && this.read === request) void this.load()
        })
    }
  }

  update = async (patch: Partial<PreferenceDraft>, signal: AbortSignal) => {
    const parsed = parsePreferencePatch(patch)
    if (!parsed.input) throw new Error('Invalid preference patch')
    const categories = Object.keys(parsed.input) as (keyof typeof preferenceSelection)[]
    if (!categories.length) return patch
    const scope = this.scope
    if (scope.signal.aborted || this.store.getState().status !== 'ready') throw new DOMException('Cancelled', 'AbortError')
    this.read?.abort()
    this.loading?.cancel()
    this.read = undefined
    const nodeSelection: Partial<typeof preferenceSelection> = {}
    for (const category of categories) Object.assign(nodeSelection, { [category]: preferenceSelection[category] })
    const selection = {
      userPreferencesUpdate: { __args: { input: parsed.input }, node: nodeSelection },
    } satisfies MutationGenqlSelection
    const {
      userPreferencesUpdate: { node },
    } = await graphql(AbortSignal.any([signal, scope.signal])).mutation(selection)
    if (scope.signal.aborted || signal.aborted || this.scope !== scope) throw new DOMException('Cancelled', 'AbortError')
    const saved = this.store.getState().saved
    if (!saved) throw new Error('Preferences have not been loaded')
    const confirmed = { ...saved, ...node }
    this.store.setState({ saved: confirmed })
    if (categories.includes('navigation')) this.applyHomeView()
    if (categories.includes('sidebar')) this.applySidebarOrder()
    if (categories.includes('typography')) appearanceController.setTextSize(confirmed.typography.size)
    return confirmedPreferenceDraft(confirmed, patch)
  }
}

export const userPreferenceController = new UserPreferenceController(appController)
