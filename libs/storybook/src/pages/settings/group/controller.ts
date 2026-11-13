import { useFormStore } from '../../../components/form'
import type { FormStore } from '../../../components/form'
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
  store: FormStore<GroupState>
  toggleGroupMember(id: string): void
  saveGroup(draft: GroupDraft): void | false | GroupDraft | Promise<void | false | GroupDraft>
  resetGroup(): void
}
export function useGroupController() {
  return useSettingsController().group
}
export function useGroup<T>(selector: (state: GroupState) => T, names: readonly string[] = []) {
  return useFormStore(useGroupController().store, selector, names)
}
