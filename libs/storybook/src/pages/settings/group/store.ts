import { createStore } from 'zustand/vanilla'
import type { GroupState } from './controller'

export function createGroupStore(initial: Partial<GroupState> = {}) {
  return createStore<GroupState>()(() => ({ draft: null, busy: false, error: null, query: '', ...initial }))
}
