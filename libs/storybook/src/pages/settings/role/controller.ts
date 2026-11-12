import { useStore } from 'zustand'
import type { StoreApi } from 'zustand/vanilla'
import { useSettingsController } from '../controller'
import type { RoleDraft } from '../types'

export interface RoleState {
  query: string
  filters: Record<string, string>
  draft: RoleDraft | null
  busy: boolean
  policySaveFailed?: boolean
  disabledPolicies?: Record<string, string[]>
  error: string | null
}
export interface RoleController {
  setQuery(value: string): void
  setFilter(key: string, value: string): void
  store: StoreApi<RoleState>
  updateRole(value: Partial<Pick<RoleDraft, 'name' | 'description'>>): void
  commitRoleField?(field: 'name' | 'description'): void
  setPolicyMode(mode: RoleDraft['mode']): void
  setPolicySource(source: string): void
  commitPolicySource?(): void
  togglePolicyAction(action: string): void
  setPolicyActions(actions: string[], enabled: boolean): void
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
