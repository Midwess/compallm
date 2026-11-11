import { createStore } from 'zustand/vanilla'
import type { RoleState } from './controller'

export function createRoleStore(initial: Partial<RoleState> = {}) {
  return createStore<RoleState>()(() => ({ draft: null, busy: false, error: null, query: '', filters: {}, ...initial }))
}
