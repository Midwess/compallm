import { createGroupStore } from '../../../../src/pages/settings'
import type { GroupController, GroupDraft, SettingsController } from '../../../../src/pages/settings'

const colors = ['#A49ADA', '#6EB8A8', '#D7A466', '#8CAFD1']
export class DemoGroupController implements GroupController {
  readonly store = createGroupStore()
  loadMore = () => {
    this.store.setState({
      members: this.settings.store.getState().members.map((member) => ({ ...member, available: true })),
      hasNextPage: false,
    })
  }
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
            memberSelection: Object.fromEntries(
              this.settings.store.getState().members.map((member) => [member.id, group.memberIds.includes(member.id)]),
            ),
            roleIds: [...group.roleIds],
          }
        : { id: null, name: '', description: '', memberIds: [], roleIds: [] },
      error: null,
    })
  }
  toggleGroupMember = (id: string) =>
    this.store.form.change(`draft.memberSelection.${id}`, !this.store.getState().draft?.memberSelection?.[id])
  saveGroup = (draft: GroupDraft): Promise<GroupDraft | false> => {
    if (!draft?.name.trim() || this.store.getState().busy || this.settings.store.getState().readonly)
      return Promise.resolve(false)
    this.store.setState({ busy: true, error: null })
    return new Promise((resolve) => {
      this.saveTimer = setTimeout(() => {
        if (this.closed) {
          resolve(false)
          return
        }
        if (this.failWrites) {
          this.store.setState({
            busy: false,
            error:
              this.failWrites === 'conflict'
                ? 'This group changed elsewhere. Your draft is retained.'
                : 'Could not save changes. Your draft is retained.',
          })
          this.settings.store.setState({ error: 'Could not save changes. Your draft is retained.' })
          resolve(false)
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
        this.store.setState(draft.id ? { busy: false } : { draft: { ...draft, id }, busy: false })
        this.settings.report(draft.id ? 'Group saved' : 'Group created')
        resolve({ ...draft, id, name: group.name })
      }, 240)
    })
  }
  updateGroup = async (draft: GroupDraft, _names: string[], signal: AbortSignal) => {
    if (signal.aborted) return false as const
    return this.saveGroup({
      ...draft,
      memberIds: draft.memberSelection
        ? Object.keys(draft.memberSelection).filter((id) => draft.memberSelection?.[id])
        : draft.memberIds,
    })
  }
  resetGroup = () => {
    const draft = this.store.getState().draft
    this.store.setState({ draft: null })
    this.settings.navigate(draft?.id ? 'group' : 'general', draft?.id ?? undefined)
  }
}
