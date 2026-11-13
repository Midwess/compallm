import { createFormStore } from '../../../components/form'
import type { GroupState } from './controller'

export function createGroupStore(initial: Partial<GroupState> = {}) {
  return createFormStore<GroupState>({ draft: null, busy: false, error: null, query: '', ...initial }, ['draft'])
}
