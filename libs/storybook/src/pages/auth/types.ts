import type { Session } from '@compallm/contract/graphql'
import type { FormStore } from '../../components/form'

export type AuthFailureReason = 'email-registered' | 'incorrect-code' | 'expired-code' | 'throttled' | 'unavailable'

export type AuthResult =
  { ok: true; retryAfterSeconds?: number } | { ok: false; reason: AuthFailureReason; retryAfterSeconds?: number }

export type AuthInitialState =
  | { step: 'email'; email?: string; error?: AuthFailureReason }
  | { step: 'code'; email: string; code?: string; error?: AuthFailureReason; retryAfterSeconds?: number }

export type AuthOperation = 'request' | 'verify' | 'resend' | 'passkey'
export type AuthError = AuthFailureReason | 'invalid-email'

export type AuthUser = Pick<Session, 'id' | 'userId' | 'organizationId' | 'organizationName'> &
  Partial<Pick<Session, 'avatarUrl' | 'ipAddress' | 'userAgent' | 'signedInAt' | 'lastSeenAt'>>

export interface AuthState {
  verificationActive?: boolean
  lastLoginMethod?: 'email' | 'passkey' | null
  currentUser: AuthUser | null
  status: 'unknown' | 'checking' | 'authenticated' | 'anonymous' | 'error'
  step: 'choose' | 'email' | 'code' | 'passkey' | 'complete'
  email: string
  code: string
  error: AuthError | null
  serverError: string | null
  pending: AuthOperation | null
  remaining: number
  resendRevealed: boolean
}

export type AuthStore = FormStore<AuthState>

export interface AuthController {
  readonly store: AuthStore
  readonly links?: { signup?: string; signin?: string }
  readonly navigate?: (href: string) => void
  start(mode: 'signup' | 'signin'): () => void
  stop(): void
  signOut(): Promise<void>
  ensureCurrentUser(): Promise<AuthUser | null>
  refreshCurrentUser(signal?: AbortSignal, redirectAnonymous?: boolean): Promise<AuthUser | null>
  verifyCode(code: string): Promise<void>
  clearError(): void
  onInvalidEmail(): void
  requestCode(email: string): Promise<void>
  requestVerification?(signal?: AbortSignal): Promise<boolean>
  cancelVerification?(): void
  verifyPasskey?(): Promise<void>
  useEmailCode?(): Promise<void>
  backToMethods?(): void
  resendCode(): Promise<void>
  retry(): Promise<void>
  showResend(): void
  useDifferentEmail(): void
}
