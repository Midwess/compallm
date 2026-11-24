import type { FormStore } from '../../../components/form'

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

import { getSettingsController } from '../../../context'
import { readStore } from '../../page-state.svelte'
export function useRoleController() {
  return getSettingsController().pages.role
}
export function useRole<T>(selector: (state: RoleState) => T, names: readonly string[] = []) {
  return readStore(useRoleController().store, selector, names)
}
