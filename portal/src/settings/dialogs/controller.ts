import type { SettingsDialog, SettingsDialogKind, SettingsState } from '@compallm/storybook/pages/settings'
import type { MutationGenqlSelection } from '@compallm/contract/graphql'
import type { StoreApi } from 'zustand/vanilla'
import { graphql } from '../../graphql-client'
import { notify } from '../../notifications/toast'

export class DialogsController {
  private readonly store: StoreApi<SettingsState>
  private readonly roleDeleted: (id: string) => void
  private readonly revokeOtherSessions?: () => Promise<void>
  private readonly groupDeleted: (id: string) => void
  constructor(
    store: StoreApi<SettingsState>,
    roleDeleted: (id: string) => void,
    groupDeleted: (id: string) => void = () => {},
    revokeOtherSessions?: () => Promise<void>,
  ) {
    this.revokeOtherSessions = revokeOtherSessions
    this.groupDeleted = groupDeleted
    this.store = store
    this.roleDeleted = roleDeleted
  }
  openDialog = (kind: SettingsDialogKind, targetId?: string) => {
    if (kind === 'delete-group' || kind === 'delete-role' || kind === 'customize-sidebar' || kind === 'revoke-sessions')
      this.store.setState({ dialog: { kind, targetId, name: '', value: '', phrase: '' } })
  }
  closeDialog = () => {
    if (!this.store.getState().busy) this.store.setState({ dialog: null })
  }
  confirmDialog = (dialog: SettingsDialog) => {
    if (dialog.kind === 'revoke-sessions') {
      if (this.store.getState().busy || this.store.getState().dialog !== dialog || !this.revokeOtherSessions) return
      this.store.setState({ busy: true })
      void this.revokeOtherSessions()
        .then(() => this.store.setState({ dialog: null }))
        .catch(notify.error)
        .finally(() => this.store.setState({ busy: false }))
      return
    }
    const { busy, dialog: activeDialog } = this.store.getState()
    if (busy || (dialog?.kind !== 'delete-role' && dialog?.kind !== 'delete-group') || !dialog.targetId) return
    if (activeDialog?.kind !== dialog.kind || activeDialog.targetId !== dialog.targetId) return
    const id = dialog.targetId
    this.store.setState({ busy: true })
    void (async () => {
      try {
        if (dialog.kind === 'delete-group') {
          const selection = { groupDelete: { __args: { id }, node: true } } satisfies MutationGenqlSelection
          const result = await graphql().mutation(selection)
          if (!result.groupDelete.node) throw new Error('Group deletion was not confirmed')
          this.store.setState((state) => ({ dialog: state.dialog === activeDialog ? null : state.dialog }))
          this.groupDeleted(id)
          return
        }
        const selection = { roleDelete: { __args: { id }, node: true } } satisfies MutationGenqlSelection
        const result = await graphql().mutation(selection)
        if (!result.roleDelete.node) throw new Error('Role deletion was not confirmed')
        this.store.setState((state) => ({ dialog: state.dialog === activeDialog ? null : state.dialog }))
        this.roleDeleted(id)
      } catch (error) {
        notify.apiError(error, () => {
          if (this.store.getState().dialog === activeDialog) this.confirmDialog(activeDialog)
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
