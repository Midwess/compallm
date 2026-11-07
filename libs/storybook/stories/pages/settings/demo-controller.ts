import { createSettingsStore } from '../../../src/pages/settings'
import type {
  SettingsController,
  SettingsState,
  SettingsDestination,
  SettingsDialogKind,
  SettingsDialog,
} from '../../../src/pages/settings'
import type { GroupDraft, RoleDraft } from '../../../src/pages/settings'
import { initialSettings, actionSource } from './fixtures'
import { cedar, project } from './cedar'

const colors = ['#A49ADA', '#6EB8A8', '#D7A466', '#8CAFD1']
export class DemoSettingsController implements SettingsController {
  store = createSettingsStore(structuredClone(initialSettings))
  private timers = new Set<ReturnType<typeof setTimeout>>()
  private urls: string[] = []
  private revision = 0
  private parseTimer: ReturnType<typeof setTimeout> | undefined
  private verificationTimer: ReturnType<typeof setTimeout> | undefined
  private closed = false
  constructor(
    initial: Partial<SettingsState> = {},
    private failWrites: boolean | 'conflict' = false,
  ) {
    this.store.setState(structuredClone(initial))
    const location = this.store.getState().location
    this.navigate(location.page, location.id)
    if (initial.roleDraft) this.store.setState({ roleDraft: structuredClone(initial.roleDraft) })
    if (initial.groupDraft) this.store.setState({ groupDraft: structuredClone(initial.groupDraft) })
    if (initial.filters) this.store.setState({ filters: structuredClone(initial.filters) })
    if (initial.query !== undefined) this.store.setState({ query: initial.query })
    if (initial.dialog) this.store.setState({ dialog: initial.dialog })
  }
  start = () => {
    this.closed = false
    const draft = this.store.getState().roleDraft
    if (draft?.source) this.setPolicySource(draft.source)
  }
  stop = () => {
    this.closed = true
    this.revision++
    this.timers.forEach(clearTimeout)
    this.urls.forEach(URL.revokeObjectURL)
  }
  private report(notice: string) {
    this.store.setState({ notice, error: null })
  }
  private commit(operation: () => void) {
    if (this.store.getState().busy || this.store.getState().readonly) return
    this.store.setState({ busy: true, error: null })
    const timer = setTimeout(() => {
      this.timers.delete(timer)
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
      operation()
      this.store.setState({ busy: false })
    }, 240)
    this.timers.add(timer)
  }
  navigate = (page: SettingsDestination, id?: string) => {
    this.revision++
    const s = this.store.getState()
    this.store.setState({ location: { page, id }, query: '', filters: {}, error: null, dialog: null })
    if (page === 'new-role' || page === 'new-group') this.store.setState({ navigationQuery: '' })
    if (page === 'role' || page === 'new-role') {
      const role = page === 'role' ? s.roles.find((r) => r.id === id) : null
      if (role && s.roleDraft?.id === role.id) {
        this.setPolicySource(s.roleDraft.source)
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
        builtin: role?.builtin ?? false,
      }
      this.store.setState({ roleDraft: draft })
      if (draft.source) this.setPolicySource(draft.source)
    }
    if (page === 'group' || page === 'new-group') {
      const group = page === 'group' ? s.groups.find((g) => g.id === id) : null
      if (group && s.groupDraft?.id === group.id) return
      this.store.setState({
        groupDraft: group
          ? {
              id: group.id,
              name: group.name,
              description: group.description,
              memberIds: [...group.memberIds],
              roleIds: [...group.roleIds],
            }
          : { id: null, name: '', description: '', memberIds: [], roleIds: [] },
      })
    }
  }
  setQuery = (query: string) => this.store.setState({ query })
  setNavigationQuery = (navigationQuery: string) => this.store.setState({ navigationQuery })
  setFilter = (key: string, value: string) => this.store.setState((s) => ({ filters: { ...s.filters, [key]: value } }))
  setPreference = (key: string, value: string | boolean) =>
    this.store.setState((s) => ({ preferences: { ...s.preferences, [key]: value } }))
  updateProfile = (value: Partial<SettingsState['profile']>) =>
    this.store.setState((s) => ({ profile: { ...s.profile, ...value } }))
  saveProfile = () => this.commit(() => this.report('Profile saved'))
  updateOrganization = (value: Partial<SettingsState['organization']>) =>
    this.store.setState((s) => ({ organization: { ...s.organization, ...value } }))
  setAvatar = (file: File | null, organization = false) => {
    const url = file ? URL.createObjectURL(file) : null
    if (url) this.urls.push(url)
    if (organization) this.updateOrganization({ logo: url })
    else this.updateProfile({ avatar: url })
  }
  openDialog = (kind: SettingsDialogKind, targetId?: string) => {
    const s = this.store.getState()
    const value =
      kind === 'organization-limit'
        ? s.organizationLimitUsd
        : kind === 'member-limit'
          ? targetId
            ? s.members.find((m) => m.id === targetId)?.limitUsd
            : s.memberDefaultLimitUsd
          : kind === 'reset-schedule'
            ? s.timezone
            : kind === 'welcome'
              ? s.organization.welcome
              : kind === 'announcements'
                ? s.organization.announcements
                : ''
    this.store.setState({ dialog: { kind, targetId, name: '', value: value ?? '', phrase: '' }, error: null })
  }
  updateDialog = (value: Partial<SettingsDialog>) =>
    this.store.setState((s) => ({ dialog: s.dialog ? { ...s.dialog, ...value } : null }))
  closeDialog = () => {
    if (this.store.getState().busy) return
    if (this.verificationTimer) {
      clearTimeout(this.verificationTimer)
      this.timers.delete(this.verificationTimer)
      this.verificationTimer = undefined
    }
    this.store.setState({ dialog: null })
  }
  confirmDialog = () => {
    const s = this.store.getState(),
      d = s.dialog
    if (!d) return
    if (d.kind === 'delete-account' && d.phrase !== `I want to delete ${s.profile.email}`) return
    if (d.kind === 'delete-organization' && d.phrase !== `I want to delete ${s.organization.name}`) return
    if (['add-passkey', 'create-key'].includes(d.kind) && !d.name.trim()) return
    if (d.kind === 'add-passkey') {
      this.store.setState({ dialog: { ...d, kind: 'verify-passkey' } })
      const timer = setTimeout(() => {
        this.timers.delete(timer)
        if (this.closed || this.store.getState().dialog?.kind !== 'verify-passkey') return
        this.commit(() => {
          this.store.setState((p) => ({ passkeys: [...p.passkeys, { id: crypto.randomUUID(), name: d.name }], dialog: null }))
          this.report('Passkey added')
        })
      }, 900)
      this.timers.add(timer)
      this.verificationTimer = timer
      return
    }
    this.commit(() => {
      const id = d.targetId
      if (d.kind === 'create-key') {
        const key = { id: crypto.randomUUID(), name: d.name.trim(), prefix: 'preview_key_••••' }
        this.store.setState((p) => ({
          apiKeys: [...p.apiKeys, key],
          dialog: { ...d, kind: 'reveal-key', revealedKey: 'preview_key_not_a_real_credential' },
        }))
        return
      }
      if (d.kind === 'remove-member')
        this.store.setState((p) => ({
          members: p.members.filter((m) => m.id !== id),
          groups: p.groups.map((g) => ({ ...g, memberIds: g.memberIds.filter((member) => member !== id) })),
          groupDraft: p.groupDraft
            ? { ...p.groupDraft, memberIds: p.groupDraft.memberIds.filter((member) => member !== id) }
            : null,
        }))
      if (d.kind === 'remove-passkey') this.store.setState((p) => ({ passkeys: p.passkeys.filter((key) => key.id !== id) }))
      if (d.kind === 'application')
        this.store.setState((p) => ({ applications: p.applications.filter((app) => app.id !== id) }))
      if (d.kind === 'revoke-key') this.store.setState((p) => ({ apiKeys: p.apiKeys.filter((key) => key.id !== id) }))
      if (d.kind === 'revoke-sessions') this.store.setState({ hasOtherSessions: false })
      if (d.kind === 'delete-role') {
        this.store.setState((p) => ({
          roles: p.roles.filter((r) => r.id !== id),
          roleDraft: null,
          groupDraft: null,
          groups: p.groups.map((g) => ({ ...g, roleIds: g.roleIds.filter((role) => role !== id) })),
          members: p.members.map((m) => ({ ...m, roleIds: m.roleIds.filter((role) => role !== id) })),
          location: { page: 'general' },
        }))
      }
      if (d.kind === 'delete-group')
        this.store.setState((p) => ({
          groups: p.groups.filter((g) => g.id !== id),
          groupDraft: null,
          location: { page: 'general' },
        }))
      if (d.kind === 'remove-group-member') this.toggleGroupMember(id!)
      if (d.kind === 'organization-limit') this.store.setState({ organizationLimitUsd: d.value || null })
      if (d.kind === 'member-limit')
        this.store.setState((p) =>
          id
            ? { members: p.members.map((m) => (m.id === id ? { ...m, limitUsd: d.value || null } : m)) }
            : { memberDefaultLimitUsd: d.value || null },
        )
      if (d.kind === 'reset-schedule') this.store.setState({ timezone: d.value })
      if (d.kind === 'welcome') this.updateOrganization({ welcome: d.value })
      if (d.kind === 'announcements') this.updateOrganization({ announcements: d.value })
      this.store.setState({ dialog: null })
      const destructive = ['delete-account', 'delete-organization'].includes(d.kind)
      this.report(destructive ? 'Deletion requested' : 'Changes applied')
    })
  }
  updateRole = (value: Partial<Pick<RoleDraft, 'name' | 'description'>>) =>
    this.store.setState((s) => ({ roleDraft: s.roleDraft ? { ...s.roleDraft, ...value } : null }))
  setPolicyMode = (mode: RoleDraft['mode']) =>
    this.store.setState((s) => ({
      roleDraft:
        s.roleDraft && !(mode === 'visual' && (s.roleDraft.parsing || s.roleDraft.diagnostics.length))
          ? { ...s.roleDraft, mode }
          : s.roleDraft,
    }))
  setPolicySource = (source: string) => {
    const revision = ++this.revision
    this.store.setState((s) => ({ roleDraft: s.roleDraft ? { ...s.roleDraft, source, parsing: true } : null }))
    if (this.parseTimer) {
      clearTimeout(this.parseTimer)
      this.timers.delete(this.parseTimer)
    }
    const timer = setTimeout(() => {
      this.timers.delete(timer)
      void project(source)
        .then((result) => {
          if (this.closed || revision !== this.revision) return
          this.store.setState((s) => ({
            roleDraft: s.roleDraft
              ? {
                  ...s.roleDraft,
                  source,
                  parsing: false,
                  diagnostics: result.diagnostics,
                  projection: result.diagnostics.length ? s.roleDraft.projection : result.projection,
                }
              : null,
          }))
        })
        .catch(() => {
          if (this.closed || revision !== this.revision) return
          this.store.setState((s) => ({
            roleDraft: s.roleDraft
              ? { ...s.roleDraft, parsing: false, diagnostics: ['Policy parser is unavailable. Your draft is retained.'] }
              : null,
          }))
        })
    }, 150)
    this.parseTimer = timer
    this.timers.add(timer)
  }
  togglePolicyAction = (action: string) => {
    const draft = this.store.getState().roleDraft
    this.setPolicyActions([action], !draft?.projection.some((p) => p.action === action))
  }
  setPolicyActions = (actions: string[], enabled: boolean) => {
    const state = this.store.getState(),
      draft = state.roleDraft
    if (!draft || draft.parsing || draft.diagnostics.length || draft.builtin || state.readonly || state.busy) return
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
    const draft = this.store.getState().roleDraft
    if (!draft || draft.parsing || draft.diagnostics.length) return
    const source = draft.source
    void cedar().then((c) => {
      if (this.closed || this.store.getState().roleDraft?.source !== source) return
      const result = c.formatPolicies({ policyText: source, lineWidth: 80, indentWidth: 2 })
      if (result.type === 'success') this.setPolicySource(result.formatted_policy)
    })
  }
  saveRole = () => {
    const d = this.store.getState().roleDraft
    if (!d || !d.name.trim() || !d.source.trim() || d.parsing || d.diagnostics.length || d.builtin) return
    this.commit(() => {
      const id = d.id ?? crypto.randomUUID(),
        roles = this.store.getState().roles
      const role = {
        id,
        name: d.name.trim(),
        description: d.description,
        source: d.source,
        color: roles.find((r) => r.id === id)?.color ?? colors[roles.length % colors.length],
      }
      this.store.setState({
        roles: roles.some((r) => r.id === id) ? roles.map((r) => (r.id === id ? role : r)) : [...roles, role],
        roleDraft: { ...d, id },
        location: { page: 'role', id },
      })
      this.report(d.id ? 'Role saved' : 'Role created')
    })
  }
  resetRole = () => {
    const d = this.store.getState().roleDraft
    this.store.setState({ roleDraft: null })
    this.navigate(d?.id ? 'role' : 'general', d?.id ?? undefined)
  }
  updateGroup = (value: Partial<Pick<GroupDraft, 'name' | 'description'>>) =>
    this.store.setState((s) => ({ groupDraft: s.groupDraft ? { ...s.groupDraft, ...value } : null }))
  toggleGroupMember = (id: string) => {
    if (this.store.getState().readonly || this.store.getState().busy) return
    this.store.setState((s) => ({
      groupDraft: s.groupDraft
        ? {
            ...s.groupDraft,
            memberIds: s.groupDraft.memberIds.includes(id)
              ? s.groupDraft.memberIds.filter((value) => value !== id)
              : [...s.groupDraft.memberIds, id],
          }
        : null,
    }))
  }
  toggleGroupRole = (id: string) => {
    if (this.store.getState().readonly || this.store.getState().busy) return
    this.store.setState((s) => ({
      groupDraft: s.groupDraft
        ? {
            ...s.groupDraft,
            roleIds: s.groupDraft.roleIds.includes(id)
              ? s.groupDraft.roleIds.filter((value) => value !== id)
              : [...s.groupDraft.roleIds, id],
          }
        : null,
    }))
  }
  saveGroup = () => {
    const d = this.store.getState().groupDraft
    if (!d?.name.trim()) return
    this.commit(() => {
      const id = d.id ?? crypto.randomUUID(),
        groups = this.store.getState().groups
      const group = {
        ...d,
        id,
        name: d.name.trim(),
        color: groups.find((g) => g.id === id)?.color ?? colors[groups.length % colors.length],
      }
      this.store.setState({
        groups: groups.some((g) => g.id === id) ? groups.map((g) => (g.id === id ? group : g)) : [...groups, group],
        groupDraft: { ...d, id },
        location: { page: 'group', id },
      })
      this.report(d.id ? 'Group saved' : 'Group created')
    })
  }
  resetGroup = () => {
    const draft = this.store.getState().groupDraft
    this.store.setState({ groupDraft: null })
    this.navigate(draft?.id ? 'group' : 'general', draft?.id ?? undefined)
  }
  toggleMemberRole = (memberId: string, roleId: string) => {
    if (this.store.getState().readonly) return
    this.store.setState((s) => ({
      members: s.members.map((m) =>
        m.id !== memberId
          ? m
          : { ...m, roleIds: m.roleIds.includes(roleId) ? m.roleIds.filter((id) => id !== roleId) : [...m.roleIds, roleId] },
      ),
    }))
  }
  copy = (value: string) => {
    void navigator.clipboard
      ?.writeText(value)
      .then(() => this.report('Copied'))
      .catch(() => this.store.setState({ error: 'Could not copy. Select and copy the value directly.' }))
  }
  signOut = () => this.report('Sign out requested')
  leaveSettings = () => this.report('Back to app requested')
  dismissNotice = () => this.store.setState({ notice: null })
}
