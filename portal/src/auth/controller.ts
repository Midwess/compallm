import { credential, ceremonySelection } from '../passkey/browser'
import { PasskeyFinishInput } from '@compallm/contract/validation'
import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'
import { createSignupStore, type SignupController, type SignupState } from '@compallm/storybook/pages/signup'
import type { SigninController } from '@compallm/storybook/pages/signin'
import get from 'lodash-es/get'
import isObjectLike from 'lodash-es/isObjectLike'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import type { AuthUser } from '@compallm/storybook'
import { graphql, ApiError } from '../graphql-client'
import { urlController } from '../url/controller'
import { appController } from '../app/controller'
import type { AppController, AppLoadingTask } from '@compallm/storybook'
import type { UrlController } from '@compallm/storybook'
import { notify, apiErrorMessage } from '../notifications/toast'
import { organizationRoot } from '../organization/paths'

type Operation = NonNullable<SignupState['pending']>

const authFailureReasons: Partial<Record<string, NonNullable<SignupState['error']>>> = {
  INCORRECT_CODE: 'incorrect-code',
  EXPIRED_TOKEN: 'expired-code',
  THROTTLED: 'throttled',
}

const initialFormState = createSignupStore().getInitialState()

export class AuthController implements SignupController, SigninController {
  readonly store = createSignupStore()
  private verificationResolve?: (verified: boolean) => void
  securityOfferSession: string | undefined
  private sessionPending: Promise<AuthUser | null> | undefined
  private sessionRequest: AbortController | undefined
  private sessionGeneration = 0
  private sessionLoading?: AppLoadingTask
  private readonly app?: Pick<AppController, 'beginLoading'>
  private signingOut: Promise<void> | undefined
  readonly links = { signup: '/', signin: '/login' }
  private generation = 0
  private verificationCode = ''
  private lastCode: string | undefined
  private failedOperation: Operation | null = null
  private abort: AbortController | undefined

  private mode: 'signup' | 'signin' = 'signin'
  private passkeyUncertain = false
  private expectedPasskey?: string
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
  private readonly retryFailedRoute?: () => Promise<void>
  constructor(
    url: Pick<UrlController, 'navigate' | 'store'>,
    app?: Pick<AppController, 'beginLoading'>,
    retryFailedRoute?: () => Promise<void>,
  ) {
    this.url = url
    this.app = app
    this.retryFailedRoute = retryFailedRoute
  }
  private rememberMethod(method: 'email' | 'passkey') {
    this.store.setState({ lastLoginMethod: method })
    try {
      localStorage.setItem('compallm.last-login-method', method)
    } catch {
      /* Storage can be unavailable. */
    }
  }
  private rememberedMethod() {
    try {
      const value = localStorage.getItem('compallm.last-login-method')
      return value === 'email' || value === 'passkey' ? value : this.store.getState().lastLoginMethod
    } catch {
      return this.store.getState().lastLoginMethod
    }
  }
  requestVerification = async (signal?: AbortSignal): Promise<boolean> => {
    if (!this.store.getState().currentUser || signal?.aborted) return false
    this.cancelVerification()
    this.stop()
    this.otpExpiresAt = 0
    this.store.setState({
      verificationActive: true,
      step: 'choose',
      email: '',
      code: '',
      error: null,
      serverError: null,
      pending: null,
      remaining: 0,
      lastLoginMethod: this.rememberedMethod(),
    })
    return new Promise<boolean>((resolve) => {
      const cancel = () => this.cancelVerification()
      this.verificationResolve = (verified) => {
        signal?.removeEventListener('abort', cancel)
        resolve(verified)
      }
      signal?.addEventListener('abort', cancel, { once: true })
    })
  }
  cancelVerification = () => this.finishVerification(false)
  private finishVerification(verified: boolean) {
    const resolve = this.verificationResolve
    this.verificationResolve = undefined
    if (!this.store.getState().verificationActive && !resolve) return
    this.abort?.abort()
    this.generation += 1
    this.setResendDeadline(0)
    this.lastCode = undefined
    this.verificationCode = ''
    this.store.setState({
      verificationActive: false,
      step: verified ? 'complete' : 'choose',
      code: '',
      email: '',
      pending: null,
      error: null,
      serverError: null,
    })
    resolve?.(verified)
  }
  navigate = (href: string) => this.url.navigate(href, { replace: href === '/login' })

  start = (mode: 'signup' | 'signin') => {
    this.stop()
    this.resendDeadline = 0
    this.mode = mode
    this.lastCode = undefined
    this.failedOperation = null
    this.confirmed = false
    this.passkeyUncertain = false
    this.destination = undefined
    const { currentUser, status } = this.store.getState()
    this.store.setState({
      ...initialFormState,
      currentUser,
      status,
      lastLoginMethod: this.rememberedMethod(),
      step: mode === 'signin' ? 'choose' : 'email',
    })
    this.authLocation = ''
    this.applyAuthUrl()
    this.unsubscribeUrl = this.url.store.subscribe(this.applyAuthUrl)
    return this.stop
  }
  stop = () => {
    this.cancelVerification()
    this.unsubscribeUrl?.()
    this.unsubscribeUrl = undefined
    this.clearResendTimer()
    this.generation += 1
    this.abort?.abort()
    this.verificationCode = ''
    this.store.setState({ pending: null, code: '' })
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
        this.store.setState({
          step: this.mode === 'signin' ? 'choose' : 'email',
          code: '',
          pending: null,
          error: null,
          serverError: null,
        })
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

  clearError = () => this.store.setState({ error: null, serverError: null })
  onInvalidEmail = () => this.store.setState({ error: 'invalid-email', serverError: null })
  showResend = () => this.store.setState({ resendRevealed: true })
  verifyCode = async (code: string) => {
    this.verificationCode = code
    await this.verify(false, code)
  }
  useDifferentEmail = () => {
    if (this.store.getState().pending === 'passkey') {
      this.abort?.abort()
      this.generation += 1
      this.store.setState({ pending: null })
    }
    if (this.store.getState().pending) return
    this.passkeyUncertain = false
    this.lastCode = undefined
    this.confirmed = false
    this.failedOperation = null
    this.setResendDeadline(0)
    this.store.setState({ step: 'email', code: '', error: null, serverError: null })
    this.navigate(this.mode === 'signup' ? '/signup' : '/login')
  }
  requestCode = async (email: string) => {
    await this.request('request', email)
  }
  useEmailCode = async () => {
    if (this.store.getState().verificationActive && this.store.getState().email && this.otpExpiresAt > Date.now()) {
      this.store.setState({ step: 'code', error: null, serverError: null })
      return
    }
    this.abort?.abort()
    this.generation += 1
    this.confirmed = false
    this.passkeyUncertain = false
    this.expectedPasskey = undefined
    this.store.setState({ step: 'email', pending: null, error: null, serverError: null })
  }
  backToMethods = () => {
    this.abort?.abort()
    this.generation += 1
    this.confirmed = false
    this.passkeyUncertain = false
    this.expectedPasskey = undefined
    if (!this.store.getState().verificationActive) {
      this.setResendDeadline(0)
      this.lastCode = undefined
      this.navigate(this.mode === 'signup' ? '/signup' : '/login')
    }
    this.store.setState({ step: 'choose', pending: null, error: null, serverError: null })
  }
  verifyPasskey = async () => {
    if (this.store.getState().pending || !['choose', 'passkey'].includes(this.store.getState().step)) return
    this.store.setState({ step: 'passkey' })
    await this.perform(
      'passkey',
      async (signal) => {
        if (this.store.getState().verificationActive) {
          const {
            passkeyAuthenticationStart: { node: ceremony },
          } = await graphql(signal).mutation({
            passkeyAuthenticationStart: { __args: { purpose: 'PASSKEY_CHANGE' }, node: ceremonySelection },
          } satisfies MutationGenqlSelection)
          const input = PasskeyFinishInput.parse({
            ...(await credential(ceremony.options, ceremony.token, false, signal)),
            purpose: 'PASSKEY_CHANGE',
          })
          await graphql(signal).mutation({
            passkeyAuthenticationFinish: { __args: { input }, node: { purpose: true } },
          } satisfies MutationGenqlSelection)
          return
        }
        if (this.passkeyUncertain) {
          const restored = await this.refreshCurrentUser(signal, false)
          if (restored) {
            const result = await graphql(signal).query({
              session: { node: { passkey: { credentialId: true } } },
            } satisfies QueryGenqlSelection)
            if (result.session.node.passkey?.credentialId === this.expectedPasskey) this.confirmed = true
          }
          this.passkeyUncertain = false
          if (!this.confirmed)
            throw new Error('Sign-in could not be confirmed. Try your passkey again or use email code instead.')
        }
        if (!this.confirmed) {
          const {
            passkeyAuthenticationStart: { node: ceremony },
          } = await graphql(signal).mutation({
            passkeyAuthenticationStart: { node: ceremonySelection },
          } satisfies MutationGenqlSelection)
          const input = PasskeyFinishInput.parse(await credential(ceremony.options, ceremony.token, false, signal))
          this.expectedPasskey = input.rawId
          this.passkeyUncertain = true
          await graphql(signal).mutation({
            passkeyAuthenticationFinish: { __args: { input }, node: { accessLifetime: { expiresInSeconds: true } } },
          } satisfies MutationGenqlSelection)
          if (signal.aborted) return
          this.confirmed = true
          this.passkeyUncertain = false
        }
        const user = await this.refreshCurrentUser(signal, false)
        if (!user) throw new Error('Session could not be confirmed')
        this.destination = organizationRoot(user)
      },
      () => {
        this.rememberMethod('passkey')
        if (this.store.getState().verificationActive) {
          this.finishVerification(true)
          return
        }
        this.store.setState({ step: 'complete' })
        if (this.destination) this.navigate(this.destination)
      },
    )
  }
  resendCode = async () => {
    await this.request('resend')
  }
  retry = async () => {
    if (!this.failedOperation) return
    if (this.failedOperation === 'passkey') {
      await this.verifyPasskey()
      return
    }
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
    this.sessionLoading?.cancel()
    this.sessionPending = undefined
    this.stop()
    this.signingOut = this.performSignOut()
    await this.signingOut
  }

  forgetCurrentUser = () => {
    this.sessionGeneration += 1
    this.sessionRequest?.abort()
    this.sessionLoading?.cancel()
    this.sessionPending = undefined
    this.stop()
    this.store.setState({ currentUser: null, status: 'anonymous' })
  }

  invalidAuthorization = () => {
    this.sessionGeneration += 1
    this.sessionRequest?.abort()
    this.sessionLoading?.cancel()
    this.sessionPending = undefined
    this.stop()
    this.confirmed = false
    this.lastCode = undefined
    this.failedOperation = null
    this.destination = undefined
    this.store.setState({ ...initialFormState, currentUser: null, status: 'anonymous' })
    this.navigate('/login')
  }

  private async performSignOut() {
    try {
      const selection = { sessionRevoke: { node: true } } satisfies MutationGenqlSelection
      const result = await graphql().mutation(selection)
      if (!result.sessionRevoke.node) throw new Error('Session could not be revoked')
      this.sessionGeneration += 1
      this.sessionRequest?.abort()
      this.sessionLoading?.cancel()
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
    this.sessionLoading?.cancel()
    this.sessionRequest = new AbortController()
    const generation = ++this.sessionGeneration
    const requestSignal = signal ? AbortSignal.any([signal, this.sessionRequest.signal]) : this.sessionRequest.signal
    this.store.setState({ status: 'checking' })
    const loading = this.app?.beginLoading('session')
    this.sessionLoading = loading
    this.sessionPending = this.fetchCurrentUser(requestSignal, generation, redirectAnonymous)
    try {
      const user = await this.sessionPending
      loading?.complete()
      return user
    } catch (error) {
      if (requestSignal.aborted || generation !== this.sessionGeneration) loading?.cancel()
      else
        loading?.fail(error, async () => {
          await this.refreshCurrentUser(undefined, false)
          await this.retryFailedRoute?.()
        })
      throw error
    }
  }

  private async fetchCurrentUser(requestSignal: AbortSignal, generation: number, redirectAnonymous: boolean) {
    try {
      const selection = {
        session: {
          node: {
            id: true,
            userId: true,
            organizationId: true,
            organizationName: true,
            avatarUrl: true,
            location: true,
            ipAddress: true,
            userAgent: true,
          },
        },
      } satisfies QueryGenqlSelection
      const {
        session: { node: user },
      } = await graphql(requestSignal).query(selection)
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

  private async request(operation: 'request' | 'resend', emailValue = this.store.getState().email) {
    const state = this.store.getState()
    this.updateResendCountdown()
    if (state.pending || this.store.getState().remaining > 0 || state.step !== (operation === 'request' ? 'email' : 'code'))
      return
    const verification = !!state.verificationActive
    if (verification && !emailValue) {
      await this.perform(
        operation,
        async (signal) => {
          const user = this.store.getState().currentUser
          if (!user) throw new Error('Session is unavailable')
          const result = await graphql(signal).query({
            userProfile: { __args: { id: user.userId }, node: { email: true } },
          } satisfies QueryGenqlSelection)
          emailValue = result.userProfile?.node.email ?? ''
        },
        () => {},
      )
      if (!this.store.getState().verificationActive || !emailValue) return
    }
    const input = emailRequestSchema(verification ? 'signin' : this.mode).safeParse({
      email: emailValue,
      delivery: 'COOKIE',
      purpose: verification ? 'PASSKEY_CHANGE' : 'LOGIN',
    })
    if (!input.success) {
      this.onInvalidEmail()
      return
    }
    const email = input.data.email
    this.store.setState({ email })
    let deadline = 0
    let expiresAt = 0
    await this.perform(
      operation,
      async (signal) => {
        const lifetime = { expiresInSeconds: true, throttleSeconds: true } as const
        const signup = {
          emailSignupRequest: { __args: { input: input.data }, node: { lifetime } },
        } satisfies MutationGenqlSelection
        const signin = {
          emailSigninRequest: { __args: { input: input.data }, node: { lifetime } },
        } satisfies MutationGenqlSelection
        const client = graphql(signal)
        const response =
          !verification && this.mode === 'signup'
            ? (await client.mutation(signup)).emailSignupRequest.node
            : (await client.mutation(signin)).emailSigninRequest.node
        if (!response) throw new Error('Email challenge was not issued')
        const seconds = Number(response.lifetime.throttleSeconds)
        deadline = typeof seconds === 'number' && Number.isFinite(seconds) && seconds >= 0 ? Date.now() + seconds * 1000 : 0
        const remaining = Number(response.lifetime.expiresInSeconds)
        expiresAt =
          typeof remaining === 'number' && Number.isFinite(remaining) && remaining >= 0 ? Date.now() + remaining * 1000 : 0
      },
      () => {
        this.lastCode = undefined
        this.confirmed = false
        this.store.setState({ step: 'code', code: '', error: null, serverError: null })
        this.setResendDeadline(deadline)
        this.otpExpiresAt = expiresAt
        if (!verification) this.url.navigate(this.confirmationUrl(), { replace: operation === 'resend' })
      },
    )
  }

  private async verify(retry = false, code = this.verificationCode) {
    const state = this.store.getState()
    const verification = !!state.verificationActive
    const schema = otpRequestSchema(verification ? 'signin' : this.mode)
    const input = schema.safeParse({ code, delivery: 'COOKIE', purpose: verification ? 'PASSKEY_CHANGE' : 'LOGIN' })
    if (state.pending || state.step !== 'code' || !input.success) return
    if (!retry && this.lastCode === code) return
    this.lastCode = code
    await this.perform(
      'verify',
      async (signal) => {
        if (verification) {
          await graphql(signal).mutation({
            emailSigninConfirm: { __args: { input: input.data }, node: { purpose: true } },
          } satisfies MutationGenqlSelection)
          return
        }
        if (!this.confirmed) {
          const lifetime = { expiresInSeconds: true } as const
          const signup = {
            emailSignupConfirm: { __args: { input: input.data }, node: { accessLifetime: lifetime } },
          } satisfies MutationGenqlSelection
          const signin = {
            emailSigninConfirm: { __args: { input: input.data }, node: { accessLifetime: lifetime } },
          } satisfies MutationGenqlSelection
          const client = graphql(signal)
          const result =
            this.mode === 'signup'
              ? (await client.mutation(signup)).emailSignupConfirm.node
              : (await client.mutation(signin)).emailSigninConfirm.node
          if (!result) throw new Error('Session could not be confirmed')
          if (signal.aborted) throw new DOMException('Cancelled', 'AbortError')
          this.confirmed = true
        }
        const user = await this.refreshCurrentUser(signal)
        if (!user) throw new Error('Session could not be confirmed')
        if (this.mode === 'signup') {
          this.securityOfferSession = user.id
          this.destination = '/security'
        } else this.destination = organizationRoot(user)
      },
      () => {
        this.rememberMethod('email')
        if (verification) {
          this.finishVerification(true)
          return
        }
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
      if (isObjectLike(error) && get(error, 'name') === 'AbortError' && this.abort?.signal.aborted) return
      if (operation === 'passkey' && error instanceof DOMException && ['AbortError', 'NotAllowedError'].includes(error.name)) {
        this.store.setState({
          serverError: 'Passkey verification was cancelled or timed out. Try again or use email code instead.',
        })
        return
      }
      if (operation === 'passkey') {
        if (error instanceof ApiError && error.status < 500) this.passkeyUncertain = false
        this.failedOperation = operation
        this.store.setState({ error: null, serverError: apiErrorMessage(error) })
        return
      }
      this.failedOperation = operation
      this.lastCode = undefined
      const reason = error instanceof ApiError ? authFailureReasons[error.code] : undefined
      this.store.setState(reason ? { error: reason } : { serverError: apiErrorMessage(error) })
    } finally {
      if (generation === this.generation) this.store.setState({ pending: null })
    }
  }
}

export const authController = new AuthController(urlController, appController, urlController.retryFailedRoute)
