import { createStore } from 'zustand/vanilla'
import type { AuthState } from './types'

export function createAuthStore(initial: Partial<AuthState> = {}) {
  return createStore<AuthState>()(() => ({
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
  }))
}
