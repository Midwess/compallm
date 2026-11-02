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
import { notify } from '../notifications/toast'
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

  navigate?: (href: string) => void

  start = (mode: 'signup' | 'signin') => {
    this.stop()
    this.mode = mode
    this.lastCode = undefined
    this.failedOperation = null
    this.confirmed = false
    this.destination = undefined
    const { currentUser, status } = this.store.getState()
    this.store.setState({ ...initialFormState, currentUser, status })
    return this.stop
  }
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
    this.confirmed = false
    this.failedOperation = null
    this.store.setState({ step: 'email', code: '', error: null })
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
        this.confirmed = false
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
        if (!this.confirmed) {
          const confirm = this.mode === 'signup' ? confirmSignupOtp : confirmSigninOtp
          await confirm({ code: state.code, delivery: 'cookie' }, { signal, credentials: 'same-origin' })
          if (signal.aborted) throw new DOMException('Cancelled', 'AbortError')
          this.confirmed = true
        }
        const user = await this.refreshCurrentUser(signal)
        if (!user) throw new Error('Session could not be confirmed')
        this.destination = organizationHome(user)
      },
      () => {
        this.store.setState({ step: 'complete', code: '' })
        if (this.destination) this.navigate?.(this.destination)
      },
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

export const authController = new AuthController()
