import { useStore } from 'zustand'
import type { StoreApi } from 'zustand/vanilla'
import { useSettingsController } from '../controller'
import type { GroupDraft } from '../types'

export interface GroupState {
  query: string
  draft: GroupDraft | null
  busy: boolean
  error: string | null
}
export interface GroupController {
  setQuery(value: string): void
  store: StoreApi<GroupState>
  updateGroup(value: Partial<Pick<GroupDraft, 'name' | 'description'>>): void
  toggleGroupMember(id: string): void
  toggleGroupRole(id: string): void
  saveGroup(): void
  resetGroup(): void
}
export function useGroupController() {
  return useSettingsController().group
}
export function useGroup<T>(selector: (state: GroupState) => T) {
  return useStore(useGroupController().store, selector)
}
