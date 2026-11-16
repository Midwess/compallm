import { AccessLoginPage } from '../auth/access-panel'
import { createAuthStore } from '../auth/store'
import type { AuthController, AuthState } from '../auth/types'

export type SignupController = AuthController
export type SignupState = AuthState

export function createSignupStore(initial: Partial<SignupState> = {}) {
  return createAuthStore(initial)
}

export type SignupStore = ReturnType<typeof createSignupStore>

export function SignupPage() {
  return <AccessLoginPage mode="signup" />
}
