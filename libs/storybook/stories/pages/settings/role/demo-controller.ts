import { createRoleStore } from '../../../../src/pages/settings'
import type { RoleController, RoleDraft, SettingsController } from '../../../../src/pages/settings'
import { actionSource } from '../fixtures'
import { cedar, project } from '../cedar'

const colors = ['#A49ADA', '#6EB8A8', '#D7A466', '#8CAFD1']
export class DemoRoleController implements RoleController {
  readonly store = createRoleStore()
  setQuery = (query: string) => this.store.setState({ query })
  setFilter = (key: string, value: string) => this.store.setState((s) => ({ filters: { ...s.filters, [key]: value } }))
  private revision = 0
  private parseTimer?: ReturnType<typeof setTimeout>
  private saveTimer?: ReturnType<typeof setTimeout>
  private closed = false
  constructor(
    private readonly settings: Pick<SettingsController, 'store' | 'navigate'> & { report(notice: string): void },
    private readonly failWrites: boolean | 'conflict',
  ) {}
  start = () => {
    this.closed = false
    const source = this.store.getState().draft?.source
    if (source) this.setPolicySource(source)
  }
  cancel = () => {
    this.revision++
    clearTimeout(this.parseTimer)
  }
  stop = () => {
    this.closed = true
    this.cancel()
    clearTimeout(this.saveTimer)
  }
  open = (id?: string) => {
    this.revision++
    this.store.setState({ query: '', filters: {} })
    const role = id ? this.settings.store.getState().roles.find((item) => item.id === id) : undefined
    if (role && this.store.getState().draft?.id === role.id) {
      this.setPolicySource(this.store.getState().draft!.source)
      return
    }
    const draft: RoleDraft = {
      id: role?.id ?? null,
      name: role?.name ?? '',
      description: role?.description ?? '',
      source: role?.source ?? '',
      mode: 'visual',
      projection: [],
      diagnostics: [],
      parsing: false,
      builtin: role?.id === 'owner',
    }
    this.store.setState({ draft, error: null })
    if (draft.source) this.setPolicySource(draft.source)
  }
  updateMetadata = async (value: Partial<Pick<RoleDraft, 'name' | 'description'>>) => {
    if (this.failWrites) throw new Error('Could not save changes. Your draft is retained.')
    const draft = this.store.getState().draft
    if (!draft || draft.builtin) throw new Error('Role is not editable')
    const accepted = { name: (value.name ?? draft.name).trim(), description: value.description ?? draft.description }
    this.settings.store.setState((state) => ({
      roles: state.roles.map((role) => (role.id === draft.id ? { ...role, ...accepted } : role)),
    }))
    return accepted
  }
  setPolicyMode = (mode: RoleDraft['mode']) =>
    this.store.setState((s) => ({
      draft:
        s.draft && !(mode === 'visual' && (s.draft.parsing || s.draft.diagnostics.length)) ? { ...s.draft, mode } : s.draft,
    }))
  setPolicySource = (source: string) => {
    const revision = ++this.revision
    this.store.setState((s) => ({ draft: s.draft ? { ...s.draft, source, parsing: true } : null }))
    clearTimeout(this.parseTimer)
    this.parseTimer = setTimeout(() => {
      void project(source)
        .then((result) => {
          if (this.closed || revision !== this.revision) return
          this.store.setState((s) => ({
            draft: s.draft
              ? {
                  ...s.draft,
                  source,
                  parsing: false,
                  diagnostics: result.diagnostics,
                  projection: result.diagnostics.length ? s.draft.projection : result.projection,
                }
              : null,
          }))
        })
        .catch(() => {
          if (this.closed || revision !== this.revision) return
          this.store.setState((s) => ({
            draft: s.draft
              ? { ...s.draft, parsing: false, diagnostics: ['Policy parser is unavailable. Your draft is retained.'] }
              : null,
          }))
        })
    }, 150)
  }
  togglePolicyAction = (action: string) =>
    this.setPolicyActions([action], !this.store.getState().draft?.projection.some((p) => p.action === action))
  setPolicyActions = (actions: string[], enabled: boolean) => {
    const state = this.settings.store.getState(),
      draft = this.store.getState().draft
    if (!draft || draft.parsing || draft.diagnostics.length || this.store.getState().busy) return
    const allowed = new Set(state.policyActions.map((a) => a.id))
    let source = draft.source
    for (const action of new Set(actions)) {
      if (!allowed.has(action)) continue
      const present = draft.projection.filter((p) => p.action === action)
      if (present.some((p) => !p.editable)) continue
      if (!enabled) {
        for (const p of present) source = source.replace(p.source, '')
      } else if (!present.length) source += `${source ? '\n\n' : ''}${actionSource(action)}`
    }
    if (source !== draft.source) this.setPolicySource(source)
  }
  formatPolicy = () => {
    const draft = this.store.getState().draft
    if (!draft || draft.parsing || draft.diagnostics.length) return
    const source = draft.source
    void cedar().then((c) => {
      if (this.closed || this.store.getState().draft?.source !== source) return
      const result = c.formatPolicies({ policyText: source, lineWidth: 80, indentWidth: 2 })
      if (result.type === 'success') this.setPolicySource(result.formatted_policy)
    })
  }
  commitPolicySource = async (source: string) => {
    const draft = this.store.getState().draft
    if (!draft || draft.builtin) return false
    if (this.failWrites) throw new Error('Could not save policy. Your draft is retained.')
    const parsed = await project(source)
    if (parsed.diagnostics.length) return false
    this.settings.store.setState((state) => ({
      roles: state.roles.map((role) => (role.id === draft.id ? { ...role, source } : role)),
    }))
    return true
  }
  saveRole = () => {
    const draft = this.store.getState().draft
    if (
      !draft ||
      !draft.name.trim() ||
      !draft.source.trim() ||
      draft.parsing ||
      draft.diagnostics.length ||
      draft.builtin ||
      this.store.getState().busy ||
      this.settings.store.getState().readonly
    )
      return
    this.store.setState({ busy: true, error: null })
    this.saveTimer = setTimeout(() => {
      if (this.closed) return
      if (this.failWrites) {
        this.store.setState({
          busy: false,
          error:
            this.failWrites === 'conflict'
              ? 'This role changed elsewhere. Your draft is retained.'
              : 'Could not save changes. Your draft is retained.',
        })
        return
      }
      const id = draft.id ?? crypto.randomUUID(),
        roles = this.settings.store.getState().roles
      const role = {
        id,
        name: draft.name.trim(),
        description: draft.description,
        source: draft.source,
        color: roles.find((r) => r.id === id)?.color ?? colors[roles.length % colors.length],
      }
      this.settings.store.setState({
        roles: roles.some((r) => r.id === id) ? roles.map((r) => (r.id === id ? role : r)) : [...roles, role],
        location: { page: 'role', id },
      })
      this.store.setState({ draft: { ...draft, id }, busy: false })
      this.settings.report(draft.id ? 'Role saved' : 'Role created')
    }, 240)
  }
  resetRole = () => {
    const draft = this.store.getState().draft
    this.store.setState({ draft: null })
    this.settings.navigate(draft?.id ? 'role' : 'general', draft?.id ?? undefined)
  }
}
