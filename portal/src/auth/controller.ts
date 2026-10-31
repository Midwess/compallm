import { createSignupStore, type SignupController, type SignupState } from '@compallm/storybook/pages/signup'
import { confirmSigninOtp, confirmSignupOtp, signinEmail, signupEmail } from '../contract/api'
import { notify } from '../notifications/toast'

type Operation = NonNullable<SignupState['pending']>

export class AuthController implements SignupController {
  readonly store = createSignupStore()
  readonly links = { signup: '/', signin: '/login' }
  private generation = 0
  private lastCode: string | undefined
  private failedOperation: Operation | null = null
  private abort: AbortController | undefined

  private readonly mode: 'signup' | 'signin'

  readonly navigate?: (href: string) => void

  constructor(mode: 'signup' | 'signin', navigate?: (href: string) => void) {
    this.mode = mode
    this.navigate = navigate
  }

  start = () => this.stop
  stop = () => {
    this.generation += 1
    this.abort?.abort()
    this.store.setState({ pending: null })
  }

  onEmailChanged = (email: string) => {
    if (!this.store.getState().pending) this.store.setState({ email, error: null })
  }
  onInvalidEmail = () => this.store.setState({ error: 'invalid-email' })
  showResend = () => this.store.setState({ resendRevealed: true })
  onCodeChanged = (code: string) => {
    const state = this.store.getState()
    if (state.pending || state.step !== 'code' || state.code === code) return
    this.store.setState({ code, error: null })
    void this.verify()
  }
  useDifferentEmail = () => {
    if (this.store.getState().pending) return
    this.lastCode = undefined
    this.failedOperation = null
    this.store.setState({ step: 'email', code: '', error: null })
  }
  requestCode = () => this.request('request')
  resendCode = () => this.request('resend')
  retry = async () => {
    if (!this.failedOperation) return
    if (this.failedOperation === 'verify') await this.verify(true)
    else if (this.failedOperation) await this.request(this.failedOperation)
  }

  private async request(operation: 'request' | 'resend') {
    const state = this.store.getState()
    if (state.pending || state.step !== (operation === 'request' ? 'email' : 'code')) return
    const email = state.email.trim().replace(/[A-Z]/g, (letter) => letter.toLowerCase())
    this.store.setState({ email })
    await this.perform(
      operation,
      async (signal) => {
        const request = this.mode === 'signup' ? signupEmail : signinEmail
        await request({ email, delivery: 'cookie' }, { signal, credentials: 'same-origin' })
      },
      () => {
        this.lastCode = undefined
        this.store.setState({ step: 'code', code: '', error: null })
      },
    )
  }

  private async verify(retry = false) {
    const state = this.store.getState()
    if (state.pending || state.step !== 'code' || !/^[0-9]{6}$/.test(state.code)) return
    if (!retry && this.lastCode === state.code) return
    this.lastCode = state.code
    await this.perform(
      'verify',
      async (signal) => {
        const confirm = this.mode === 'signup' ? confirmSignupOtp : confirmSigninOtp
        await confirm({ code: state.code, delivery: 'cookie' }, { signal, credentials: 'same-origin' })
      },
      () => this.store.setState({ step: 'complete', code: '' }),
    )
  }

  private async perform(operation: Operation, request: (signal: AbortSignal) => Promise<void>, success: () => void) {
    const generation = ++this.generation
    this.abort = new AbortController()
    this.store.setState({ pending: operation, error: null })
    try {
      await request(this.abort.signal)
      if (generation !== this.generation) return
      this.failedOperation = null
      success()
    } catch (error) {
      if (generation !== this.generation) return
      this.failedOperation = operation
      this.lastCode = undefined
      notify.error(error)
    } finally {
      if (generation === this.generation) this.store.setState({ pending: null })
    }
  }
}
