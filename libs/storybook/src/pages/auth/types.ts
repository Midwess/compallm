import type { StoreApi } from 'zustand/vanilla'

export type AuthFailureReason = 'email-registered' | 'incorrect-code' | 'expired-code' | 'throttled' | 'unavailable'

export type AuthResult =
  { ok: true; retryAfterSeconds?: number } | { ok: false; reason: AuthFailureReason; retryAfterSeconds?: number }

export type AuthInitialState =
  | { step: 'email'; email?: string; error?: AuthFailureReason }
  | { step: 'code'; email: string; code?: string; error?: AuthFailureReason; retryAfterSeconds?: number }

export type AuthOperation = 'request' | 'verify' | 'resend'
export type AuthError = AuthFailureReason | 'invalid-email'

export interface AuthState {
  step: 'email' | 'code' | 'complete'
  email: string
  code: string
  error: AuthError | null
  pending: AuthOperation | null
  remaining: number
  resendRevealed: boolean
}

export type AuthStore = Pick<StoreApi<AuthState>, 'getState' | 'getInitialState' | 'subscribe'>

export interface AuthController {
  readonly store: AuthStore
  readonly links?: { signup?: string; signin?: string }
  onEmailChanged(email: string): void
  onCodeChanged(code: string): void
  onInvalidEmail(): void
  requestCode(): Promise<void>
  resendCode(): Promise<void>
  retry(): Promise<void>
  showResend(): void
  useDifferentEmail(): void
}

export interface AuthPageProps {
  controller: AuthController
}
