import { createSettingsStore } from '../../../src/pages/settings'
import type {
  SettingsController,
  SettingsState,
  SettingsDestination,
  SettingsDialogKind,
  SettingsDialog,
} from '../../../src/pages/settings'
import type { GroupDraft, RoleDraft } from '../../../src/pages/settings'
import { initialSettings } from './fixtures'
import { DemoRoleController } from './role/demo-controller'
import { DemoGroupController } from './group/demo-controller'
import { DemoProfileController } from './profile/demo-controller'
import { DemoOrganizationController } from './organization/demo-controller'

export class DemoSettingsController implements SettingsController {
  store = createSettingsStore(structuredClone(initialSettings))
  pages: SettingsController['pages'] = {
    search: { setQuery: (value) => this.setQuery(value), setFilter: (key, value) => this.setFilter(key, value) },
    preferences: { setPreference: (key, value) => this.setPreference(key, value) },
    profile: new DemoProfileController(this.store, (file) => this.avatarUrl(file)),
    organization: new DemoOrganizationController(this.store, (file) => this.avatarUrl(file)),
    members: { toggleMemberRole: (member, role) => this.toggleMemberRole(member, role) },
    dialogs: {
      openDialog: (kind, id) => this.openDialog(kind, id),
      updateDialog: (value) => this.updateDialog(value),
      closeDialog: () => this.closeDialog(),
      confirmDialog: () => this.confirmDialog(),
      copy: (value) => this.copy(value),
    },
  }
  private timers = new Set<ReturnType<typeof setTimeout>>()
  private urls: string[] = []
  private verificationTimer: ReturnType<typeof setTimeout> | undefined
  private closed = false
  readonly role: DemoRoleController
  readonly group: DemoGroupController
  constructor(
    initial: Partial<SettingsState> & { roleDraft?: RoleDraft | null; groupDraft?: GroupDraft | null } = {},
    private failWrites: boolean | 'conflict' = false,
  ) {
    const { roleDraft, groupDraft, ...shared } = initial
    this.store.setState(structuredClone(shared))
    this.role = new DemoRoleController(this, failWrites)
    this.group = new DemoGroupController(this, failWrites)
    const location = this.store.getState().location
    this.navigate(location.page, location.id)
    if (roleDraft) this.role.store.setState({ draft: structuredClone(roleDraft) })
    if (groupDraft) this.group.store.setState({ draft: structuredClone(groupDraft) })
    if (initial.filters) {
      this.store.setState({ filters: structuredClone(initial.filters) })
      this.role.store.setState({ filters: structuredClone(initial.filters) })
      this.group.store.setState({ query: initial.filters['group-members-query'] ?? '' })
    }
    if (initial.query !== undefined) {
      this.store.setState({ query: initial.query })
      this.role.store.setState({ query: initial.query })
    }
    if (initial.dialog) this.store.setState({ dialog: initial.dialog })
  }
  start = () => {
    this.closed = false
    this.role.start()
    this.group.start()
  }
  stop = () => {
    this.closed = true
    this.role.stop()
    this.group.stop()
    this.timers.forEach(clearTimeout)
    this.urls.forEach(URL.revokeObjectURL)
  }
  report(notice: string) {
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
    this.role.cancel()
    this.store.setState({ location: { page, id }, query: '', filters: {}, error: null, dialog: null })
    if (page === 'new-role' || page === 'new-group') this.store.setState({ navigationQuery: '' })
    if (page === 'role' || page === 'new-role') this.role.open(page === 'role' ? id : undefined)
    if (page === 'group' || page === 'new-group') this.group.open(page === 'group' ? id : undefined)
  }
  setQuery = (query: string) => this.store.setState({ query })
  setNavigationQuery = (navigationQuery: string) => this.store.setState({ navigationQuery })
  setFilter = (key: string, value: string) => this.store.setState((s) => ({ filters: { ...s.filters, [key]: value } }))
  setPreference = (key: string, value: string | boolean) =>
    this.store.setState((s) => ({ preferences: { ...s.preferences, [key]: value } }))
  private avatarUrl = (file: File | null) => {
    const url = file ? URL.createObjectURL(file) : null
    if (url) this.urls.push(url)
    return url
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
        }))
      if (d.kind === 'remove-member')
        this.group.store.setState((s) => ({
          draft: s.draft ? { ...s.draft, memberIds: s.draft.memberIds.filter((member) => member !== id) } : null,
        }))
      if (d.kind === 'remove-passkey') this.store.setState((p) => ({ passkeys: p.passkeys.filter((key) => key.id !== id) }))
      if (d.kind === 'application')
        this.store.setState((p) => ({ applications: p.applications.filter((app) => app.id !== id) }))
      if (d.kind === 'revoke-key') this.store.setState((p) => ({ apiKeys: p.apiKeys.filter((key) => key.id !== id) }))
      if (d.kind === 'revoke-sessions') this.store.setState({ hasOtherSessions: false })
      if (d.kind === 'delete-role') {
        this.role.store.setState({ draft: null })
        this.group.store.setState({ draft: null })
        this.store.setState((p) => ({
          roles: p.roles.filter((r) => r.id !== id),
          groups: p.groups.map((g) => ({ ...g, roleIds: g.roleIds.filter((role) => role !== id) })),
          members: p.members.map((m) => ({ ...m, roleIds: m.roleIds.filter((role) => role !== id) })),
          location: { page: 'general' },
        }))
      }
      if (d.kind === 'delete-group') this.group.store.setState({ draft: null })
      if (d.kind === 'delete-group')
        this.store.setState((p) => ({
          groups: p.groups.filter((g) => g.id !== id),
          location: { page: 'general' },
        }))
      if (d.kind === 'remove-group-member') this.group.toggleGroupMember(id!)
      if (d.kind === 'organization-limit') this.store.setState({ organizationLimitUsd: d.value || null })
      if (d.kind === 'member-limit')
        this.store.setState((p) =>
          id
            ? { members: p.members.map((m) => (m.id === id ? { ...m, limitUsd: d.value || null } : m)) }
            : { memberDefaultLimitUsd: d.value || null },
        )
      if (d.kind === 'reset-schedule') this.store.setState({ timezone: d.value })
      if (d.kind === 'welcome') this.pages.organization.updateOrganization({ welcome: d.value })
      if (d.kind === 'announcements') this.pages.organization.updateOrganization({ announcements: d.value })
      this.store.setState({ dialog: null })
      const destructive = ['delete-account', 'delete-organization'].includes(d.kind)
      this.report(destructive ? 'Deletion requested' : 'Changes applied')
    })
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
