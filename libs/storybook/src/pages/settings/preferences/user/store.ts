import { createFormStore } from '../../../../components/form'
import { initialPreferenceDraft } from './values'
import type { UserPreferenceState } from './controller'

export function createUserPreferenceStore(initial: Partial<UserPreferenceState> = {}) {
  return createFormStore<UserPreferenceState>(
    {
      draft: initialPreferenceDraft(),
      saved: null,
      status: 'idle',
      ...initial,
    },
    ['draft'],
  )
}
