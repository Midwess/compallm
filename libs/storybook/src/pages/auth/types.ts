import type { AuthSession2$result } from '@compallm/contract/houdini'
import type { FormModel } from '../../components/form-model.svelte'
import type { StateStore } from '../../state.svelte'

export type AuthMode = 'signup' | 'signin'
export type AuthFailureReason =
  'invalid-email' | 'email-registered' | 'incorrect-code' | 'expired-code' | 'throttled' | 'unavailable'
export type AuthOperation = 'request' | 'verify' | 'resend' | 'passkey'
export type AuthUser = AuthSession2$result['session']['node']
export interface AuthDraft {
  email: string
  code: string
}
export interface AuthState {
  currentUser: AuthUser | null
  status: 'unknown' | 'checking' | 'authenticated' | 'anonymous' | 'error'
  step: 'choose' | 'email' | 'code' | 'passkey' | 'complete'
  email: string
  error: AuthFailureReason | null
  serverError: string | null
  pending: AuthOperation | null
  remaining: number
  lastLoginMethod: 'email' | 'passkey' | null
  verificationActive: boolean
}
export type AuthStore = StateStore<AuthState>
export interface AuthController {
  signOut(): Promise<void>

  requestVerification?(signal?: AbortSignal): Promise<boolean>
  readonly store: AuthStore
  readonly form: FormModel<AuthDraft>
  readonly links?: { signup?: string; signin?: string }
  navigate?(href: string): void
  start(mode: AuthMode): () => void
  stop(): void
  requestCode(email: string): Promise<void>
  verifyCode(code: string): Promise<void>
  resendCode(): Promise<void>
  retry(): Promise<void>
  clearError(): void
  useDifferentEmail(): void
  useEmailCode(): Promise<void>
  backToMethods(): void
  verifyPasskey(): Promise<void>
  cancelVerification?(): void
}
