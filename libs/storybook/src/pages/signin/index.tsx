import { AuthenticationPage } from '../auth/auth-page'
import { createAuthStore } from '../auth/store'
import type { AuthController, AuthState } from '../auth/types'

export type SigninController = AuthController
export type SigninState = AuthState

export function createSigninStore(initial: Partial<SigninState> = {}) {
  return createAuthStore(initial)
}

export type SigninStore = ReturnType<typeof createSigninStore>

export function SigninPage() {
  return <AuthenticationPage mode="signin" />
}
