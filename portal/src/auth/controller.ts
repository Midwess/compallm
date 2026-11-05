import { ConfirmSignupOtpRequest, ConfirmSigninOtpRequest } from '@compallm/contract/validation'
import { createSignupStore, type SignupController, type SignupState } from '@compallm/storybook/pages/signup'
import type { SigninController } from '@compallm/storybook/pages/signin'
import get from 'lodash-es/get'
import isObjectLike from 'lodash-es/isObjectLike'
import {
  confirmSigninOtp,
  confirmSignupOtp,
  currentUser,
  signinEmail,
  signOut,
  signupEmail,
  type CurrentUserResponse,
} from '../contract/api'
import { urlController } from '../url/controller'
import type { UrlController } from '@compallm/storybook'
import { notify, apiErrorMessage } from '../notifications/toast'
import { organizationHome } from '../organization/paths'

type Operation = NonNullable<SignupState['pending']>

const initialFormState = createSignupStore().getInitialState()

export class AuthController implements SignupController, SigninController {
  readonly store = createSignupStore()
  private sessionPending: Promise<CurrentUserResponse | null> | undefined
  private sessionRequest: AbortController | undefined
  private sessionGeneration = 0
  private signingOut: Promise<void> | undefined
  readonly links = { signup: '/', signin: '/login' }
  private generation = 0
  private lastCode: string | undefined
  private failedOperation: Operation | null = null
  private abort: AbortController | undefined

  private mode: 'signup' | 'signin' = 'signin'
  private confirmed = false
  private destination: string | undefined
  private resendDeadline = 0
  private otpExpiresAt = 0
  private unsubscribeUrl: (() => void) | undefined
  private authLocation = ''
  private resendTimer: ReturnType<typeof setInterval> | undefined
  private updateResendCountdown = () => {
    const remaining = Math.max(0, Math.ceil((this.resendDeadline - Date.now()) / 1000))
    if (remaining !== this.store.getState().remaining) this.store.setState({ remaining })
    if (remaining === 0) this.clearResendTimer()
  }
  private clearResendTimer() {
    if (this.resendTimer !== undefined) clearInterval(this.resendTimer)
    this.resendTimer = undefined
  }
  private setResendDeadline(deadline: number) {
    this.clearResendTimer()
    this.resendDeadline = deadline
    this.updateResendCountdown()
    if (this.store.getState().remaining > 0) this.resendTimer = setInterval(this.updateResendCountdown, 250)
  }

  private readonly url: Pick<UrlController, 'navigate' | 'store'>
  constructor(url: Pick<UrlController, 'navigate' | 'store'>) {
    this.url = url
  }
  navigate = (href: string) => this.url.navigate(href, { replace: href === '/login' })

  start = (mode: 'signup' | 'signin') => {
    this.stop()
    this.resendDeadline = 0
    this.mode = mode
    this.lastCode = undefined
    this.failedOperation = null
    this.confirmed = false
    this.destination = undefined
    const { currentUser, status } = this.store.getState()
    this.store.setState({ ...initialFormState, currentUser, status })
    this.authLocation = ''
    this.applyAuthUrl()
    this.unsubscribeUrl = this.url.store.subscribe(this.applyAuthUrl)
    return this.stop
  }
  stop = () => {
    this.unsubscribeUrl?.()
    this.unsubscribeUrl = undefined
    this.clearResendTimer()
    this.generation += 1
    this.abort?.abort()
    this.store.setState({ pending: null })
  }

  private applyAuthUrl = () => {
    const location = this.url.store.getState()
    const key = location.pathname + location.search
    if (key === this.authLocation) return
    this.authLocation = key
    const expected = this.mode === 'signup' ? '/signup/confirmation' : '/login/confirmation'
    if (location.pathname !== expected) {
      const emailEntry = this.mode === 'signup' ? ['/', '/signup'].includes(location.pathname) : location.pathname === '/login'
      if (emailEntry) {
        this.generation += 1
        this.abort?.abort()
        this.lastCode = undefined
        this.setResendDeadline(0)
        this.store.setState({ step: 'email', code: '', pending: null, error: null, serverError: null })
      }
      return
    }
    const query = new URLSearchParams(location.search)
    const deadline = (name: string) => {
      const value = Number(query.get(name))
      return Number.isSafeInteger(value) && value >= 0 ? value : 0
    }
    this.otpExpiresAt = deadline('expires_at')
    this.store.setState({
      step: 'code',
      email: query.get('email') ?? '',
      code: '',
      pending: null,
      error: this.otpExpiresAt > 0 && this.otpExpiresAt <= Date.now() ? 'expired-code' : null,
      serverError: null,
    })
    this.setResendDeadline(deadline('resend_at'))
  }
  private confirmationUrl() {
    const path = this.mode === 'signup' ? '/signup/confirmation' : '/login/confirmation'
    const query = new URLSearchParams({
      email: this.store.getState().email,
      resend_at: String(this.resendDeadline),
      expires_at: String(this.otpExpiresAt),
    })
    return path + '?' + query.toString()
  }

  onEmailChanged = (email: string) => {
    if (!this.store.getState().pending) this.store.setState({ email, error: null, serverError: null })
  }
  onInvalidEmail = () => this.store.setState({ error: 'invalid-email', serverError: null })
  showResend = () => this.store.setState({ resendRevealed: true })
  onCodeChanged = (code: string) => {
    const state = this.store.getState()
    if (state.pending || state.step !== 'code' || state.code === code) return
    this.store.setState({ code, error: null, serverError: null })
    void this.verify()
  }
  useDifferentEmail = () => {
    if (this.store.getState().pending) return
    this.lastCode = undefined
    this.confirmed = false
    this.failedOperation = null
    this.setResendDeadline(0)
    this.store.setState({ step: 'email', code: '', error: null, serverError: null })
    this.navigate(this.mode === 'signup' ? '/signup' : '/login')
  }
  requestCode = async () => {
    await this.request('request')
  }
  resendCode = async () => {
    await this.request('resend')
  }
  retry = async () => {
    if (!this.failedOperation) return
    if (this.failedOperation === 'verify') await this.verify(true)
    else if (this.failedOperation) await this.request(this.failedOperation)
  }

  signOut = async () => {
    if (this.signingOut) {
      await this.signingOut
      return
    }
    // Stop older identity checks from restoring the session during signout.
    this.sessionGeneration += 1
    this.sessionRequest?.abort()
    this.sessionPending = undefined
    this.stop()
    this.signingOut = this.performSignOut()
    await this.signingOut
  }

  private async performSignOut() {
    try {
      await signOut({ credentials: 'same-origin' })
      this.sessionGeneration += 1
      this.sessionRequest?.abort()
      this.sessionPending = undefined
      this.confirmed = false
      this.lastCode = undefined
      this.failedOperation = null
      this.destination = undefined
      this.store.setState({ ...initialFormState, currentUser: null, status: 'anonymous' })
      this.navigate?.('/login')
    } catch (error) {
      notify.error(error)
    } finally {
      this.signingOut = undefined
    }
  }

  ensureCurrentUser = async () => {
    const state = this.store.getState()
    if (state.status === 'authenticated' || state.status === 'anonymous') return state.currentUser
    return await (this.sessionPending ?? this.refreshCurrentUser(undefined, false))
  }

  refreshCurrentUser = async (signal?: AbortSignal, redirectAnonymous = true) => {
    this.sessionRequest?.abort()
    this.sessionRequest = new AbortController()
    const generation = ++this.sessionGeneration
    const requestSignal = signal ? AbortSignal.any([signal, this.sessionRequest.signal]) : this.sessionRequest.signal
    this.store.setState({ status: 'checking' })
    this.sessionPending = this.fetchCurrentUser(requestSignal, generation, redirectAnonymous)
    return await this.sessionPending
  }

  private async fetchCurrentUser(requestSignal: AbortSignal, generation: number, redirectAnonymous: boolean) {
    try {
      const user = await currentUser({ credentials: 'same-origin', signal: requestSignal })
      if (requestSignal.aborted || generation !== this.sessionGeneration) throw new DOMException('Cancelled', 'AbortError')
      this.store.setState({ currentUser: user ?? null, status: user ? 'authenticated' : 'anonymous' })
      if (!user && redirectAnonymous) this.navigate?.('/login')
      return user
    } catch (error) {
      if (requestSignal.aborted || generation !== this.sessionGeneration) {
        if (generation === this.sessionGeneration) this.store.setState({ currentUser: null, status: 'unknown' })
        throw new DOMException('Cancelled', 'AbortError')
      }
      if (isObjectLike(error) && get(error, 'status') === 401) {
        this.store.setState({ currentUser: null, status: 'anonymous' })
        if (redirectAnonymous) this.navigate?.('/login')
        return null
      }
      this.store.setState({ currentUser: null, status: 'error' })
      throw error
    } finally {
      if (generation === this.sessionGeneration) this.sessionPending = undefined
    }
  }

  private async request(operation: 'request' | 'resend') {
    const state = this.store.getState()
    this.updateResendCountdown()
    if (state.pending || this.store.getState().remaining > 0 || state.step !== (operation === 'request' ? 'email' : 'code'))
      return
    const email = state.email.trim().replace(/[A-Z]/g, (letter) => letter.toLowerCase())
    this.store.setState({ email })
    let deadline = 0
    let expiresAt = 0
    await this.perform(
      operation,
      async (signal) => {
        const request = this.mode === 'signup' ? signupEmail : signinEmail
        const response = await request({ email, delivery: 'cookie' }, { signal, credentials: 'same-origin' })
        const seconds = response.lifetime?.throttleSeconds
        deadline = typeof seconds === 'number' && Number.isFinite(seconds) && seconds >= 0 ? Date.now() + seconds * 1000 : 0
        const remaining = response.lifetime?.expiresInSeconds
        expiresAt =
          typeof remaining === 'number' && Number.isFinite(remaining) && remaining >= 0 ? Date.now() + remaining * 1000 : 0
      },
      () => {
        this.lastCode = undefined
        this.confirmed = false
        this.store.setState({ step: 'code', code: '', error: null, serverError: null })
        this.setResendDeadline(deadline)
        this.otpExpiresAt = expiresAt
        this.url.navigate(this.confirmationUrl(), { replace: operation === 'resend' })
      },
    )
  }

  private async verify(retry = false) {
    const state = this.store.getState()
    const schema = this.mode === 'signup' ? ConfirmSignupOtpRequest : ConfirmSigninOtpRequest
    const input = schema.safeParse({ code: state.code, delivery: 'cookie' })
    if (state.pending || state.step !== 'code' || !input.success) return
    if (!retry && this.lastCode === state.code) return
    this.lastCode = state.code
    await this.perform(
      'verify',
      async (signal) => {
        if (!this.confirmed) {
          const confirm = this.mode === 'signup' ? confirmSignupOtp : confirmSigninOtp
          await confirm(input.data, { signal, credentials: 'same-origin' })
          if (signal.aborted) throw new DOMException('Cancelled', 'AbortError')
          this.confirmed = true
        }
        const user = await this.refreshCurrentUser(signal)
        if (!user) throw new Error('Session could not be confirmed')
        this.destination = organizationHome(user)
      },
      () => {
        this.setResendDeadline(0)
        this.store.setState({ step: 'complete', code: '' })
        if (this.destination) this.navigate?.(this.destination)
      },
    )
  }

  private async perform(operation: Operation, request: (signal: AbortSignal) => Promise<void>, success: () => void) {
    const generation = ++this.generation
    this.abort = new AbortController()
    this.store.setState({ pending: operation, error: null, serverError: null })
    try {
      await request(this.abort.signal)
      if (generation !== this.generation) return
      this.failedOperation = null
      success()
    } catch (error) {
      if (generation !== this.generation) return
      if (isObjectLike(error) && get(error, 'name') === 'AbortError') return
      this.failedOperation = operation
      this.lastCode = undefined
      this.store.setState({ serverError: apiErrorMessage(error) })
    } finally {
      if (generation === this.generation) this.store.setState({ pending: null })
    }
  }
}

export const authController = new AuthController(urlController)
