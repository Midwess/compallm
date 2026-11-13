import { createFormStore } from '../../../components/form'
import type { RoleState } from './controller'

export function createRoleStore(initial: Partial<RoleState> = {}) {
  return createFormStore<RoleState>({ draft: null, busy: false, error: null, query: '', filters: {}, ...initial }, ['draft'])
}
