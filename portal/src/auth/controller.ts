import {
  AuthSession2Store,
  AuthEmailSigninConfirmStore,
  AuthPasskeyAuthenticationStartStore,
  AuthPasskeyAuthenticationFinishStore,
  AuthSessionStore,
  AuthUserProfileStore,
  AuthSessionRevokeStore,
  AuthEmailSigninRequestStore,
  AuthEmailSignupRequestStore,
  AuthEmailSigninConfirm2Store,
  AuthEmailSignupConfirmStore,
  AuthPasskeyAuthenticationStart2Store,
  AuthPasskeyAuthenticationFinish2Store,
} from '@compallm/contract/houdini'
import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'
import { PasskeyFinishInput } from '@compallm/contract/validation'
import { createAuthStore, createAuthForm, authFormOptions } from '@compallm/storybook/pages/auth/model'
import type { AuthController as AuthContract, AuthMode, AuthOperation, AuthUser } from '@compallm/storybook/pages/auth'
import { createStore } from '@compallm/storybook/state'
import { ApiError, GraphqlClient, errorMessage, requireData } from '../graphql-client'
import { UrlController } from '../url/controller'
import { credential, PasskeyError } from './passkey'

export const organizationRoot = (user: AuthUser) => `/${encodeURIComponent(user.organizationName)}`
const cancelled = () => new DOMException('Cancelled', 'AbortError')
export const rejectedConfirmation = (error: unknown) =>
  error instanceof ApiError &&
  error.status < 500 &&
  [
    'INCORRECT_CODE',
    'EXPIRED_TOKEN',
    'INVALID_TOKEN',
    'UNAUTHENTICATED',
    'BAD_USER_INPUT',
    'THROTTLED',
    'PASSKEY_INVALID',
  ].includes(error.code)
function deadline(seconds: string) {
  const value = Number(seconds)
  const result = Date.now() + value * 1000
  if (!Number.isSafeInteger(value) || value < 0 || !Number.isSafeInteger(result))
    throw new ApiError('Request failed. Please try again.', 502, 'INVALID_RESPONSE')
  return result
}
export class AuthController implements AuthContract {
  private verificationResolve?: (verified: boolean) => void
  securityOfferSession: string | undefined
  readonly store = createAuthStore()
  readonly form = createAuthForm(this, 'signin')
  readonly lifecycle = createStore(() => ({ signingOut: false, error: null as string | null }))
  readonly links = { signup: '/signup', signin: '/login' }
  private mode: AuthMode = 'signin'
  private generation = 0
  private operation?: AbortController
  private sessionGeneration = 0
  private sessionRequest?: AbortController
  private sessionPending?: Promise<AuthUser | null>
  private signingOut?: Promise<void>
  private confirmed = false
  private uncertain: 'email' | 'passkey' | null = null
  private expectedPasskey?: string
  private failed?: AuthOperation
  private verificationCode = ''
  private destination?: string
  private expiresAt = 0
  private resendAt = 0
  private countdown?: ReturnType<typeof setInterval>
  private scopeKey?: string
  private stopUrl?: () => void
  private releaseForm?: () => void
  constructor(
    private readonly url: Pick<UrlController, 'navigate' | 'store'>,
    private readonly client: GraphqlClient,
  ) {}
  seed(user: AuthUser | null) {
    this.scopeKey = user ? `${user.userId}:${user.organizationId}:${user.id}` : undefined
    this.store.setState({ currentUser: user, status: user ? 'authenticated' : 'anonymous' })
  }
  requestVerification = async (signal?: AbortSignal): Promise<boolean> => {
    if (!this.store.getState().currentUser || signal?.aborted) return false
    this.cancelVerification()
    this.stop()
    this.mode = 'signin'
    this.confirmed = false
    this.uncertain = null
    this.expiresAt = 0
    this.setResendDeadline(0)
    this.form.configure(authFormOptions(this, 'signin'))
    this.form.reset({ email: '', code: '' })
    this.releaseForm = this.form.mount()
    this.store.setState({ verificationActive: true, step: 'choose', email: '', error: null, serverError: null, pending: null })
    return new Promise((resolve) => {
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
    if (!resolve && !this.store.getState().verificationActive) return
    this.operation?.abort()
    this.generation++
    this.setResendDeadline(0)
    this.verificationCode = ''
    this.form.reset({ email: '', code: '' })
    this.store.setState({
      verificationActive: false,
      step: verified ? 'complete' : 'choose',
      email: '',
      pending: null,
      error: null,
      serverError: null,
    })
    resolve?.(verified)
  }
  forgetCurrentUser = () => {
    this.cancelVerification()
    this.stop()
    this.sessionGeneration++
    this.sessionRequest?.abort()
    this.sessionPending = undefined
    this.scopeKey = undefined
    this.client.resetScope()
    this.store.setState({ currentUser: null, status: 'unknown' })
  }
  navigate = (href: string) => {
    void this.url.navigate(href).catch((error) => this.lifecycle.setState({ error: errorMessage(error) }))
  }
  clearError = () => this.store.setState({ error: null, serverError: null })
  start = (mode: AuthMode) => {
    this.stop()
    this.mode = mode
    this.confirmed = false
    this.uncertain = null
    this.failed = undefined
    this.resendAt = 0
    this.expiresAt = 0
    this.location = ''
    this.destination = new URLSearchParams(this.url.store.getState().search).get('return_to') ?? undefined
    this.form.configure(authFormOptions(this, mode))
    this.form.reset({ email: '', code: '' })
    let lastLoginMethod = this.store.getState().lastLoginMethod
    try {
      const value = localStorage.getItem('compallm.last-login-method')
      if (value === 'email' || value === 'passkey') lastLoginMethod = value
    } catch {
      /* Optional presentation preference. */
    }
    this.store.setState({
      step: mode === 'signup' ? 'email' : 'choose',
      email: '',
      pending: null,
      error: null,
      serverError: null,
      remaining: 0,
      lastLoginMethod,
    })
    this.releaseForm = this.form.mount()
    this.applyLocation()
    this.stopUrl = this.url.store.subscribe(this.applyLocation)
    return this.stop
  }
  stop = () => {
    this.cancelVerification()
    this.stopUrl?.()
    this.stopUrl = undefined
    this.generation++
    this.operation?.abort()
    clearInterval(this.countdown)
    this.countdown = undefined
    this.releaseForm?.()
    this.releaseForm = undefined
    this.form.stop()
    this.form.set('code', '')
    this.verificationCode = ''
    this.store.setState({ pending: null })
  }
  dispose = () => {
    this.stop()
    this.sessionGeneration++
    this.sessionRequest?.abort()
    this.client.resetScope()
  }
  private location = ''
  private applyLocation = () => {
    const location = this.url.store.getState()
    const expected = this.mode === 'signup' ? '/signup/confirmation' : '/login/confirmation'
    const key = location.pathname + location.search
    if (key === this.location) return
    this.location = key
    if (location.pathname !== expected) {
      const entry = this.mode === 'signup' ? ['/', '/signup'] : ['/login', '/signin']
      if (entry.includes(location.pathname)) {
        this.generation++
        this.operation?.abort()
        this.form.stop()
        this.form.set('code', '')
        this.setResendDeadline(0)
        this.confirmed = false
        this.uncertain = null
        this.expectedPasskey = undefined
        this.failed = undefined
        this.store.setState({
          step: this.mode === 'signup' ? 'email' : 'choose',
          pending: null,
          error: null,
          serverError: null,
        })
      }
      return
    }
    const query = new URLSearchParams(location.search)
    const parsed = emailRequestSchema(this.mode).safeParse({ email: query.get('email') ?? '' })
    if (!parsed.success) {
      this.store.setState({ step: 'email' })
      return
    }
    const time = (key: string) => {
      const value = Number(query.get(key))
      return Number.isSafeInteger(value) && value >= 0 ? value : 0
    }
    const expiresAt = time('expires_at'),
      resendAt = time('resend_at')
    if (this.store.getState().email !== parsed.data.email || this.expiresAt !== expiresAt || this.resendAt !== resendAt) {
      this.generation++
      this.operation?.abort()
      this.form.stop()
      this.form.set('code', '')
      this.confirmed = false
      this.uncertain = null
      this.store.setState({ pending: null, serverError: null })
    }
    this.expiresAt = expiresAt
    this.setResendDeadline(resendAt)
    this.store.setState({
      step: 'code',
      email: parsed.data.email,
      error: this.expiresAt && this.expiresAt <= Date.now() ? 'expired-code' : null,
    })
  }
  private setResendDeadline(value: number) {
    clearInterval(this.countdown)
    this.resendAt = value
    const update = () => {
      const remaining = Math.max(0, Math.ceil((this.resendAt - Date.now()) / 1000))
      if (remaining !== this.store.getState().remaining) this.store.setState({ remaining })
      if (!remaining) {
        clearInterval(this.countdown)
        this.countdown = undefined
      }
    }
    update()
    if (typeof window !== 'undefined' && this.store.getState().remaining) this.countdown = setInterval(update, 250)
  }
  private remember(method: 'email' | 'passkey') {
    this.store.setState({ lastLoginMethod: method })
    try {
      localStorage.setItem('compallm.last-login-method', method)
    } catch {
      /* Optional presentation preference. */
    }
  }
  useEmailCode = async () => {
    this.generation++
    this.operation?.abort()
    this.confirmed = false
    this.uncertain = null
    this.expectedPasskey = undefined
    this.failed = undefined
    this.store.setState({ step: 'email', pending: null, error: null, serverError: null })
  }
  backToMethods = () => {
    if (this.store.getState().verificationActive) {
      this.generation++
      this.operation?.abort()
      this.uncertain = null
      this.store.setState({ step: 'choose', pending: null, error: null, serverError: null })
      return
    }

    this.generation++
    this.operation?.abort()
    this.confirmed = false
    this.uncertain = null
    this.expectedPasskey = undefined
    this.failed = undefined
    this.store.setState({ step: 'choose', pending: null, error: null, serverError: null })
    void this.url
      .navigate(this.mode === 'signup' ? '/signup' : '/login')
      .catch((error) => this.lifecycle.setState({ error: errorMessage(error) }))
  }
  useDifferentEmail = () => {
    if (this.store.getState().verificationActive) return

    if (this.store.getState().pending) return
    this.confirmed = false
    this.uncertain = null
    this.failed = undefined
    this.setResendDeadline(0)
    this.form.reset({ email: this.store.getState().email, code: '' })
    this.store.setState({ step: 'email', error: null, serverError: null })
    void this.url
      .navigate(this.mode === 'signup' ? '/signup' : '/login')
      .catch((error) => this.lifecycle.setState({ error: errorMessage(error) }))
  }
  requestCode = (email: string) => this.sendCode('request', email)
  resendCode = () => this.sendCode('resend', this.store.getState().email)
  private sendCode = async (operation: 'request' | 'resend', email: string) => {
    const state = this.store.getState()
    if (state.pending || this.resendAt > Date.now() || state.step !== (operation === 'request' ? 'email' : 'code')) return
    const verification = state.verificationActive
    await this.perform(operation, async (signal) => {
      if (verification && !email) {
        const user = state.currentUser
        if (!user) return
        const profile = requireData(
          await new AuthUserProfileStore().fetch({
            variables: { userProfileId: user.userId },
            policy: 'NoCache',
            fetch: this.client.fetch(signal),
            blocking: true,
          }),
        )
        signal.throwIfAborted()
        if (this.store.getState().currentUser?.id !== user.id) return
        email = profile.userProfile?.node.email ?? ''
      }
      const parsed = emailRequestSchema(this.mode).safeParse({
        email,
        delivery: 'COOKIE',
        purpose: verification ? 'PASSKEY_CHANGE' : 'LOGIN',
        invitation: false,
      })
      if (!parsed.success) return
      const options = { fetch: this.client.fetch(signal) }
      const lifetime =
        this.mode === 'signup'
          ? requireData(await new AuthEmailSignupRequestStore().mutate({ emailSignupRequestInput: parsed.data }, options))
              .emailSignupRequest.node.lifetime
          : requireData(await new AuthEmailSigninRequestStore().mutate({ emailSigninRequestInput: parsed.data }, options))
              .emailSigninRequest.node.lifetime
      signal.throwIfAborted()
      if (!verification) this.client.resetScope()
      this.confirmed = false
      this.uncertain = null
      this.form.set('code', '')
      this.expiresAt = deadline(lifetime.expiresInSeconds)
      this.setResendDeadline(deadline(lifetime.throttleSeconds))
      this.store.setState({ step: 'code', email: parsed.data.email })
      if (verification) return
      const path = this.mode === 'signup' ? '/signup/confirmation' : '/login/confirmation'
      const query = new URLSearchParams({
        email: parsed.data.email,
        resend_at: String(this.resendAt),
        expires_at: String(this.expiresAt),
      })
      if (this.destination) query.set('return_to', this.destination)
      await this.url.navigate(`${path}?${query}`, { replace: operation === 'resend' })
    })
  }
  verifyCode = async (code: string) => {
    if (this.store.getState().pending || this.store.getState().step !== 'code') return
    const parsed = otpRequestSchema(this.mode).safeParse({
      code,
      delivery: 'COOKIE',
      purpose: this.store.getState().verificationActive ? 'PASSKEY_CHANGE' : 'LOGIN',
    })
    if (!parsed.success) return
    this.verificationCode = parsed.data.code
    await this.perform('verify', async (signal) => {
      if (this.store.getState().verificationActive) {
        if (this.uncertain)
          throw new ApiError(
            'Verification could not be confirmed. Request a new code before trying again.',
            503,
            'VERIFICATION_UNAVAILABLE',
          )
        this.uncertain = 'email'
        try {
          requireData(
            await new AuthEmailSigninConfirmStore().mutate(
              { emailSigninConfirmInput: parsed.data },
              { fetch: this.client.fetch(signal) },
            ),
          )
          this.uncertain = null
        } catch (error) {
          if (rejectedConfirmation(error)) this.uncertain = null
          throw error
        }
        signal.throwIfAborted()
        this.finishVerification(true)
        return
      }
      if (!this.confirmed && !this.uncertain) {
        this.uncertain = 'email'
        try {
          const options = { fetch: this.client.fetch(signal) }
          if (this.mode === 'signup')
            requireData(await new AuthEmailSignupConfirmStore().mutate({ emailSignupConfirmInput: parsed.data }, options))
          else requireData(await new AuthEmailSigninConfirm2Store().mutate({ emailSigninConfirmInput: parsed.data }, options))
          this.confirmed = true
          this.uncertain = null
          this.client.resetScope()
        } catch (error) {
          if (rejectedConfirmation(error)) this.uncertain = null
          throw error
        }
      }
      await this.finishLogin(signal, 'email')
    })
  }
  verifyPasskey = async () => {
    if (this.store.getState().pending || !['choose', 'passkey'].includes(this.store.getState().step)) return
    this.store.setState({ step: 'passkey' })
    await this.perform('passkey', async (signal) => {
      if (this.store.getState().verificationActive) {
        if (this.uncertain)
          throw new ApiError(
            'Verification could not be confirmed. Start a new verification before trying again.',
            503,
            'VERIFICATION_UNAVAILABLE',
          )
        const ceremony = requireData(
          await new AuthPasskeyAuthenticationStartStore().mutate(undefined, { fetch: this.client.fetch(signal) }),
        ).passkeyAuthenticationStart.node
        const input = PasskeyFinishInput.parse({
          ...(await credential(ceremony.options, ceremony.token, signal)),
          purpose: 'PASSKEY_CHANGE',
        })
        this.uncertain = 'passkey'
        try {
          requireData(
            await new AuthPasskeyAuthenticationFinishStore().mutate(
              { passkeyAuthenticationFinishInput: input },
              { fetch: this.client.fetch(signal) },
            ),
          )
          this.uncertain = null
        } catch (error) {
          if (rejectedConfirmation(error)) this.uncertain = null
          throw error
        }
        signal.throwIfAborted()
        this.finishVerification(true)
        return
      }
      if (!this.confirmed && !this.uncertain) {
        const ceremony = requireData(
          await new AuthPasskeyAuthenticationStart2Store().mutate(undefined, { fetch: this.client.fetch(signal) }),
        ).passkeyAuthenticationStart.node
        const input = PasskeyFinishInput.parse(await credential(ceremony.options, ceremony.token, signal))
        this.expectedPasskey = input.rawId
        this.uncertain = 'passkey'
        try {
          requireData(
            await new AuthPasskeyAuthenticationFinish2Store().mutate(
              { passkeyAuthenticationFinishInput: input },
              { fetch: this.client.fetch(signal) },
            ),
          )
          this.confirmed = true
          this.uncertain = null
          this.client.resetScope()
        } catch (error) {
          if (rejectedConfirmation(error)) this.uncertain = null
          throw error
        }
      }
      await this.finishLogin(signal, 'passkey')
    })
  }
  private async finishLogin(signal: AbortSignal, method: 'email' | 'passkey') {
    const user = await this.refreshCurrentUser(signal, false)
    if (!user)
      throw new ApiError(
        this.uncertain
          ? 'Verification could not be confirmed. Request a new code or try your passkey again.'
          : 'Session could not be confirmed. Please try again.',
        503,
        'SESSION_UNAVAILABLE',
      )
    if (this.uncertain === 'email') {
      const profile = requireData(
        await new AuthUserProfileStore().fetch({
          variables: { userProfileId: user.userId },
          policy: 'NoCache',
          fetch: this.client.fetch(signal),
          blocking: true,
        }),
      )
      if (profile.userProfile?.node.email !== this.store.getState().email)
        throw new ApiError('Email sign-in could not be confirmed. Request a new code.', 503, 'SESSION_UNAVAILABLE')
    }
    if (this.uncertain === 'passkey') {
      const current = requireData(
        await new AuthSessionStore().fetch({ policy: 'NoCache', fetch: this.client.fetch(signal), blocking: true }),
      )
      if (current.session.node.passkey?.credentialId !== this.expectedPasskey)
        throw new ApiError('Passkey sign-in could not be confirmed. Please try again.', 503, 'SESSION_UNAVAILABLE')
    }
    signal.throwIfAborted()
    this.acceptIdentity(user)
    this.confirmed = true
    this.uncertain = null
    this.remember(method)
    this.setResendDeadline(0)
    this.store.setState({ step: 'complete' })
    this.form.set('code', '')
    const root = organizationRoot(user)
    const target = this.destination?.startsWith(root + '/') || this.destination === root ? this.destination : root
    if (this.mode === 'signup') {
      this.securityOfferSession = user.id
      await this.url.navigate('/security', { replace: true })
    } else await this.url.navigate(target, { replace: true })
  }
  retry = async () => {
    if (this.failed === 'verify') await this.verifyCode(this.verificationCode || String(this.form.get('code') ?? ''))
    else if (this.failed === 'passkey') await this.verifyPasskey()
    else if (this.failed === 'resend') await this.resendCode()
    else if (this.failed === 'request') await this.requestCode(String(this.form.get('email') ?? ''))
  }
  private async perform(operation: AuthOperation, work: (signal: AbortSignal) => Promise<void>) {
    const generation = ++this.generation
    const request = new AbortController()
    this.operation = request
    this.store.setState({ pending: operation, error: null, serverError: null })
    try {
      await work(request.signal)
      if (generation === this.generation && !request.signal.aborted) this.failed = undefined
    } catch (error) {
      if (generation !== this.generation || request.signal.aborted) return
      this.failed = operation
      if (this.store.getState().status === 'checking') this.store.setState({ status: 'error' })
      const reason =
        error instanceof ApiError
          ? ({ INCORRECT_CODE: 'incorrect-code', EXPIRED_TOKEN: 'expired-code', THROTTLED: 'throttled' } as const)[
              error.code as 'INCORRECT_CODE' | 'EXPIRED_TOKEN' | 'THROTTLED'
            ]
          : undefined
      const message =
        operation === 'passkey' && error instanceof DOMException && ['NotAllowedError', 'AbortError'].includes(error.name)
          ? 'Passkey verification was cancelled or timed out. Try again or use email instead.'
          : error instanceof PasskeyError
            ? error.message
            : errorMessage(error)
      this.store.setState(reason ? { error: reason } : { serverError: message })
    } finally {
      if (generation === this.generation) this.store.setState({ pending: null })
    }
  }
  ensureCurrentUser = () => {
    const state = this.store.getState()
    if (state.status === 'authenticated' || state.status === 'anonymous') return Promise.resolve(state.currentUser)
    return this.sessionPending ?? this.refreshCurrentUser()
  }
  ensureGuestAccess = () => {
    const state = this.store.getState()
    if (state.step === 'code' || state.step === 'passkey') return Promise.resolve(state.currentUser)
    return this.ensureCurrentUser()
  }
  refreshCurrentUser = (
    signal?: AbortSignal,
    publish = true,
    expected?: { email: string; organizationName: string },
  ): Promise<AuthUser | null> => {
    if (this.sessionPending) return this.sessionPending
    const request = new AbortController()
    this.sessionRequest = request
    const combined = signal ? AbortSignal.any([signal, request.signal]) : request.signal
    const generation = ++this.sessionGeneration
    this.store.setState({ status: 'checking' })
    const promise = (async () => {
      try {
        const data = requireData(
          await new AuthSession2Store().fetch({ policy: 'NoCache', fetch: this.client.fetch(combined), blocking: true }),
        )
        combined.throwIfAborted()
        if (generation !== this.sessionGeneration) throw cancelled()
        const user = data.session.node
        if (expected) {
          const profile = requireData(
            await new AuthUserProfileStore().fetch({
              variables: { userProfileId: user.userId },
              policy: 'NoCache',
              fetch: this.client.fetch(combined),
              blocking: true,
            }),
          )
          combined.throwIfAborted()
          if (generation !== this.sessionGeneration) throw cancelled()
          if (profile.userProfile?.node.email !== expected.email || user.organizationName !== expected.organizationName)
            throw new ApiError(
              'Invitation acceptance could not be confirmed. Please retry the session check.',
              503,
              'SESSION_UNAVAILABLE',
            )
        }
        if (publish) this.acceptIdentity(user)
        return user
      } catch (error) {
        if (combined.aborted || generation !== this.sessionGeneration) throw cancelled()
        if (error instanceof ApiError && error.status === 401) {
          this.cancelVerification()
          this.scopeKey = undefined
          this.client.resetScope()
          this.store.setState({ currentUser: null, status: 'anonymous' })
          return null
        }
        this.store.setState({ status: 'error' })
        throw error
      } finally {
        if (generation === this.sessionGeneration) this.sessionPending = undefined
      }
    })()
    this.sessionPending = promise
    return promise
  }
  private acceptIdentity(user: AuthUser) {
    const key = `${user.userId}:${user.organizationId}:${user.id}`
    if (key !== this.scopeKey) {
      this.cancelVerification()
      this.client.resetScope()
      this.scopeKey = key
    }
    this.store.setState({ currentUser: user, status: 'authenticated' })
  }
  invalidAuthorization = () => {
    this.stop()
    this.sessionGeneration++
    this.sessionRequest?.abort()
    this.sessionPending = undefined
    this.client.resetScope()
    this.scopeKey = undefined
    this.confirmed = false
    this.uncertain = null
    this.store.setState({ currentUser: null, status: 'anonymous' })
    this.navigate('/login')
  }
  signOut = (): Promise<void> => {
    if (this.signingOut) return this.signingOut
    this.sessionGeneration++
    this.sessionRequest?.abort()
    this.sessionPending = undefined
    this.stop()
    this.store.setState({ status: this.store.getState().currentUser ? 'authenticated' : 'anonymous' })
    this.lifecycle.setState({ signingOut: true, error: null })
    this.signingOut = (async () => {
      try {
        const result = requireData(await new AuthSessionRevokeStore().mutate(undefined, { fetch: this.client.fetch() }))
        if (!result.sessionRevoke.node)
          throw new ApiError('Session could not be revoked. Please try again.', 503, 'SIGNOUT_FAILED')
        this.client.resetScope()
        this.scopeKey = undefined
        this.confirmed = false
        this.uncertain = null
        this.store.setState({ currentUser: null, status: 'anonymous' })
        await this.url.navigate('/login', { replace: true })
      } catch (error) {
        this.lifecycle.setState({ error: errorMessage(error) })
      } finally {
        this.signingOut = undefined
        this.lifecycle.setState({ signingOut: false })
      }
    })()
    return this.signingOut
  }
}
