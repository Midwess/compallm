import { useStore } from 'zustand'
import type { FormStore } from '../../../components/form'
import { useSettingsController } from '../controller'
import type { RoleDraft } from '../types'

export interface RoleState {
  query: string
  filters: Record<string, string>
  draft: RoleDraft | null
  loading?: boolean
  busy: boolean
  disabledPolicies?: Record<string, string[]>
  error: string | null
}
export interface RoleController {
  setQuery(value: string): void
  setFilter(key: string, value: string): void
  store: FormStore<RoleState>
  updateMetadata?(
    value: Partial<Pick<RoleDraft, 'name' | 'description'>>,
    signal?: AbortSignal,
  ): Promise<Pick<RoleDraft, 'name' | 'description'>>
  setPolicyMode(mode: RoleDraft['mode']): void
  setPolicySource(source: string): void
  commitPolicySource?(source: string): boolean | Promise<boolean>
  togglePolicyAction(action: string): void | Promise<void>
  setPolicyActions(actions: string[], enabled: boolean): void | Promise<void>
  formatPolicy(): void
  saveRole(): void
  resetRole(): void
}
export function useRoleController() {
  return useSettingsController().pages.role
}
export function useRole<T>(selector: (state: RoleState) => T) {
  return useStore(useRoleController().store, selector)
}
