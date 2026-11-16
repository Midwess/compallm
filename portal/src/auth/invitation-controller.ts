import { createStore } from 'zustand/vanilla'
import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'
import type { QueryGenqlSelection, MutationGenqlSelection, InvitationContext } from '@compallm/contract/graphql'
import { graphql } from '../graphql-client'
import { authController } from '../auth/controller'
import { urlController } from '../url/controller'
import { organizationRoot } from '../organization/paths'
import { apiErrorMessage } from '../notifications/toast'

type Context = Pick<InvitationContext, 'organizationName' | 'email' | 'expiresAt' | 'signup'>
export class InvitationController {
  readonly store = createStore<{
    context: Context | null
    loading: boolean
    step: 'continue' | 'code' | 'confirmed'
    error: string | null
    busy: boolean
  }>(() => ({ context: null, loading: true, step: 'continue', error: null, busy: false }))
  private request?: AbortController
  private confirmed = false
  start = () => {
    authController.forgetCurrentUser()
    this.request?.abort()
    const request = new AbortController()
    this.request = request
    void this.load(request)
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
      if (new URLSearchParams(urlController.store.getState().search).has('unavailable'))
        throw { info: { message: 'Invitation is expired, revoked, consumed, or unavailable.' } }
      const selection = {
        invitationContext: { node: { organizationName: true, email: true, expiresAt: true, signup: true } },
      } satisfies QueryGenqlSelection
      const result = await graphql(request.signal).query(selection)
      if (!request.signal.aborted)
        this.store.setState({
          context: result.invitationContext.node,
          loading: false,
          step: this.confirmed
            ? 'confirmed'
            : new URLSearchParams(urlController.store.getState().search).get('step') === 'code'
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
      const input = emailRequestSchema(context.signup ? 'signup' : 'signin').parse({ email: context.email, delivery: 'COOKIE' })
      const client = graphql(request.signal)
      if (context.signup)
        await client.mutation({
          emailSignupRequest: { __args: { input }, node: { lifetime: { expiresInSeconds: true } } },
        } satisfies MutationGenqlSelection)
      else
        await client.mutation({
          emailSigninRequest: { __args: { input }, node: { lifetime: { expiresInSeconds: true } } },
        } satisfies MutationGenqlSelection)
      if (!request.signal.aborted) {
        this.store.setState({ step: 'code' })
        urlController.navigate('/invitation?step=code', { replace: true })
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
      if (!this.confirmed) {
        const input = otpRequestSchema(context.signup ? 'signup' : 'signin').parse({ code, delivery: 'COOKIE' })
        const client = graphql(request.signal)
        if (context.signup)
          await client.mutation({
            emailSignupConfirm: { __args: { input }, node: { accessLifetime: { expiresInSeconds: true } } },
          } satisfies MutationGenqlSelection)
        else
          await client.mutation({
            emailSigninConfirm: { __args: { input }, node: { accessLifetime: { expiresInSeconds: true } } },
          } satisfies MutationGenqlSelection)
        this.confirmed = true
        this.store.setState({ step: 'confirmed' })
      }
      const user = await authController.refreshCurrentUser(request.signal, false)
      if (user && !request.signal.aborted) urlController.navigate(organizationRoot(user), { replace: true })
      return true
    } catch (error) {
      if (!request.signal.aborted) this.store.setState({ error: apiErrorMessage(error) })
      return false
    } finally {
      if (!request.signal.aborted) this.store.setState({ busy: false })
    }
  }
}
