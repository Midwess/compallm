import { useStore } from 'zustand'
import type { StoreApi } from 'zustand/vanilla'
import { useSettingsController } from '../controller'
import type { RoleDraft } from '../types'

export interface RoleState {
  query: string
  filters: Record<string, string>
  draft: RoleDraft | null
  busy: boolean
  error: string | null
}
export interface RoleController {
  setQuery(value: string): void
  setFilter(key: string, value: string): void
  store: StoreApi<RoleState>
  updateRole(value: Partial<Pick<RoleDraft, 'name' | 'description'>>): void
  setPolicyMode(mode: RoleDraft['mode']): void
  setPolicySource(source: string): void
  togglePolicyAction(action: string): void
  setPolicyActions(actions: string[], enabled: boolean): void
  formatPolicy(): void
  saveRole(): void
  resetRole(): void
}
export function useRoleController() {
  return useSettingsController().role
}
export function useRole<T>(selector: (state: RoleState) => T) {
  return useStore(useRoleController().store, selector)
}
