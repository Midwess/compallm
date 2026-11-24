import type { AuthController } from '@compallm/storybook/pages/auth'
import {
  SettingsMeAndOrganizationAndRolesAndGroupsStore,
  SettingsSessionsStore,
  SettingsSessionRevokeOtherStore,
  SettingsSessionRevokeOthersStore,
  SettingsRoleCreateStore,
  SettingsGroupCreateStore,
} from '@compallm/contract/houdini'
import { permissionKey, type PermissionController } from '@compallm/storybook'
import { requirePermissions } from '../permission/controller'
import {
  createSettingsStore,
  describeSessionBrowser,
  type SettingsController as Controller,
  type SettingsDestination,
  type UserPreferenceController,
} from '@compallm/storybook/pages/settings'
import type { UrlController } from '@compallm/storybook'
import { notify } from '../notifications/toast'
import { codeExamples } from './code-examples'
import { graphqlClient, requireData } from '../graphql-client'

import { RoleCreateInput, GroupCreateInput } from '@compallm/contract/validation'
import { createSettingsPages } from './pages'
import { lookupSessionLocation } from './session-location'

import { settingsPages } from './destinations'
export { settingsPages } from './destinations'

export class SettingsController implements Controller {
  reopenOnboarding?: () => void
  private readonly permissions?: PermissionController
  readonly store = createSettingsStore({
    readonly: true,
    codeExamples,
    navigationLoaded: false,
    membersLoaded: false,
    invitationsLoaded: false,
    sessionsLoaded: false,
    profileLoaded: false,
    organizationProfileLoaded: false,
  })
  readonly pages: ReturnType<typeof createSettingsPages>
  readonly role: ReturnType<typeof createSettingsPages>['role']
  readonly group: ReturnType<typeof createSettingsPages>['group']
  private readonly url: Pick<UrlController, 'navigate'> & Partial<Pick<UrlController, 'store'>>
  private readonly auth: Pick<AuthController, 'signOut'> & Partial<Pick<AuthController, 'store'>>
  private readonly initialName: string
  private stopOrganization?: () => void
  private locationRequest?: AbortController
  private sessionsRequest?: AbortController
  private sessionsCursor: string | null = null
  private readonly revokedSessions = new Set<string>()

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
    const stopDialogs = this.pages.dialogs.start()
    this.stopProfile?.()
    this.stopProfile = undefined
    this.stopOrganization?.()
    this.stopOrganization = undefined
    const controller = new AbortController()
    this.request?.abort()
    this.request = controller
    this.locationRequest?.abort()
    this.locationRequest = undefined
    this.store.setState({ sessionLocation: null })
    const generation = ++this.generation
    this.store.setState({ navigationLoading: true, navigationLoadError: false })
    if (this.store.getState().location.page === 'members') this.pages.members.start()
    if (this.navigationSeeded) {
      this.navigationSeeded = false
      this.store.setState({ navigationLoading: false })
    } else void this.loadPermissions(controller.signal, generation)
    const location = this.store.getState().location
    if (location.page === 'organization') this.stopOrganization = this.pages.organization.start()
    if (location.page === 'profile') this.stopProfile = this.pages.profile.start()
    if (location.page === 'access' || location.page === 'security') {
      void this.loadSessionLocation()
      void this.loadSessions()
    }
    if (location.page === 'role' && location.id) void this.role.load(location.id)
    if (location.page === 'group' && location.id) void this.group.load(location.id)
    return () => {
      stopDialogs()
      this.stopProfile?.()
      this.stopProfile = undefined
      this.stopOrganization?.()
      this.stopOrganization = undefined
      controller.abort()
      this.sessionsRequest?.abort()
      this.sessionsRequest = undefined
      this.locationRequest?.abort()
      this.locationRequest = undefined
      this.store.setState({ sessionLocation: null })
      this.role.cancel()
      this.group.cancel()
      this.pages.onboarding.cancel()
      this.pages.members.cancel()
      if (this.request === controller) this.request = undefined
      this.generation += 1
    }
  }

  reloadNavigation = () => {
    if (this.request && !this.request.signal.aborted) void this.loadPermissions(this.request.signal, this.generation)
  }

  private async loadPermissions(signal: AbortSignal, generation: number) {
    this.store.setState({ navigationLoading: true, navigationLoadError: false })
    try {
      const checks = (['role.read', 'group.read', 'member.read'] as const).map((action) => ({
        action,
        resourceType: 'Organization' as const,
      }))
      if (this.permissions) await this.permissions.evaluate(checks)
      if (signal.aborted || generation !== this.generation) return
      const allowed = (index: number) =>
        !this.permissions || this.permissions.store.getState().entries[permissionKey(checks[index])]?.status === 'allowed'

      const result = await new SettingsMeAndOrganizationAndRolesAndGroupsStore()
        .fetch({
          variables: { includeFields: allowed(2), includeFields2: allowed(0), includeFields3: allowed(1) },
          fetch: graphqlClient.fetch(signal),
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
      this.adoptNavigation(result)
    } catch (error) {
      if (!signal.aborted && generation === this.generation) {
        this.store.setState({ navigationLoadError: true })
        notify.apiError(error, () => {
          if (!signal.aborted && generation === this.generation) void this.loadPermissions(signal, generation)
        })
      }
    } finally {
      if (!signal.aborted && generation === this.generation) this.store.setState({ navigationLoading: false })
    }
  }

  seedNavigation(result: import('@compallm/contract/houdini').SettingsMeAndOrganizationAndRolesAndGroups$result) {
    this.adoptNavigation(result)
    this.navigationSeeded = true
    this.store.setState({ navigationLoading: false })
  }
  private navigationSeeded = false
  private adoptNavigation(result: import('@compallm/contract/houdini').SettingsMeAndOrganizationAndRolesAndGroups$result) {
    const roles = result.roles ?? { edges: [] }
    const groups = result.groups ?? { edges: [] }
    this.permissions?.setFacts?.(
      groups.edges.flatMap(({ node }) =>
        node.roles
          ? [
              {
                resourceType: 'Group' as const,
                id: node.id,
                roleIds: node.roles.edges.map(({ node }) => node.id),
              },
            ]
          : [],
      ),
    )
    this.store.setState((state) => ({
      navigationLoaded: true,
      currentMemberId: result.me?.node.id,
      actorIsOwner: result.me?.node.organizationOwner ?? false,
      ownerMemberIds: result.organization?.node.ownerMemberIds ?? [],
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
                roleIds: node.roles?.edges.map(({ node }) => node.id) ?? [],
                memberIds: [],
                color: '',
              },
          ),
        ...state.groups.filter(
          (group) => this.createdGroups.has(group.id) && !groups.edges.some(({ node }) => node.id === group.id),
        ),
      ],
    }))
  }
  seedSessions(result: import('@compallm/contract/houdini').SettingsSessions$result) {
    const user = this.auth.store?.getState().currentUser
    this.sessionsCursor = result.sessions.pageInfo.endCursor ?? null
    this.store.setState({
      sessionsLoaded: true,
      sessionsLoading: false,
      sessionsHasNextPage: result.sessions.pageInfo.hasNextPage,
      otherSessions: result.sessions.edges
        .filter(({ node }) => node.id !== user?.id)
        .map(({ node }) => ({
          id: node.id,
          ipAddress: node.ipAddress,
          browser: describeSessionBrowser(node.userAgent),
          signedInAt: node.signedInAt,
          lastSeenAt: node.lastSeenAt,
          location: null,
        })),
    })
    this.sessionsSeeded = true
  }
  private sessionsSeeded = false
  constructor(
    organizationName: string,
    url: Pick<UrlController, 'navigate'> & Partial<Pick<UrlController, 'store'>>,
    auth: Pick<AuthController, 'signOut'> & Partial<Pick<AuthController, 'store'>>,
    page: SettingsDestination,
    preferences: UserPreferenceController,
    permissions?: PermissionController,
    reopenOnboarding?: () => void,
  ) {
    this.reopenOnboarding = reopenOnboarding
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
      this.revokeOtherSessions,
    )
    this.role = this.pages.role
    this.group = this.pages.group
    this.url = url
    this.auth = auth
    this.store.form.set('organization.name', organizationName)
    this.store.setState({ organizationSavedName: organizationName })
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

  private loadSessionLocation = async () => {
    const user = this.auth.store?.getState().currentUser
    this.locationRequest?.abort()
    this.locationRequest = undefined
    if (!user?.ipAddress) {
      this.store.setState({ sessionLocation: null })
      return
    }
    const saved = this.store.getState().sessionLocation
    if (saved?.sessionId === user.id && saved.ipAddress === user.ipAddress) return
    this.store.setState({ sessionLocation: null })
    const request = new AbortController()
    this.locationRequest = request
    const location = await lookupSessionLocation(user.ipAddress, request.signal)
    const current = this.auth.store?.getState().currentUser
    if (
      request.signal.aborted ||
      this.locationRequest !== request ||
      current?.id !== user.id ||
      current.ipAddress !== user.ipAddress
    )
      return
    if (location) this.store.setState({ sessionLocation: { sessionId: user.id, ipAddress: user.ipAddress, location } })
    this.locationRequest = undefined
  }

  reloadSessions = () => {
    void this.loadSessions()
  }
  loadMoreSessions = () => {
    if (this.store.getState().sessionsHasNextPage && !this.store.getState().sessionsLoading) void this.loadSessions(true)
  }
  private loadSessions = async (append = false) => {
    if (this.sessionsSeeded && !append) {
      this.sessionsSeeded = false
      return
    }
    const user = this.auth.store?.getState().currentUser
    if (!user) return
    this.sessionsRequest?.abort()
    const request = new AbortController()
    this.sessionsRequest = request
    if (!append) this.sessionsCursor = null
    this.store.setState({ sessionsLoading: true, sessionsError: false })
    try {
      const result = await new SettingsSessionsStore()
        .fetch({
          variables: { sessionsAfter: append ? this.sessionsCursor : undefined },
          fetch: graphqlClient.fetch(request.signal),
          policy: 'NoCache',
          blocking: true,
        })
        .then(requireData)
      if (request.signal.aborted || this.sessionsRequest !== request || this.auth.store?.getState().currentUser?.id !== user.id)
        return
      const records = result.sessions.edges
        .filter(({ node }) => node.id !== user.id && !this.revokedSessions.has(node.id))
        .map(({ node }) => {
          return {
            id: node.id,
            ipAddress: node.ipAddress ?? null,
            browser: describeSessionBrowser(node.userAgent),
            signedInAt: node.signedInAt,
            lastSeenAt: node.lastSeenAt,
            location: null,
          }
        })
      this.sessionsCursor = result.sessions.pageInfo.endCursor ?? null
      this.store.setState((state) => ({
        otherSessions: append
          ? [...state.otherSessions, ...records.filter((record) => !state.otherSessions.some((s) => s.id === record.id))]
          : records,
        sessionsLoaded: true,
        sessionsHasNextPage: result.sessions.pageInfo.hasNextPage,
        sessionsLoading: false,
      }))
      for (let index = 0; index < records.length; index += 3) {
        if (request.signal.aborted) break
        await Promise.all(
          records.slice(index, index + 3).map(async (record) => {
            if (!record.ipAddress) return
            const location = await lookupSessionLocation(record.ipAddress, request.signal)
            if (!location || request.signal.aborted || this.auth.store?.getState().currentUser?.id !== user.id) return
            this.store.setState((state) => ({
              otherSessions: state.otherSessions.map((s) => (s.id === record.id ? { ...s, location } : s)),
            }))
          }),
        )
      }
    } catch {
      if (!request.signal.aborted && this.sessionsRequest === request)
        this.store.setState({ sessionsLoading: false, sessionsError: true })
    }
  }

  revokeSession = (id: string) => {
    const user = this.auth.store?.getState().currentUser
    const owner = this.request
    if (!user || id === user.id || !owner || owner.signal.aborted) return
    void (async () => {
      try {
        await new SettingsSessionRevokeOtherStore()
          .mutate({ sessionRevokeOtherId: id }, { fetch: graphqlClient.fetch(owner.signal) })
          .then(requireData)
        if (owner.signal.aborted || this.auth.store?.getState().currentUser?.id !== user.id) return
        this.revokedSessions.add(id)
        this.store.setState((state) => ({ otherSessions: state.otherSessions.filter((s) => s.id !== id) }))
      } catch (error) {
        if (!owner.signal.aborted) notify.error(error)
      }
    })()
  }

  private revokeOtherSessions = async () => {
    const user = this.auth.store?.getState().currentUser
    const owner = this.request
    if (!user || !owner || owner.signal.aborted) return

    await new SettingsSessionRevokeOthersStore()
      .mutate(undefined, { fetch: graphqlClient.fetch(owner.signal) })
      .then(requireData)
    if (owner.signal.aborted || this.auth.store?.getState().currentUser?.id !== user.id) return
    this.sessionsRequest?.abort()
    this.sessionsRequest = undefined
    this.sessionsCursor = null
    this.store.setState({ otherSessions: [], sessionsHasNextPage: false, sessionsLoading: false })
  }

  setPage = (page: SettingsDestination, id?: string) => {
    const current = this.store.getState().location
    if (this.request && current.page === page && current.id === id) return
    this.locationRequest?.abort()
    this.locationRequest = undefined
    this.sessionsRequest?.abort()
    this.sessionsRequest = undefined
    if (page !== 'access' && page !== 'security') this.store.setState({ sessionsLoading: false })
    if (page !== 'organization') {
      this.stopOrganization?.()
      this.stopOrganization = undefined
    }
    if (page !== 'profile') {
      this.stopProfile?.()
      this.stopProfile = undefined
    }
    this.pages.members.cancel()
    this.navigationGeneration += 1
    this.role.cancel()
    this.group.cancel()
    this.store.setState({ location: { page, id } })
    if (!this.request && id && page === 'role') this.role.store.setState({ loading: true, draft: null })
    if (!this.request && id && page === 'group') this.group.store.setState({ loading: true, draft: null })
    if (page === 'organization' && this.request && !this.stopOrganization)
      this.stopOrganization = this.pages.organization.start()
    if (page === 'profile' && this.request && !this.stopProfile) this.stopProfile = this.pages.profile.start()
    if ((page === 'access' || page === 'security') && this.request) {
      void this.loadSessionLocation()
      void this.loadSessions()
    }
    if (page === 'members' && this.request) this.pages.members.start()
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
          { action: 'role.read', resourceType: 'Organization' },
          { action: 'role.create', resourceType: 'Organization' },
        ])
      if (signal.aborted || generation !== this.generation) return

      const {
        roleCreate: { node },
      } = await new SettingsRoleCreateStore()
        .mutate({ roleCreateInput: input }, { fetch: graphqlClient.fetch(signal) })
        .then(requireData)
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
          { action: 'group.read', resourceType: 'Organization' },
          { action: 'group.create', resourceType: 'Organization' },
        ])
      if (request.signal.aborted || generation !== this.generation) return

      const {
        groupCreate: { node },
      } = await new SettingsGroupCreateStore()
        .mutate({ groupCreateInput: input }, { fetch: graphqlClient.fetch(request.signal) })
        .then(requireData)
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
  href = (page: SettingsDestination, id?: string) => `${this.base}/settings/${page}${id ? '/' + encodeURIComponent(id) : ''}`
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
