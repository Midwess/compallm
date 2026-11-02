import type { StoreApi } from 'zustand/vanilla'

export type AuthFailureReason = 'email-registered' | 'incorrect-code' | 'expired-code' | 'throttled' | 'unavailable'

export type AuthResult =
  { ok: true; retryAfterSeconds?: number } | { ok: false; reason: AuthFailureReason; retryAfterSeconds?: number }

export type AuthInitialState =
  | { step: 'email'; email?: string; error?: AuthFailureReason }
  | { step: 'code'; email: string; code?: string; error?: AuthFailureReason; retryAfterSeconds?: number }

export type AuthOperation = 'request' | 'verify' | 'resend'
export type AuthError = AuthFailureReason | 'invalid-email'

export interface AuthUser {
  userId: number
  sessionId: number
  organizationId: number
  organizationName: string
  organizationRole: string
}

export interface AuthState {
  currentUser: AuthUser | null
  status: 'unknown' | 'checking' | 'authenticated' | 'anonymous' | 'error'
  step: 'email' | 'code' | 'complete'
  email: string
  code: string
  error: AuthError | null
  serverError: string | null
  pending: AuthOperation | null
  remaining: number
  resendRevealed: boolean
}

export type AuthStore = Pick<StoreApi<AuthState>, 'getState' | 'getInitialState' | 'subscribe'>

export interface AuthController {
  readonly store: AuthStore
  readonly links?: { signup?: string; signin?: string }
  readonly navigate?: (href: string) => void
  start(mode: 'signup' | 'signin'): () => void
  stop(): void
  signOut(): Promise<void>
  ensureCurrentUser(): Promise<AuthUser | null>
  refreshCurrentUser(signal?: AbortSignal, redirectAnonymous?: boolean): Promise<AuthUser | null>
  onEmailChanged(email: string): void
  onCodeChanged(code: string): void
  onInvalidEmail(): void
  requestCode(): Promise<void>
  resendCode(): Promise<void>
  retry(): Promise<void>
  showResend(): void
  useDifferentEmail(): void
}
