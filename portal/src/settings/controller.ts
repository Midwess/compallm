import { permissionKey, type PermissionController } from '@compallm/storybook'
import { requirePermissions } from '../permission/controller'
import {
  createSettingsStore,
  type SettingsController as Controller,
  type SettingsDestination,
  type UserPreferenceController,
} from '@compallm/storybook/pages/settings'
import type { UrlController, AuthController } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import { codeExamples } from './code-examples'
import { graphql } from '../graphql-client'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { RoleCreateInput, GroupCreateInput } from '@compallm/contract/validation'
import { createSettingsPages } from './pages'

export const settingsPages = [
  'blank',
  'general',
  'messaging',
  'code',
  'profile',
  'access',
  'security',
  'api-keys',
  'organization',
  'organization-onboarding',
  'members',
  'usage-limits',
  'member-limits',
  'usage-explorer',
  'audit',
] as const satisfies readonly SettingsDestination[]

export class SettingsController implements Controller {
  private readonly permissions?: PermissionController
  readonly store = createSettingsStore({ readonly: true, codeExamples })
  readonly pages: ReturnType<typeof createSettingsPages>
  readonly role: ReturnType<typeof createSettingsPages>['role']
  readonly group: ReturnType<typeof createSettingsPages>['group']
  private readonly url: Pick<UrlController, 'navigate'> & Partial<Pick<UrlController, 'store'>>
  private readonly auth: Pick<AuthController, 'signOut'> & Partial<Pick<AuthController, 'store'>>
  private readonly initialName: string
  private stopOrganization?: () => void

  private get base() {
    return `/${encodeURIComponent(this.auth.store?.getState().currentUser?.organizationName ?? this.initialName)}`
  }
  private request?: AbortController
  private stopProfile?: () => void
  private generation = 0
  private creatingGroup = false
  private readonly deletedGroups = new Set<string>()
  private readonly createdGroups = new Set<string>()
  private creatingRole = false
  private navigationGeneration = 0
  private readonly knownEmptyRoles = new Set<string>()
  private readonly deletedRoles = new Set<string>()

  start = () => {
    this.stopProfile?.()
    this.stopProfile = undefined
    this.stopOrganization?.()
    this.stopOrganization = undefined
    const controller = new AbortController()
    this.request?.abort()
    this.request = controller
    const generation = ++this.generation
    this.store.setState({ roles: [], groups: [] })
    void this.loadPermissions(controller.signal, generation)
    const location = this.store.getState().location
    if (location.page === 'organization') this.stopOrganization = this.pages.organization.start()
    if (location.page === 'profile') this.stopProfile = this.pages.profile.start()
    if (location.page === 'role' && location.id) void this.role.load(location.id)
    if (location.page === 'group' && location.id) void this.group.load(location.id)
    return () => {
      this.stopProfile?.()
      this.stopProfile = undefined
      this.stopOrganization?.()
      this.stopOrganization = undefined
      controller.abort()
      this.role.cancel()
      this.group.cancel()
      if (this.request === controller) this.request = undefined
      this.generation += 1
    }
  }

  private async loadPermissions(signal: AbortSignal, generation: number) {
    try {
      const checks = (['ROLE_READ', 'GROUP_READ'] as const).map((action) => ({ action, resourceType: 'ORGANIZATION' as const }))
      if (this.permissions) await this.permissions.ensure(checks)
      if (signal.aborted || generation !== this.generation) return
      const allowed = (index: number) =>
        !this.permissions || this.permissions.store.getState().entries[permissionKey(checks[index])]?.status === 'allowed'
      const selection = {
        ...(allowed(0) ? { roles: { edges: { node: { id: true, name: true, color: true } } } } : {}),
        ...(allowed(1) ? { groups: { edges: { node: { id: true, name: true } } } } : {}),
      } satisfies QueryGenqlSelection
      if (!Object.keys(selection).length) {
        this.store.setState({ roles: [], groups: [] })
        return
      }
      const result = await graphql(signal).query(selection)
      const roles = result.roles ?? { edges: [] }
      const groups = result.groups ?? { edges: [] }
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
        groups: [
          ...groups.edges
            .filter(({ node }) => !this.deletedGroups.has(node.id))
            .map(
              ({ node }) =>
                state.groups.find((group) => group.id === node.id) ?? {
                  ...node,
                  description: '',
                  roleIds: [],
                  memberIds: [],
                  color: '',
                },
            ),
          ...state.groups.filter(
            (group) => this.createdGroups.has(group.id) && !groups.edges.some(({ node }) => node.id === group.id),
          ),
        ],
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
    url: Pick<UrlController, 'navigate'> & Partial<Pick<UrlController, 'store'>>,
    auth: Pick<AuthController, 'signOut'> & Partial<Pick<AuthController, 'store'>>,
    page: SettingsDestination,
    preferences: UserPreferenceController,
    permissions?: PermissionController,
  ) {
    this.permissions = permissions
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
      this.organizationRenamed,
      this.groupDeleted,
      permissions,
    )
    this.role = this.pages.role
    this.group = this.pages.group
    this.url = url
    this.auth = auth
    this.initialName = organizationName
    this.setPage(page)
  }

  private organizationRenamed = (name: string) => {
    const user = this.auth.store?.getState().currentUser
    if (!user || user.organizationName === name) return
    const previous = this.base
    this.auth.store!.setState({ currentUser: { ...user, organizationName: name } })
    const location = this.url.store?.getState()
    if (location && (location.pathname === previous || location.pathname.startsWith(previous + '/'))) {
      this.url.navigate(this.base + location.pathname.slice(previous.length) + location.search + location.hash, {
        replace: true,
      })
    }
  }

  setPage = (page: SettingsDestination, id?: string) => {
    if (page !== 'organization') {
      this.stopOrganization?.()
      this.stopOrganization = undefined
    }
    if (page !== 'profile') {
      this.stopProfile?.()
      this.stopProfile = undefined
    }
    this.navigationGeneration += 1
    this.role.cancel()
    this.group.cancel()
    this.store.setState({ location: { page, id } })
    if (page === 'organization' && this.request && !this.stopOrganization)
      this.stopOrganization = this.pages.organization.start()
    if (page === 'profile' && this.request && !this.stopProfile) this.stopProfile = this.pages.profile.start()
    if (page === 'role' && id && this.request) void this.role.load(id)
    if (page === 'group' && id && this.request) void this.group.load(id)
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
      if (this.permissions)
        await requirePermissions(this.permissions, [
          { action: 'ROLE_READ', resourceType: 'ORGANIZATION' },
          { action: 'ROLE_CREATE', resourceType: 'ORGANIZATION' },
        ])
      if (signal.aborted || generation !== this.generation) return
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
  private groupDeleted = (id: string) => {
    this.deletedGroups.add(id)
    this.createdGroups.delete(id)
    this.store.setState((state) => ({ groups: state.groups.filter((group) => group.id !== id) }))
    if (this.request && this.store.getState().location.page === 'group' && this.store.getState().location.id === id)
      this.navigate('blank')
  }
  private createGroup = async () => {
    if (this.creatingGroup || !this.request) return
    this.creatingGroup = true
    const request = this.request,
      generation = this.generation,
      navigation = this.navigationGeneration
    const time = new Date().toLocaleTimeString('en-GB', {
      hour: '2-digit',
      minute: '2-digit',
      second: '2-digit',
      fractionalSecondDigits: 3,
      hour12: false,
    })
    const input = GroupCreateInput.parse({ name: `New group at ${time}`, description: '', roleIds: [] })
    try {
      if (this.permissions)
        await requirePermissions(this.permissions, [
          { action: 'GROUP_READ', resourceType: 'ORGANIZATION' },
          { action: 'GROUP_CREATE', resourceType: 'ORGANIZATION' },
        ])
      if (request.signal.aborted || generation !== this.generation) return
      const selection = { groupCreate: { __args: { input }, node: { id: true, name: true } } } satisfies MutationGenqlSelection
      const {
        groupCreate: { node },
      } = await graphql(request.signal).mutation(selection)
      if (request.signal.aborted || generation !== this.generation) return
      this.createdGroups.add(node.id)
      this.store.setState((state) => ({
        groups: [
          ...state.groups.filter((group) => group.id !== node.id),
          { ...node, description: '', roleIds: [], memberIds: [], color: '' },
        ],
      }))
      if (navigation === this.navigationGeneration)
        this.url.navigate(`${this.base}/settings/group/${encodeURIComponent(node.id)}`)
    } catch (error) {
      if (!request.signal.aborted)
        notify.apiError(error, () => {
          if (this.request === request && this.navigationGeneration === navigation) void this.createGroup()
        })
    } finally {
      this.creatingGroup = false
    }
  }
  navigate = (page: SettingsDestination, id?: string) => {
    if (page === 'new-group') void this.createGroup()
    else if (page === 'new-role') void this.createRole()
    else if ((page === 'role' || page === 'group') && id) {
      this.navigationGeneration += 1
      this.url.navigate(`${this.base}/settings/${page}/${encodeURIComponent(id)}`)
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
