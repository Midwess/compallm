import { AuthenticationPage } from '../auth/auth-page'
import type { AuthPageProps } from '../auth/types'
import { createAuthStore } from '../auth/store'
import type { AuthController, AuthState } from '../auth/types'

export type { AuthPageProps as SignupPageProps } from '../auth/types'
export type SignupController = AuthController
export type SignupState = AuthState

export function createSignupStore(initial: Partial<SignupState> = {}) {
  return createAuthStore(initial)
}

export type SignupStore = ReturnType<typeof createSignupStore>

export function SignupPage(props: AuthPageProps) {
  return <AuthenticationPage {...props} mode="signup" />
}
