import type { SettingsDialog, SettingsDialogKind, SettingsState } from '@compallm/storybook/pages/settings'
import type { MutationGenqlSelection } from '@compallm/contract/graphql'
import type { StoreApi } from 'zustand/vanilla'
import { graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'

export class DialogsController {
  private readonly store: StoreApi<SettingsState>
  private readonly roleDeleted: (id: string) => void
  constructor(store: StoreApi<SettingsState>, roleDeleted: (id: string) => void) {
    this.store = store
    this.roleDeleted = roleDeleted
  }
  openDialog = (kind: SettingsDialogKind, targetId?: string) => {
    if (kind === 'delete-role' || kind === 'customize-sidebar')
      this.store.setState({ dialog: { kind, targetId, name: '', value: '', phrase: '' } })
  }
  closeDialog = () => {
    if (!this.store.getState().busy) this.store.setState({ dialog: null })
  }
  confirmDialog = (dialog: SettingsDialog) => {
    const { busy } = this.store.getState()
    if (busy || dialog?.kind !== 'delete-role' || !dialog.targetId) return
    const id = dialog.targetId
    this.store.setState({ busy: true })
    void (async () => {
      try {
        const selection = { roleDelete: { __args: { id }, node: true } } satisfies MutationGenqlSelection
        const result = await graphql().mutation(selection)
        if (!result.roleDelete.node) throw new Error('Role deletion was not confirmed')
        this.store.setState((state) => ({ dialog: state.dialog === dialog ? null : state.dialog }))
        this.roleDeleted(id)
      } catch (error) {
        notify.apiError(error, () => {
          if (this.store.getState().dialog === dialog) this.confirmDialog(dialog)
        })
      } finally {
        this.store.setState({ busy: false })
      }
    })()
  }
  copy = (value: string) => {
    void navigator.clipboard.writeText(value).catch(notify.error)
  }
}
