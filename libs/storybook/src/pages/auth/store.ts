import { createFormStore } from '../../components/form'
import type { AuthState } from './types'

export function createAuthStore(initial: Partial<AuthState> = {}) {
  return createFormStore<AuthState>(
    {
      currentUser: null,
      status: 'unknown',
      step: 'email',
      email: '',
      code: '',
      error: null,
      serverError: null,
      pending: null,
      remaining: 0,
      resendRevealed: false,
      ...initial,
    },
    ['code'],
  )
}
