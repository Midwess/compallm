import { createGroupStore, type GroupController as Contract } from '@compallm/storybook/pages/settings'
import { notify } from '../../notifications/toast'

export class GroupController implements Contract {
  readonly store = createGroupStore()
  setQuery = (query: string) => this.store.setState({ query })
  open = () => {
    if (this.store.getState().draft) return
    this.store.setState({ draft: { id: null, name: '', description: '', memberIds: [], roleIds: [] } })
  }
  private unavailable = () => {
    notify.info('Settings changes are not available yet.')
    return false as const
  }
  toggleGroupMember = this.unavailable
  saveGroup = this.unavailable
  resetGroup = this.unavailable
}
