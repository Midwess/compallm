import type { FormStore } from '../../../components/form'

import type { GroupDraft } from '../types'

export interface GroupState {
  loading?: boolean
  membersLoaded?: boolean
  membersLoadError?: boolean
  membersLoading?: boolean
  members?: { id: string; name: string; email: string; available: boolean }[]
  roles?: import('../types').SettingsRole[]
  memberCount?: number
  hasNextPage?: boolean
  query: string
  draft: GroupDraft | null
  busy: boolean
  error: string | null
}
export interface GroupController {
  setQuery(value: string): void
  store: FormStore<GroupState>
  loadMore?(): void
  rolesChanged?(ids: string[]): void
  updateGroup(draft: GroupDraft, names: string[], signal: AbortSignal): Promise<GroupDraft | false>
}

import { getSettingsController } from '../../../context'
import { readStore } from '../../page-state.svelte'
export function useGroupController() {
  return getSettingsController().pages.group
}
export function useGroup<T>(selector: (state: GroupState) => T, names: readonly string[] = []) {
  return readStore(useGroupController().store, selector, names)
}
