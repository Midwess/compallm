import {
  createSettingsStore,
  type SettingsController as Controller,
  type SettingsDestination,
  type UserPreferenceController,
} from '@compallm/storybook/pages/settings'
import type { UrlController, AuthController } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import { codeExamples } from './code-examples'
import { GroupController } from './group/controller'
import { graphql } from '../graphql-client'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { RoleCreateInput } from '@compallm/contract/validation'
import { createSettingsPages } from './pages'

export const settingsPages = [
  'blank',
  'general',
  'messaging',
  'code',
  'profile',
  'security',
  'api-keys',
  'danger',
  'organization',
  'members',
  'new-group',
  'usage-limits',
  'member-limits',
  'usage-explorer',
  'audit',
] as const satisfies readonly SettingsDestination[]

export class SettingsController implements Controller {
  readonly store = createSettingsStore({ readonly: true, codeExamples })
  readonly pages: ReturnType<typeof createSettingsPages>
  readonly role: ReturnType<typeof createSettingsPages>['role']
  readonly group = new GroupController()
  private readonly url: Pick<UrlController, 'navigate'>
  private readonly auth: Pick<AuthController, 'signOut'> & Partial<Pick<AuthController, 'store'>>
  private readonly base: string
  private request?: AbortController
  private stopProfile?: () => void
  private generation = 0
  private creatingRole = false
  private navigationGeneration = 0
  private readonly knownEmptyRoles = new Set<string>()
  private readonly deletedRoles = new Set<string>()

  start = () => {
    this.stopProfile?.()
    this.stopProfile = undefined
    const controller = new AbortController()
    this.request?.abort()
    this.request = controller
    const generation = ++this.generation
    this.store.setState({ roles: [], groups: [] })
    void this.loadPermissions(controller.signal, generation)
    const location = this.store.getState().location
    if (location.page === 'profile') this.stopProfile = this.pages.profile.start()
    if (location.page === 'role' && location.id) void this.role.load(location.id)
    return () => {
      this.stopProfile?.()
      this.stopProfile = undefined
      controller.abort()
      this.role.cancel()
      if (this.request === controller) this.request = undefined
      this.generation += 1
    }
  }

  private async loadPermissions(signal: AbortSignal, generation: number) {
    try {
      const selection = {
        roles: { edges: { node: { id: true, name: true, color: true } } },
        groups: { edges: { node: { id: true, name: true, description: true } } },
      } satisfies QueryGenqlSelection
      const { roles, groups } = await graphql(signal).query(selection)
      if (signal.aborted || generation !== this.generation) return
      this.store.setState((state) => ({
        roles: [
          ...roles.edges
            .filter(({ node }) => !this.deletedRoles.has(node.id))
            .map(({ node: role }) => ({ ...role, description: '', source: '' })),
          ...state.roles.filter(
            (role) => this.knownEmptyRoles.has(role.id) && !roles.edges.some(({ node }) => node.id === role.id),
          ),
        ],
        groups: groups.edges.map(({ node: group }) => ({ ...group, roleIds: [], memberIds: [], color: '' })),
      }))
    } catch (error) {
      if (!signal.aborted && generation === this.generation) {
        notify.apiError(error, () => {
          if (!signal.aborted && generation === this.generation) void this.loadPermissions(signal, generation)
        })
      }
    }
  }

  constructor(
    organizationName: string,
    url: Pick<UrlController, 'navigate'>,
    auth: Pick<AuthController, 'signOut'> & Partial<Pick<AuthController, 'store'>>,
    page: SettingsDestination,
    preferences: UserPreferenceController,
  ) {
    this.pages = createSettingsPages(
      this.store,
      (id) => {
        this.knownEmptyRoles.delete(id)
        this.deletedRoles.add(id)
        this.store.setState((state) => ({ roles: state.roles.filter((role) => role.id !== id) }))
        const location = this.store.getState().location
        if (location.page === 'role' && location.id === id) this.navigate('blank')
      },
      preferences,
      auth,
    )
    this.role = this.pages.role
    this.url = url
    this.auth = auth
    this.base = `/${encodeURIComponent(organizationName)}`
    this.setPage(page)
  }

  setPage = (page: SettingsDestination, id?: string) => {
    if (page !== 'profile') {
      this.stopProfile?.()
      this.stopProfile = undefined
    }
    this.navigationGeneration += 1
    this.role.cancel()
    this.store.setState({ location: { page, id } })
    if (page === 'profile' && this.request && !this.stopProfile) this.stopProfile = this.pages.profile.start()
    if (page === 'role' && id && this.request) void this.role.load(id)
    if (page === 'new-group') this.group.open()
  }
  private createRole = async () => {
    if (this.creatingRole || !this.request) return
    this.creatingRole = true
    const signal = this.request.signal
    const generation = this.generation
    const navigationGeneration = this.navigationGeneration
    const time = new Date().toLocaleTimeString('en-GB', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      fractionalSecondDigits: 3,
      hour12: false,
    })
    const input = RoleCreateInput.parse({ name: `New role at ${time}`, description: '', cedarSource: '' })
    try {
      const selection = {
        roleCreate: { __args: { input }, node: { id: true, name: true, description: true, color: true } },
      } satisfies MutationGenqlSelection
      const {
        roleCreate: { node },
      } = await graphql(signal).mutation(selection)
      if (signal.aborted || generation !== this.generation) return
      this.knownEmptyRoles.add(node.id)
      this.store.setState((state) => ({
        roles: [...state.roles.filter((role) => role.id !== node.id), { ...node, source: '' }],
      }))
      if (navigationGeneration === this.navigationGeneration) {
        this.url.navigate(`${this.base}/settings/role/${encodeURIComponent(node.id)}`)
      }
    } catch (error) {
      if (!signal.aborted && generation === this.generation) notify.error(error)
    } finally {
      this.creatingRole = false
    }
  }
  navigate = (page: SettingsDestination, id?: string) => {
    if (page === 'new-role') void this.createRole()
    else if (page === 'role' && id) {
      this.navigationGeneration += 1
      this.url.navigate(`${this.base}/settings/role/${encodeURIComponent(id)}`)
    } else if (settingsPages.some((entry) => entry === page)) {
      this.navigationGeneration += 1
      this.url.navigate(`${this.base}/settings/${page}`)
    } else this.unavailable()
  }
  setNavigationQuery = (navigationQuery: string) => this.store.setState({ navigationQuery })
  private unavailable = () => {
    notify.info('Settings changes are not available yet.')
  }
  signOut = () => {
    void this.auth.signOut()
  }
  leaveSettings = () => this.url.navigate(`${this.base}/chat`)
  apiError = (error: unknown, retry?: () => void) => {
    if (retry) notify.apiError(error, retry)
    else notify.error(error)
  }
  dismissErrorToast = () => this.store.setState({ errorToast: null })
  dismissNotice = () => this.store.setState({ notice: null })
}
