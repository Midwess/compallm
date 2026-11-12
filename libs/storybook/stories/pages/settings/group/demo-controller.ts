import { createGroupStore } from '../../../../src/pages/settings'
import type { GroupController, GroupDraft, SettingsController } from '../../../../src/pages/settings'

const colors = ['#A49ADA', '#6EB8A8', '#D7A466', '#8CAFD1']
export class DemoGroupController implements GroupController {
  readonly store = createGroupStore()
  setQuery = (query: string) => this.store.setState({ query })
  private saveTimer?: ReturnType<typeof setTimeout>
  private closed = false
  constructor(
    private readonly settings: Pick<SettingsController, 'store' | 'navigate'> & { report(notice: string): void },
    private readonly failWrites: boolean | 'conflict',
  ) {}
  start = () => {
    this.closed = false
  }
  stop = () => {
    this.closed = true
    clearTimeout(this.saveTimer)
  }
  open = (id?: string) => {
    this.store.setState({ query: '' })
    const group = id ? this.settings.store.getState().groups.find((item) => item.id === id) : undefined
    if (group && this.store.getState().draft?.id === group.id) return
    this.store.setState({
      draft: group
        ? {
            id: group.id,
            name: group.name,
            description: group.description,
            memberIds: [...group.memberIds],
            roleIds: [...group.roleIds],
          }
        : { id: null, name: '', description: '', memberIds: [], roleIds: [] },
      error: null,
    })
  }
  updateGroup = (value: Partial<Pick<GroupDraft, 'name' | 'description'>>) =>
    this.store.setState((s) => ({ draft: s.draft ? { ...s.draft, ...value } : null }))
  private toggle = (field: 'memberIds' | 'roleIds', id: string) => {
    if (this.settings.store.getState().readonly || this.store.getState().busy) return
    this.store.setState((s) => ({
      draft: s.draft
        ? {
            ...s.draft,
            [field]: s.draft[field].includes(id) ? s.draft[field].filter((value) => value !== id) : [...s.draft[field], id],
          }
        : null,
    }))
  }
  toggleGroupMember = (id: string) => this.toggle('memberIds', id)
  toggleGroupRole = (id: string) => this.toggle('roleIds', id)
  saveGroup = () => {
    const draft = this.store.getState().draft
    if (!draft?.name.trim() || this.store.getState().busy || this.settings.store.getState().readonly) return
    this.store.setState({ busy: true, error: null })
    this.saveTimer = setTimeout(() => {
      if (this.closed) return
      if (this.failWrites) {
        this.store.setState({
          busy: false,
          error:
            this.failWrites === 'conflict'
              ? 'This group changed elsewhere. Your draft is retained.'
              : 'Could not save changes. Your draft is retained.',
        })
        return
      }
      const id = draft.id ?? crypto.randomUUID(),
        groups = this.settings.store.getState().groups
      const group = {
        ...draft,
        id,
        name: draft.name.trim(),
        color: groups.find((g) => g.id === id)?.color ?? colors[groups.length % colors.length],
      }
      this.settings.store.setState({
        groups: groups.some((g) => g.id === id) ? groups.map((g) => (g.id === id ? group : g)) : [...groups, group],
        location: { page: 'group', id },
      })
      this.store.setState({ draft: { ...draft, id }, busy: false })
      this.settings.report(draft.id ? 'Group saved' : 'Group created')
    }, 240)
  }
  resetGroup = () => {
    const draft = this.store.getState().draft
    this.store.setState({ draft: null })
    this.settings.navigate(draft?.id ? 'group' : 'general', draft?.id ?? undefined)
  }
}
