import { AuthenticationPage } from '../auth/auth-page'
import type { AuthPageProps } from '../auth/types'
import { createAuthStore } from '../auth/store'
import type { AuthController, AuthState } from '../auth/types'

export type { AuthPageProps as SigninPageProps } from '../auth/types'
export type SigninController = AuthController
export type SigninState = AuthState

export function createSigninStore(initial: Partial<SigninState> = {}) {
  return createAuthStore(initial)
}

export type SigninStore = ReturnType<typeof createSigninStore>

export function SigninPage(props: AuthPageProps) {
  return <AuthenticationPage {...props} mode="signin" />
}
