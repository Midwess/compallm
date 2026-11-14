import { permissionKey, type PermissionController } from '@compallm/storybook'
import { requirePermissions } from '../../permission/controller'
import {
  createRoleStore,
  type RoleController as Contract,
  type RoleDraft,
  type SettingsState,
} from '@compallm/storybook/pages/settings'
import type { StoreApi } from 'zustand/vanilla'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { RoleUpdateInput, RoleUpdateInputConstraints } from '@compallm/contract/validation'
import { compactCedarSource } from '@compallm/contract/cedar'
import { ApiError, graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'

export class RoleController implements Contract {
  private readonly permissions?: PermissionController
  readonly store = createRoleStore()
  private request?: AbortController
  private writes: Promise<void> = Promise.resolve()
  private parseRevision = 0
  private parseTimer?: ReturnType<typeof setTimeout>
  private savedPolicy?: { id: string; source: string }
  private readonly settings: StoreApi<SettingsState>
  constructor(settings: StoreApi<SettingsState>, permissions?: PermissionController) {
    this.permissions = permissions
    this.settings = settings
  }
  cancel = () => {
    this.store.form.stop()
    this.store.setState({ disabledPolicies: {}, loading: false })
    this.parseRevision += 1
    clearTimeout(this.parseTimer)
    this.request?.abort()
    this.request = undefined
  }
  load = async (id: string) => {
    this.cancel()
    const request = new AbortController()
    this.request = request
    this.store.setState({ draft: null, loading: true, busy: false })
    this.settings.setState({ policyActions: [] })
    try {
      if (this.permissions) await requirePermissions(this.permissions, [{ action: 'ROLE_READ', resourceType: 'ORGANIZATION' }])
      if (request.signal.aborted || this.request !== request) return
      const policyRead = { action: 'ROLE_POLICY_READ', resourceType: 'ROLE', resourceId: id } as const
      if (this.permissions) await this.permissions.ensure([policyRead])
      if (request.signal.aborted || this.request !== request) return
      const canReadPolicy =
        !this.permissions || this.permissions.store.getState().entries[permissionKey(policyRead)]?.status === 'allowed'
      const roleSelection = {
        role: {
          __args: { id },
          node: { id: true, name: true, description: true, color: true, cedarSource: canReadPolicy, owner: true },
        },
      } satisfies QueryGenqlSelection
      const catalogSelection = {
        organizationPolicyActions: {
          edges: { node: { name: true, displayName: true, description: true, entityType: true, resourceType: true } },
        },
      } satisfies QueryGenqlSelection
      const [metadata, catalog] = await Promise.allSettled([
        graphql(request.signal).query(roleSelection),
        graphql(request.signal).query(catalogSelection),
      ])
      if (request.signal.aborted || this.request !== request) return
      if (metadata.status === 'rejected') throw metadata.reason
      if (!metadata.value.role) return
      if (catalog.status === 'fulfilled') {
        this.settings.setState({
          policyActions: catalog.value.organizationPolicyActions.edges.map(({ node }) => ({
            id: node.name,
            label: node.displayName,
            category: node.entityType,
            scope: node.resourceType,
            description: node.description,
          })),
        })
      } else {
        const source = metadata.value.role.node.cedarSource ?? null
        notify.apiError(catalog.reason, () => {
          const draft = this.store.getState().draft
          if (this.request === request && draft?.id === id && draft.source === (source ?? '')) void this.load(id)
        })
      }
      const role = { ...metadata.value.role.node, cedarSource: metadata.value.role.node.cedarSource ?? null }
      let projection: RoleDraft['projection'] = []
      let diagnostics =
        role.cedarSource === null
          ? ['Policy source is unavailable.']
          : catalog.status === 'rejected'
            ? ['Policy catalog is unavailable.']
            : []
      if (role.cedarSource && catalog.status === 'fulfilled') {
        try {
          const { projectPolicy } = await import('./cedar')
          const result = await projectPolicy(
            role.cedarSource,
            new Map(catalog.value.organizationPolicyActions.edges.map(({ node }) => [node.name, node.resourceType])),
          )
          projection = result.projection
          diagnostics = result.diagnostics
        } catch {
          diagnostics = ['Policy parser is unavailable.']
        }
      }
      if (request.signal.aborted || this.request !== request) return
      this.openRole({ ...role, source: role.cedarSource }, { projection, diagnostics })
    } catch (error) {
      if (!request.signal.aborted)
        notify.apiError(error, () => {
          if (this.request === request) void this.load(id)
        })
    } finally {
      if (this.request === request) this.store.setState({ loading: false })
    }
  }
  setQuery = (query: string) => this.store.setState({ query })
  setFilter = (key: string, value: string) => this.store.setState((s) => ({ filters: { ...s.filters, [key]: value } }))
  openRole = (
    role: { id: string; name: string; description: string; source: string | null; owner?: boolean },
    policy?: Pick<RoleDraft, 'projection' | 'diagnostics'>,
  ) => {
    this.savedPolicy = role.source === null ? undefined : { id: role.id, source: compactCedarSource(role.source) }
    this.store.setState({
      draft: {
        ...role,
        source: role.source ?? '',
        mode: policy ? (policy.diagnostics.length ? 'cedar' : 'visual') : role.source === '' ? 'visual' : 'cedar',
        projection: policy?.projection ?? [],
        diagnostics: policy?.diagnostics ?? (role.source === null ? ['Policy source is unavailable.'] : []),
        parsing: false,
        builtin: role.owner ?? false,
      },
    })
  }
  open = () => {
    if (this.store.getState().draft) return
    this.store.setState({
      draft: {
        id: null,
        name: '',
        description: '',
        source: '',
        mode: 'visual',
        projection: [],
        diagnostics: [],
        parsing: false,
        builtin: false,
      },
    })
  }
  private unavailable = () => notify.info('Settings changes are not available yet.')
  private policyInput = (source: string) => {
    const result = RoleUpdateInput.safeParse({ cedarSource: source })
    if (result.success) return result.data
    const length = [...compactCedarSource(source)].length
    const maximum = RoleUpdateInputConstraints.cedarSource.maxLength
    const message =
      length > maximum
        ? `Compact Cedar source has ${length.toLocaleString()} characters; the limit is ${maximum.toLocaleString()}. ` +
          'Shorten the policy before saving. Your draft is retained.'
        : 'Policy changes could not be validated. Review the Cedar source before saving.'
    throw new ApiError(message, 400, 'BAD_USER_INPUT')
  }
  updateMetadata = async (value: Partial<Pick<RoleDraft, 'name' | 'description'>>, signal?: AbortSignal) => {
    const draft = this.store.getState().draft
    const request = this.request
    if (!draft?.id || draft.builtin || !request) throw new Error('Role is not editable')
    const id = draft.id
    if (this.permissions)
      await requirePermissions(this.permissions, [{ action: 'ROLE_METADATA_UPDATE', resourceType: 'ROLE', resourceId: id }])
    if (request.signal.aborted || this.request !== request || signal?.aborted) throw new DOMException('Cancelled', 'AbortError')
    const input = RoleUpdateInput.parse(value)
    const selection = {
      roleUpdate: { __args: { id, input }, node: { id: true, name: true, description: true, color: true } },
    } satisfies MutationGenqlSelection
    const {
      roleUpdate: { node },
    } = await graphql(signal ? AbortSignal.any([signal, request.signal]) : request.signal).mutation(selection)
    if (request.signal.aborted || this.request !== request) throw new DOMException('Cancelled', 'AbortError')
    this.settings.setState((state) => ({ roles: state.roles.map((role) => (role.id === id ? { ...role, ...node } : role)) }))
    return node
  }
  setPolicyMode = (mode: RoleDraft['mode']) => {
    const draft = this.store.getState().draft
    if (!draft) return
    if (mode === 'visual' && (draft.parsing || draft.diagnostics.length > 0)) {
      return
    }
    this.store.setState({ draft: { ...draft, mode } })
  }
  private projectSource = async (id: string, source: string, revision: number) => {
    try {
      const { projectPolicy } = await import('./cedar')
      const catalog = this.settings.getState().policyActions
      const result = await projectPolicy(source, new Map(catalog.map((item) => [item.id, item.scope])))
      if (revision !== this.parseRevision || this.store.getState().draft?.id !== id) return false
      this.store.setState((state) => ({
        draft:
          state.draft?.id === id && state.draft.source === source
            ? {
                ...state.draft,
                parsing: false,
                diagnostics: result.diagnostics,
                projection: result.diagnostics.length ? [] : result.projection,
              }
            : state.draft,
      }))
      return result.diagnostics.length === 0
    } catch {
      if (revision === this.parseRevision)
        this.store.setState((state) => ({
          draft:
            state.draft?.id === id
              ? { ...state.draft, parsing: false, projection: [], diagnostics: ['Policy parser is unavailable.'] }
              : state.draft,
        }))
      return false
    }
  }
  setPolicySource = (source: string) => {
    const draft = this.store.getState().draft
    if (!draft?.id || draft.builtin) return
    const id = draft.id
    const revision = ++this.parseRevision
    clearTimeout(this.parseTimer)
    this.store.setState({
      disabledPolicies: {},
      draft: { ...draft, source, parsing: true, projection: [], diagnostics: [] },
    })
    this.parseTimer = setTimeout(() => {
      void this.projectSource(id, source, revision)
    }, 150)
  }
  commitPolicySource = async (source: string) => {
    const draft = this.store.getState().draft
    const request = this.request
    if (!draft?.id || draft.builtin || !request || this.savedPolicy?.id !== draft.id) return false
    const id = draft.id
    if (compactCedarSource(draft.source) === compactCedarSource(source)) {
      clearTimeout(this.parseTimer)
      const revision = ++this.parseRevision
      this.store.setState({ draft: { ...draft, parsing: true } })
      if (!(await this.projectSource(id, draft.source, revision))) return false
      if (revision !== this.parseRevision) return false
    } else {
      const { projectPolicy } = await import('./cedar')
      const catalog = this.settings.getState().policyActions
      const parsed = await projectPolicy(source, new Map(catalog.map((item) => [item.id, item.scope])))
      if (parsed.diagnostics.length) return false
    }
    if (request.signal.aborted || this.request !== request) return false
    if (this.savedPolicy?.source === compactCedarSource(source)) return true
    if (this.permissions)
      await requirePermissions(this.permissions, [{ action: 'ROLE_POLICY_UPDATE', resourceType: 'ROLE', resourceId: id }])
    if (request.signal.aborted || this.request !== request) return false
    const input = this.policyInput(source)
    const selection = { roleUpdate: { __args: { id, input }, node: { id: true } } } satisfies MutationGenqlSelection
    await graphql(request.signal).mutation(selection)
    if (request.signal.aborted || this.request !== request) return false
    this.savedPolicy = { id, source: compactCedarSource(source) }
    return true
  }
  togglePolicyAction = (action: string) => {
    const draft = this.store.getState().draft
    if (draft)
      return this.setPolicyActions(
        [action],
        !draft.projection.some((entry) => entry.action === action && entry.effect !== 'forbid'),
      )
  }
  setPolicyActions = (actions: string[], enabled: boolean) => {
    const { draft, busy } = this.store.getState()
    if (!draft?.id || draft.builtin || !this.request || busy || draft.parsing || draft.diagnostics.length) return
    const id = draft.id
    const original = draft.source
    const request = this.request
    const catalog = this.settings.getState().policyActions
    const disabledPolicies = { ...this.store.getState().disabledPolicies }
    this.store.setState({ busy: true })
    this.writes = this.writes.then(async () => {
      try {
        const {
          actionStatement,
          actionPolicies,
          formatPolicySource,
          normalizePolicySource,
          projectPolicy,
          removePolicyAction,
        } = await import('./cedar')
        let source = await normalizePolicySource(original)
        for (const action of new Set(actions)) {
          const entry = catalog.find((item) => item.id === action)
          if (!entry) continue
          const current = draft.projection.filter((item) => item.action === action && item.effect !== 'forbid')
          if (current.some((item) => !item.editable)) throw new Error('Edit this action scope in Cedar.')
          if (enabled && current.length === 0) {
            const statements = disabledPolicies[action] ?? [actionStatement(action, entry.scope)]
            source += `${source && !source.endsWith('\n') ? '\n' : ''}${statements.join('\n')}\n`
          }
          if (!enabled && current.length) {
            disabledPolicies[action] = await actionPolicies(source, action)
            source = await removePolicyAction(source, action)
          }
        }
        source = await formatPolicySource(source)
        if (source === original || request.signal.aborted || this.request !== request) return
        const projected = await projectPolicy(source, new Map(catalog.map((item) => [item.id, item.scope])))
        if (projected.diagnostics.length) throw new Error('Policy parsing failed')
        if (request.signal.aborted || this.request !== request) return
        this.store.setState((state) => ({
          disabledPolicies,
          draft:
            state.draft?.id === id && state.draft.source === original
              ? { ...state.draft, source, projection: projected.projection, diagnostics: projected.diagnostics }
              : state.draft,
        }))
      } catch (error) {
        if (!request.signal.aborted) {
          notify.apiError(error, () => {
            if (this.request !== request || this.store.getState().draft?.id !== id) return
            if (this.store.getState().draft?.source === original)
              void this.store.form.perform(() => this.setPolicyActions(actions, enabled), ['draft.source'])
            else void this.store.form.retry()
          })
        }
      } finally {
        if (this.request === request) this.store.setState({ busy: false })
      }
    })
    return this.writes
  }
  formatPolicy = () => {
    const draft = this.store.getState().draft
    if (!draft || draft.builtin || draft.parsing || draft.diagnostics.length) return
    void import('./cedar')
      .then(({ formatPolicySource }) => formatPolicySource(draft.source))
      .then((source) => {
        if (this.store.getState().draft?.id === draft.id && this.store.getState().draft?.source === draft.source)
          this.setPolicySource(source)
      })
      .catch(notify.error)
  }
  saveRole = this.unavailable
  resetRole = this.unavailable
}
