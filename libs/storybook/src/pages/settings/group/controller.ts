import { useFormStore } from '../../../components/form'
import type { FormStore } from '../../../components/form'
import { useSettingsController } from '../controller'
import type { GroupDraft } from '../types'

export interface GroupState {
  loading?: boolean
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
export function useGroupController() {
  return useSettingsController().pages.group
}
export function useGroup<T>(selector: (state: GroupState) => T, names: readonly string[] = []) {
  return useFormStore(useGroupController().store, selector, names)
}
