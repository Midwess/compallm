import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'
import { createAuthStore, createAuthForm } from '../../src/pages/auth/store'
import type { AuthController, AuthDraft, AuthMode, AuthOperation, AuthState } from '../../src/pages/auth/types'

export type AuthDemoScenario = 'default' | 'network-error' | 'throttled' | 'expired' | 'pending'
export interface AuthDemoOptions {
  mode?: AuthMode
  scenario?: AuthDemoScenario
  initial?: Partial<AuthState>
  draft?: Partial<AuthDraft>
  waiting?: boolean
  passkeySucceeds?: boolean
  onNavigate?: (mode: AuthMode) => void
}
export class DemoAuthController implements AuthController {
  readonly store
  readonly form
  readonly links = { signin: '#signin', signup: '#signup' }
  private generation = 0
  private request?: AbortController
  private timer?: ReturnType<typeof setInterval>
  private retryAt = 0
  private challengeUntil = 0
  private failedOperation: AuthOperation | null = null
  private verificationCode = ''
  private attempts = 0
  private releaseForm?: () => void
  private readonly onNavigate?: (mode: AuthMode) => void
  private readonly mode: AuthMode
  private readonly scenario: AuthDemoScenario
  private readonly waiting: boolean
  private readonly passkeySucceeds: boolean
  constructor(options: AuthDemoOptions = {}) {
    this.onNavigate = options.onNavigate
    this.mode = options.mode ?? 'signin'
    this.scenario = options.scenario ?? 'default'
    this.waiting = options.waiting ?? false
    this.passkeySucceeds = options.passkeySucceeds ?? false
    this.store = createAuthStore({ step: this.mode === 'signin' ? 'choose' : 'email', ...options.initial })
    this.form = createAuthForm(this, this.mode, options.draft)
  }
  start = () => {
    this.releaseForm = this.form.mount()
    this.retryAt = Date.now() + this.store.getState().remaining * 1000
    if (this.store.getState().step === 'code') this.challengeUntil = Date.now() + 600_000
    this.countdown()
    return this.stop
  }
  stop = () => {
    this.generation++
    this.request?.abort()
    this.request = undefined
    clearInterval(this.timer)
    this.timer = undefined
    this.releaseForm?.()
    this.releaseForm = undefined
    this.form.stop()
    this.store.setState({ pending: null })
  }
  signOut = async () => {
    this.stop()
    this.store.setState({ currentUser: null, status: 'anonymous' })
  }
  clearError = () => this.store.setState({ error: null, serverError: null })
  navigate = (href: string) => {
    this.onNavigate?.(href === this.links.signin ? 'signin' : 'signup')
  }
  private countdown() {
    clearInterval(this.timer)
    this.timer = undefined
    if (!this.store.getState().remaining) return
    this.timer = setInterval(() => {
      const remaining = Math.max(0, Math.ceil((this.retryAt - Date.now()) / 1000))
      if (remaining !== this.store.getState().remaining) this.store.setState({ remaining })
      if (!remaining) {
        clearInterval(this.timer)
        this.timer = undefined
      }
    }, 250)
  }
  private delay(seconds: number) {
    this.retryAt = Date.now() + seconds * 1000
    this.store.setState({ remaining: seconds })
    this.countdown()
  }
  private begin(operation: AuthOperation) {
    this.request?.abort()
    this.request = new AbortController()
    const generation = ++this.generation
    this.store.setState({ pending: operation, error: null, serverError: null })
    return { generation, signal: this.request.signal }
  }
  private wait(signal: AbortSignal, frozen = false) {
    return new Promise<boolean>((resolve) => {
      let timer: ReturnType<typeof setTimeout> | undefined
      const finish = (accepted: boolean) => {
        clearTimeout(timer)
        signal.removeEventListener('abort', abort)
        resolve(accepted)
      }
      const abort = () => finish(false)
      if (signal.aborted) return resolve(false)
      signal.addEventListener('abort', abort, { once: true })
      if (!frozen) timer = setTimeout(() => finish(true), 500)
    })
  }
  useEmailCode = async () => {
    this.generation++
    this.request?.abort()
    this.store.setState({
      step: this.challengeUntil > Date.now() ? 'code' : 'email',
      pending: null,
      error: null,
      serverError: null,
    })
  }
  backToMethods = () => {
    this.generation++
    this.request?.abort()
    this.store.setState({ step: 'choose', pending: null, error: null, serverError: null })
  }
  useDifferentEmail = () => {
    if (this.store.getState().pending) return
    this.challengeUntil = 0
    this.failedOperation = null
    this.form.reset({ email: this.store.getState().email, code: '' })
    this.store.setState({ step: 'email', error: null, serverError: null })
  }
  requestCode = (email: string) => this.send('request', email)
  resendCode = () => this.send('resend', this.store.getState().email)
  private async send(operation: 'request' | 'resend', email: string) {
    const state = this.store.getState()
    if (state.pending || state.remaining > 0 || state.step === 'complete') return
    if (operation === 'request' ? state.step !== 'email' : state.step !== 'code') return
    const parsed = emailRequestSchema(this.mode).safeParse({ email })
    if (!parsed.success) return
    const { generation, signal } = this.begin(operation)
    if (!(await this.wait(signal, this.scenario === 'pending')) || generation !== this.generation) return
    if (this.mode === 'signup' && parsed.data.email === 'registered@example.com') {
      this.store.setState({
        error: 'email-registered',
        serverError: 'Email already registered. Sign in instead.',
        pending: null,
      })
      return
    }
    if (this.scenario === 'throttled') {
      this.delay(5)
      this.store.setState({ error: 'throttled', pending: null })
      return
    }
    this.challengeUntil = Date.now() + 600_000
    this.failedOperation = null
    this.form.set('code', '')
    this.delay(5)
    this.store.setState({ step: 'code', email: parsed.data.email, pending: null })
  }
  verifyCode = async (code: string) => {
    const state = this.store.getState()
    if (state.pending || state.step !== 'code') return
    const parsed = otpRequestSchema(this.mode).safeParse({ code })
    if (!parsed.success) return
    this.verificationCode = parsed.data.code
    const { generation, signal } = this.begin('verify')
    if (!(await this.wait(signal, this.scenario === 'pending')) || generation !== this.generation) return
    if (this.scenario === 'network-error' && this.attempts++ === 0) {
      this.failedOperation = 'verify'
      this.store.setState({ error: 'unavailable', serverError: 'Could not verify your code. Please try again.', pending: null })
    } else if (this.scenario === 'expired') this.store.setState({ error: 'expired-code', pending: null })
    else if (parsed.data.code !== '123456') this.store.setState({ error: 'incorrect-code', pending: null })
    else {
      this.failedOperation = null
      this.store.setState({ step: 'complete', pending: null, lastLoginMethod: 'email' })
    }
  }
  verifyPasskey = async () => {
    const { generation, signal } = this.begin('passkey')
    this.store.setState({ step: 'passkey' })
    if (!(await this.wait(signal, this.waiting)) || generation !== this.generation) return
    this.store.setState(
      this.passkeySucceeds
        ? { step: 'complete', pending: null, lastLoginMethod: 'passkey' }
        : { pending: null, serverError: 'Passkey verification was cancelled. Try again or continue with email.' },
    )
  }
  retry = async () => {
    if (this.failedOperation === 'verify') await this.verifyCode(this.verificationCode)
    else if (this.failedOperation === 'resend') await this.resendCode()
    else if (this.failedOperation === 'request') await this.requestCode(this.form.values.email)
  }
  cancelVerification = () => {
    this.request?.abort()
    this.generation++
    this.store.setState({ verificationActive: false, pending: null })
  }
}
