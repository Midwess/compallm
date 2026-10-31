import { createSignupStore, type SignupController } from '../../src/pages/signup'
import { createSigninStore, type SigninController } from '../../src/pages/signin'
import type { AuthInitialState, AuthOperation, AuthResult, AuthState } from '../../src/pages/auth/types'

export type AuthDemoScenario = 'default' | 'network-error' | 'throttled' | 'expired' | 'pending'

function initialValues(initial?: AuthInitialState): Partial<AuthState> {
  return {
    step: initial?.step ?? 'email',
    email: initial?.email ?? '',
    error: initial?.error ?? null,
    code: initial?.step === 'code' ? (initial.code ?? '') : '',
    remaining: initial?.step === 'code' ? (initial.retryAfterSeconds ?? 0) : 0,
  }
}

export class DemoAuthController implements SignupController, SigninController {
  readonly store: ReturnType<typeof createSignupStore>
  private readonly mode: 'signup' | 'signin'
  private readonly scenario: AuthDemoScenario
  private generation = 0
  private lastAttempt: string | undefined
  private failedOperation: AuthOperation | null = null
  private attempts = 0
  private retryAt = 0
  private timer: ReturnType<typeof setInterval> | undefined

  constructor(mode: 'signup' | 'signin', scenario: AuthDemoScenario = 'default', initial?: AuthInitialState) {
    this.mode = mode
    this.scenario = scenario
    this.store = (mode === 'signup' ? createSignupStore : createSigninStore)(initialValues(initial))
    this.lastAttempt = initial?.step === 'code' && initial.error ? initial.code : undefined
  }

  start = () => {
    this.retryAt = Date.now() + this.store.getState().remaining * 1000
    this.scheduleCountdown()
    return this.stop
  }

  stop = () => {
    this.generation += 1
    if (this.timer !== undefined) clearInterval(this.timer)
    this.timer = undefined
    // A stopped pending demo can be mounted again safely (including React StrictMode).
    this.store.setState({ pending: null })
  }

  private scheduleCountdown() {
    if (this.timer !== undefined) clearInterval(this.timer)
    this.timer = undefined
    if (this.store.getState().remaining === 0) return
    this.timer = setInterval(() => {
      const remaining = Math.max(0, Math.ceil((this.retryAt - Date.now()) / 1000))
      if (remaining !== this.store.getState().remaining) this.store.setState({ remaining })
      if (remaining === 0 && this.timer !== undefined) {
        clearInterval(this.timer)
        this.timer = undefined
      }
    }, 250)
  }

  private applyDelay(result: AuthResult) {
    if (result.retryAfterSeconds === undefined) return
    const remaining = Math.max(0, result.retryAfterSeconds)
    this.retryAt = Date.now() + remaining * 1000
    this.store.setState({ remaining })
    this.scheduleCountdown()
  }

  onEmailChanged = (email: string) => {
    if (this.store.getState().pending) return
    this.store.setState({ email, error: null })
  }

  onInvalidEmail = () => this.store.setState({ error: 'invalid-email' })

  onCodeChanged = (code: string) => {
    const state = this.store.getState()
    if (state.pending || state.code === code || state.step !== 'code') return
    this.store.setState({ code, error: null })
    void this.verifyCode()
  }

  requestCode = () => this.request('request')
  resendCode = () => this.request('resend')
  showResend = () => this.store.setState({ resendRevealed: true })

  useDifferentEmail = () => {
    if (this.store.getState().pending) return
    this.generation += 1
    this.lastAttempt = undefined
    this.failedOperation = null
    this.store.setState({ step: 'email', code: '', error: null, pending: null, resendRevealed: false })
  }

  retry = async () => {
    if (this.store.getState().error !== 'unavailable') return
    if (this.failedOperation === 'resend') await this.resendCode()
    else if (this.failedOperation === 'request') await this.requestCode()
    else await this.verifyCode(true)
  }

  private async request(operation: 'request' | 'resend') {
    const state = this.store.getState()
    if (state.pending || state.remaining > 0 || state.step === 'complete') return
    if ((operation === 'request' && state.step !== 'email') || (operation === 'resend' && state.step !== 'code')) return
    const generation = ++this.generation
    const email = state.email.trim().replace(/[A-Z]/g, (character) => character.toLowerCase())
    this.store.setState({ email, error: null, pending: operation })
    try {
      if (this.scenario === 'pending') await new Promise<never>(() => {})
      await new Promise((resolve) => setTimeout(resolve, 500))
      if (generation !== this.generation) return
      const result: AuthResult =
        this.mode === 'signup' && email === 'registered@example.com'
          ? { ok: false, reason: 'email-registered' }
          : this.scenario === 'throttled'
            ? { ok: false, reason: 'throttled', retryAfterSeconds: 5 }
            : { ok: true, retryAfterSeconds: 5 }
      this.applyDelay(result)
      if (result.ok) {
        this.lastAttempt = undefined
        this.failedOperation = null
        this.store.setState({ step: 'code', code: '', error: null, resendRevealed: false })
      } else {
        this.failedOperation = operation
        this.store.setState({ error: result.reason })
      }
    } finally {
      if (generation === this.generation) this.store.setState({ pending: null })
    }
  }

  private async verifyCode(retry = false) {
    const state = this.store.getState()
    if (state.pending || state.step !== 'code' || !/^[0-9]{6}$/.test(state.code)) return
    if (!retry && this.lastAttempt === state.code) return
    this.lastAttempt = state.code
    const generation = ++this.generation
    this.store.setState({ error: null, pending: 'verify' })
    try {
      if (this.scenario === 'pending') await new Promise<never>(() => {})
      await new Promise((resolve) => setTimeout(resolve, 500))
      if (generation !== this.generation) return
      if (this.scenario === 'network-error' && this.attempts++ === 0) {
        this.failedOperation = 'verify'
        this.store.setState({ error: 'unavailable' })
      } else if (this.scenario !== 'expired' && state.code === '123456') {
        this.failedOperation = null
        this.store.setState({ step: 'complete' })
      } else {
        this.failedOperation = 'verify'
        this.store.setState({ error: this.scenario === 'expired' ? 'expired-code' : 'incorrect-code' })
      }
    } finally {
      if (generation === this.generation) this.store.setState({ pending: null })
    }
  }
}
