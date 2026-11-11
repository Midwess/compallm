import type { SettingsState } from '@compallm/storybook/pages/settings'
import type { StoreApi } from 'zustand/vanilla'

export class SearchController {
  private readonly store: StoreApi<SettingsState>
  constructor(store: StoreApi<SettingsState>) {
    this.store = store
  }
  setQuery = (query: string) => this.store.setState({ query })
  setFilter = (key: string, value: string) => this.store.setState((state) => ({ filters: { ...state.filters, [key]: value } }))
}
