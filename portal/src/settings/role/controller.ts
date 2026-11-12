import { createRoleStore, type RoleController as Contract } from '@compallm/storybook/pages/settings'
import { notify } from '../../notifications/toast'

export class RoleController implements Contract {
  readonly store = createRoleStore()
  setQuery = (query: string) => this.store.setState({ query })
  setFilter = (key: string, value: string) => this.store.setState((s) => ({ filters: { ...s.filters, [key]: value } }))
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
  updateRole = this.unavailable
  setPolicyMode = this.unavailable
  setPolicySource = this.unavailable
  togglePolicyAction = this.unavailable
  setPolicyActions = this.unavailable
  formatPolicy = this.unavailable
  saveRole = this.unavailable
  resetRole = this.unavailable
}
