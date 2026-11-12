import type { SettingsState } from '@compallm/storybook/pages/settings'
import type { StoreApi } from 'zustand/vanilla'

export class DialogsController {
  private readonly store: StoreApi<SettingsState>
  constructor(store: StoreApi<SettingsState>) {
    this.store = store
  }
  openDialog = () => {}
  updateDialog = () => {}
  closeDialog = () => this.store.setState({ dialog: null })
  confirmDialog = () => {}
  copy = () => {}
}
