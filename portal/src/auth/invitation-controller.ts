import {
  AuthInvitationControllerInvitationContextStore,
  AuthInvitationControllerEmailSignupRequestStore,
  AuthInvitationControllerEmailSigninRequestStore,
  AuthInvitationControllerEmailSignupConfirmStore,
  AuthInvitationControllerEmailSigninConfirmStore,
} from '@compallm/contract/houdini'
import { createStore } from '@compallm/storybook/state'
import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'
type InvitationContext =
  import('@compallm/contract/houdini').AuthInvitationControllerInvitationContext$result['invitationContext']['node']
import { ApiError, graphqlClient, requireData } from '../graphql-client'
import { rejectedConfirmation, type AuthController } from './controller'
import type { UrlController } from '../url/controller'
import { apiErrorMessage } from '../notifications/toast'

type Context = Pick<InvitationContext, 'organizationName' | 'email' | 'expiresAt' | 'signup'>
export class InvitationController {
  constructor(
    private readonly auth: AuthController,
    private readonly url: UrlController,
  ) {}
  readonly store = createStore<{
    context: Context | null
    loading: boolean
    step: 'continue' | 'code' | 'confirmed'
    error: string | null
    busy: boolean
  }>(() => ({ context: null, loading: true, step: 'continue', error: null, busy: false }))
  private request?: AbortController
  private confirmed = false
  private uncertain = false
  private seeded = false
  seed(result: import('@compallm/contract/houdini').AuthInvitationControllerInvitationContext$result | null) {
    this.store.setState({
      context: result?.invitationContext.node ?? null,
      loading: false,
      error: result ? null : 'Invitation is expired, revoked, consumed, or unavailable.',
      step: new URLSearchParams(this.url.store.getState().search).get('step') === 'code' ? 'code' : 'continue',
    })
    this.seeded = true
  }
  start = () => {
    this.auth.forgetCurrentUser()
    this.request?.abort()
    const request = new AbortController()
    this.request = request
    if (this.seeded) this.seeded = false
    else void this.load(request)
    return () => request.abort()
  }
  reload = () => {
    const request = this.request
    if (!request || request.signal.aborted) return
    this.store.setState({ loading: true, error: null })
    void this.load(request)
  }
  private async load(request: AbortController) {
    try {
      if (new URLSearchParams(this.url.store.getState().search).has('unavailable'))
        throw { info: { message: 'Invitation is expired, revoked, consumed, or unavailable.' } }

      const result = await new AuthInvitationControllerInvitationContextStore()
        .fetch({ fetch: graphqlClient.fetch(request.signal), policy: 'NoCache', blocking: true })
        .then(requireData)
      if (!request.signal.aborted)
        this.store.setState({
          context: result.invitationContext.node,
          loading: false,
          step: this.confirmed
            ? 'confirmed'
            : new URLSearchParams(this.url.store.getState().search).get('step') === 'code'
              ? 'code'
              : 'continue',
        })
    } catch (error) {
      if (!request.signal.aborted) this.store.setState({ loading: false, error: apiErrorMessage(error) })
    }
  }
  continue = async () => {
    const request = this.request,
      context = this.store.getState().context
    if (!request || !context || this.store.getState().busy) return false
    this.store.setState({ busy: true, error: null })
    try {
      const input = emailRequestSchema(context.signup ? 'signup' : 'signin').parse({
        email: context.email,
        delivery: 'COOKIE',
        invitation: true,
      })
      if (context.signup)
        await new AuthInvitationControllerEmailSignupRequestStore()
          .mutate({ emailSignupRequestInput: input }, { fetch: graphqlClient.fetch(request.signal) })
          .then(requireData)
      else
        await new AuthInvitationControllerEmailSigninRequestStore()
          .mutate({ emailSigninRequestInput: input }, { fetch: graphqlClient.fetch(request.signal) })
          .then(requireData)
      if (!request.signal.aborted) {
        this.confirmed = false
        this.uncertain = false
        this.store.setState({ step: 'code' })
        this.url.navigate('/invitation?step=code', { replace: true })
      }
      return true
    } catch (error) {
      if (!request.signal.aborted) this.store.setState({ error: apiErrorMessage(error) })
      return false
    } finally {
      if (!request.signal.aborted) this.store.setState({ busy: false })
    }
  }
  confirm = async (code: string) => {
    const request = this.request,
      context = this.store.getState().context
    if (!request || !context || this.store.getState().busy) return false
    this.store.setState({ busy: true, error: null })
    try {
      if (!this.confirmed && !this.uncertain) {
        const input = otpRequestSchema(context.signup ? 'signup' : 'signin').parse({ code, delivery: 'COOKIE' })
        this.uncertain = true
        try {
          if (context.signup)
            await new AuthInvitationControllerEmailSignupConfirmStore()
              .mutate({ emailSignupConfirmInput: input }, { fetch: graphqlClient.fetch(request.signal) })
              .then(requireData)
          else
            await new AuthInvitationControllerEmailSigninConfirmStore()
              .mutate({ emailSigninConfirmInput: input }, { fetch: graphqlClient.fetch(request.signal) })
              .then(requireData)
        } catch (error) {
          if (rejectedConfirmation(error)) this.uncertain = false
          throw error
        }
        this.confirmed = true
        this.uncertain = false
        this.store.setState({ step: 'confirmed' })
      }
      const user = await this.auth.refreshCurrentUser(request.signal, true, {
        email: context.email,
        organizationName: context.organizationName,
      })
      if (!user && !request.signal.aborted)
        throw new ApiError(
          'Invitation acceptance could not be confirmed. Please retry the session check.',
          503,
          'SESSION_UNAVAILABLE',
        )
      if (user && !request.signal.aborted) {
        this.confirmed = true
        this.uncertain = false
        this.auth.securityOfferSession = user.id
        this.url.navigate('/security', { replace: true })
      }
      return true
    } catch (error) {
      if (!request.signal.aborted) this.store.setState({ error: apiErrorMessage(error) })
      return false
    } finally {
      if (!request.signal.aborted) this.store.setState({ busy: false })
    }
  }
}
