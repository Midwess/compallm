import {
  createRoleStore,
  type RoleController as Contract,
  type RoleDraft,
  type SettingsState,
} from '@compallm/storybook/pages/settings'
import type { StoreApi } from 'zustand/vanilla'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { RoleUpdateInput } from '@compallm/contract/validation'
import { graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'

export class RoleController implements Contract {
  readonly store = createRoleStore()
  private request?: AbortController
  private writes: Promise<void> = Promise.resolve()
  private parseRevision = 0
  private parseTimer?: ReturnType<typeof setTimeout>
  private savedPolicy?: { id: string; source: string }
  private savedMetadata?: { id: string; name: string; description: string }
  private readonly settings: StoreApi<SettingsState>
  private readonly organizationId?: string
  constructor(settings: StoreApi<SettingsState>, organizationId?: string) {
    this.settings = settings
    this.organizationId = organizationId
  }
  cancel = () => {
    this.parseRevision += 1
    clearTimeout(this.parseTimer)
    this.request?.abort()
    this.request = undefined
  }
  load = async (id: string) => {
    this.cancel()
    const request = new AbortController()
    this.request = request
    this.store.setState({ draft: null, busy: false, policyEditable: false, policySaveFailed: false })
    this.settings.setState({ policyActions: [] })
    try {
      const roleSelection = {
        role: { __args: { id }, node: { id: true, name: true, description: true, color: true, cedarSource: true } },
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
      this.openRole({
        ...metadata.value.role.node,
        source: metadata.value.role.node.cedarSource,
      })
      this.store.setState({
        policyEditable: Boolean(this.organizationId),
        cedarEditable: metadata.value.role.node.cedarSource !== null,
      })
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
      } else notify.error(catalog.reason)
      const draft = this.store.getState().draft
      if (draft?.source && catalog.status === 'fulfilled') {
        this.store.setState({ draft: { ...draft, parsing: true } })
        try {
          const { projectPolicy } = await import('./cedar')
          const result = await projectPolicy(
            draft.source,
            new Map(catalog.value.organizationPolicyActions.edges.map(({ node }) => [node.name, node.resourceType])),
            this.organizationId ?? '',
            id,
          )
          if (request.signal.aborted || this.request !== request) return
          this.store.setState((state) => ({
            policyEditable:
              state.draft?.id === id && state.draft.source === draft.source
                ? Boolean(this.organizationId) &&
                  !result.diagnostics.length &&
                  !result.projection.some((entry) => !entry.action)
                : state.policyEditable,
            draft:
              state.draft?.id === id && state.draft.source === draft.source
                ? {
                    ...state.draft,
                    parsing: false,
                    projection: result.projection,
                    diagnostics: result.diagnostics,
                    mode: result.diagnostics.length ? 'cedar' : 'visual',
                  }
                : state.draft,
          }))
        } catch {
          if (!request.signal.aborted && this.request === request)
            this.store.setState((state) => ({
              draft:
                state.draft?.id === id
                  ? { ...state.draft, parsing: false, diagnostics: ['Policy parser is unavailable.'] }
                  : state.draft,
            }))
        }
      }
    } catch (error) {
      if (!request.signal.aborted) notify.error(error)
    }
  }
  setQuery = (query: string) => this.store.setState({ query })
  setFilter = (key: string, value: string) => this.store.setState((s) => ({ filters: { ...s.filters, [key]: value } }))
  openRole = (role: { id: string; name: string; description: string; source: string | null }) => {
    this.savedMetadata = { id: role.id, name: role.name, description: role.description }
    this.savedPolicy = role.source === null ? undefined : { id: role.id, source: role.source }
    this.store.setState({
      draft: {
        ...role,
        source: role.source ?? '',
        mode: role.source === '' ? 'visual' : 'cedar',
        projection: [],
        diagnostics: role.source === null ? ['Policy source is unavailable.'] : [],
        parsing: false,
        builtin: false,
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
  updateRole = (value: Partial<Pick<RoleDraft, 'name' | 'description'>>) => {
    this.store.setState((state) => ({ draft: state.draft ? { ...state.draft, ...value } : null }))
  }
  commitRoleField = (field: 'name' | 'description') => {
    const draft = this.store.getState().draft
    if (!draft?.id || !this.savedMetadata || this.savedMetadata.id !== draft.id) return
    const id = draft.id
    const value = draft[field]
    this.writes = this.writes.then(async () => {
      if (this.savedMetadata?.id !== id || value === this.savedMetadata[field]) return
      try {
        const input = RoleUpdateInput.parse({ [field]: value })
        const selection = {
          roleUpdate: { __args: { id, input }, node: { id: true, name: true, description: true, color: true } },
        } satisfies MutationGenqlSelection
        const {
          roleUpdate: { node },
        } = await graphql().mutation(selection)
        if (this.savedMetadata?.id !== id) return
        this.savedMetadata = { id, name: node.name, description: node.description }
        this.settings.setState((state) => ({
          roles: state.roles.map((role) =>
            role.id === id ? { ...role, name: node.name, description: node.description, color: node.color } : role,
          ),
        }))
        this.store.setState((state) => ({
          draft:
            state.draft?.id === id && state.draft[field] === value ? { ...state.draft, [field]: node[field] } : state.draft,
        }))
      } catch (error) {
        notify.error(error)
      }
    })
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
      const result = await projectPolicy(
        source,
        new Map(catalog.map((item) => [item.id, item.scope])),
        this.organizationId ?? '',
        id,
      )
      if (revision !== this.parseRevision || this.store.getState().draft?.id !== id) return false
      this.store.setState((state) => ({
        policyEditable:
          Boolean(this.organizationId) && !result.diagnostics.length && !result.projection.some((entry) => !entry.action),
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
    if (!draft?.id) return
    const id = draft.id
    const revision = ++this.parseRevision
    clearTimeout(this.parseTimer)
    this.store.setState({ policyEditable: false, draft: { ...draft, source, parsing: true, projection: [], diagnostics: [] } })
    this.parseTimer = setTimeout(() => {
      void this.projectSource(id, source, revision)
    }, 150)
  }
  commitPolicySource = () => {
    const draft = this.store.getState().draft
    const request = this.request
    if (!draft?.id || !request || this.savedPolicy?.id !== draft.id) return
    const id = draft.id
    const source = draft.source
    clearTimeout(this.parseTimer)
    const revision = ++this.parseRevision
    this.store.setState({ draft: { ...draft, parsing: true } })
    this.writes = this.writes.then(async () => {
      if (request.signal.aborted || this.request !== request || revision !== this.parseRevision) return
      if (this.savedPolicy?.id !== id || this.savedPolicy.source === source) {
        this.store.setState((state) => ({ draft: state.draft?.id === id ? { ...state.draft, parsing: false } : state.draft }))
        return
      }
      if (!(await this.projectSource(id, source, revision))) return
      if (request.signal.aborted || this.request !== request || revision !== this.parseRevision) return
      try {
        const input = RoleUpdateInput.parse({ cedarSource: source })
        const selection = { roleUpdate: { __args: { id, input }, node: { id: true } } } satisfies MutationGenqlSelection
        await graphql(request.signal).mutation(selection)
        if (!request.signal.aborted && this.request === request) {
          this.savedPolicy = { id, source }
          this.store.setState({ policySaveFailed: false })
        }
      } catch (error) {
        if (!request.signal.aborted) {
          this.store.setState({ policySaveFailed: true })
          notify.error(error)
        }
      }
    })
  }
  togglePolicyAction = (action: string) => {
    const draft = this.store.getState().draft
    if (draft) this.setPolicyActions([action], !draft.projection.some((entry) => entry.action === action))
  }
  setPolicyActions = (actions: string[], enabled: boolean) => {
    const { draft, busy } = this.store.getState()
    if (!draft?.id || !this.organizationId || !this.request || busy || draft.parsing || draft.diagnostics.length) return
    const id = draft.id
    const original = draft.source
    const request = this.request
    const organizationId = this.organizationId
    const catalog = this.settings.getState().policyActions
    this.store.setState({ busy: true })
    this.writes = this.writes.then(async () => {
      try {
        const { actionStatement, projectPolicy } = await import('./cedar')
        let source = original
        for (const action of new Set(actions)) {
          const entry = catalog.find((item) => item.id === action)
          if (!entry) continue
          const current = draft.projection.filter((item) => item.action === action)
          if (current.some((item) => !item.editable)) continue
          const statement = actionStatement(action, organizationId, id, entry.scope)
          if (enabled && current.length === 0) source += `${source && !source.endsWith('\n') ? '\n' : ''}${statement}\n`
          if (!enabled && current.length === 1 && current[0].source === statement) source = source.replace(statement, '')
        }
        if (source === original || request.signal.aborted || this.request !== request) return
        const projected = await projectPolicy(source, new Map(catalog.map((item) => [item.id, item.scope])), organizationId, id)
        if (projected.diagnostics.length) throw new Error('Policy parsing failed')
        if (request.signal.aborted || this.request !== request) return
        this.store.setState((state) => ({
          draft:
            state.draft?.id === id && state.draft.source === original
              ? { ...state.draft, source, projection: projected.projection, diagnostics: projected.diagnostics }
              : state.draft,
        }))
        const input = RoleUpdateInput.parse({ cedarSource: source })
        const selection = { roleUpdate: { __args: { id, input }, node: { id: true } } } satisfies MutationGenqlSelection
        await graphql(request.signal).mutation(selection)
        if (request.signal.aborted || this.request !== request) return
        this.savedPolicy = { id, source }
        this.store.setState({ policySaveFailed: false })
      } catch (error) {
        if (!request.signal.aborted) {
          this.store.setState({ policySaveFailed: true })
          notify.error(error)
        }
      } finally {
        if (this.request === request) this.store.setState({ busy: false })
      }
    })
  }
  formatPolicy = () => {
    const draft = this.store.getState().draft
    if (!draft || draft.parsing || draft.diagnostics.length) return
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
