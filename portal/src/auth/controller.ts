import { emailRequestSchema } from '@compallm/contract/auth'
import { createSignupStore, type SignupController } from '@compallm/storybook/pages/signup'
import type { SigninController } from '@compallm/storybook/pages/signin'
import type { CurrentUserResponse } from '@compallm/contract/preview'
import type { UrlController } from '@compallm/storybook'
import { urlController } from '../url/controller'
import { notify } from '../notifications/toast'

const initialFormState = createSignupStore().getInitialState()
const unavailable = () => new Error('Authentication is currently unavailable')

/** Keeps editable form state while authentication transport is unavailable. */
export class AuthController implements SignupController, SigninController {
  readonly store = createSignupStore()
  readonly links = { signup: '/', signin: '/login' }
  private mode: 'signup' | 'signin' = 'signin'
  private readonly url: Pick<UrlController, 'navigate' | 'store'>

  constructor(url: Pick<UrlController, 'navigate' | 'store'>) {
    this.url = url
  }

  navigate = (href: string) => this.url.navigate(href, { replace: href === '/login' })

  start = (mode: 'signup' | 'signin') => {
    this.mode = mode
    const location = this.url.store.getState()
    const confirmation = location.pathname.endsWith('/confirmation')
    this.store.setState({
      ...initialFormState,
      step: confirmation ? 'code' : 'email',
      email: confirmation ? (new URLSearchParams(location.search).get('email') ?? '') : '',
    })
    return this.stop
  }

  stop = () => this.store.setState({ pending: null })
  onEmailChanged = (email: string) => this.store.setState({ email, error: null, serverError: null })
  onInvalidEmail = () => this.store.setState({ error: 'invalid-email', serverError: null })
  showResend = () => this.store.setState({ resendRevealed: true })
  onCodeChanged = (code: string) => this.store.setState({ code, error: null, serverError: null })

  useDifferentEmail = () => {
    this.store.setState({ step: 'email', code: '', error: null, serverError: null })
    this.navigate(this.mode === 'signup' ? '/signup' : '/login')
  }

  requestCode = async () => {
    const input = emailRequestSchema(this.mode).safeParse({ email: this.store.getState().email, delivery: 'cookie' })
    if (!input.success) {
      this.onInvalidEmail()
      return
    }
    this.store.setState({ email: input.data.email, pending: null, serverError: unavailable().message })
  }

  resendCode = async () => {
    this.store.setState({ pending: null, serverError: unavailable().message })
  }

  retry = this.resendCode

  signOut = async () => {
    notify.error(unavailable())
  }

  ensureCurrentUser = async (): Promise<CurrentUserResponse | null> => this.refreshCurrentUser()

  refreshCurrentUser = async (signal?: AbortSignal): Promise<CurrentUserResponse | null> => {
    signal?.throwIfAborted()
    this.store.setState({ currentUser: null, status: 'error' })
    throw unavailable()
  }
}

export const authController = new AuthController(urlController)
