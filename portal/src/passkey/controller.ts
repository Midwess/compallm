import { createStore } from 'zustand/vanilla'
import type { PasskeyController as Contract, PasskeyState } from '@compallm/storybook'
import type { AuthController } from '@compallm/storybook'
import type { MutationGenqlSelection, QueryGenqlSelection } from '@compallm/contract/graphql'
import { PasskeyRegistrationInput, PasskeyFinishInput, EmailReauthenticationInput } from '@compallm/contract/validation'
import { graphql, ApiError } from '../graphql-client'
import { apiErrorMessage } from '../notifications/toast'
import { credential, ceremonySelection, passkeySelection } from './browser'
export class PasskeyController implements Contract {
  readonly store = createStore<PasskeyState>(() => ({
    records: [],
    status: 'loading',
    dialog: null,
    busy: false,
    error: null,
    reauthentication: false,
    emailSent: false,
  }))
  private scope = new AbortController()
  private read?: AbortController
  private operation?: AbortController
  private emailToken?: string
  private pendingCredential?: string
  private pendingRemoval?: string
  private readonly auth: Pick<AuthController, 'store'>
  constructor(auth: Pick<AuthController, 'store'>) {
    this.auth = auth
  }
  start = () => {
    let session = this.auth.store.getState().currentUser?.id
    const stop = this.auth.store.subscribe((state) => {
      if (state.currentUser?.id !== session) {
        session = state.currentUser?.id
        this.reset()
      }
    })
    return () => {
      stop()
      this.reset()
    }
  }
  private reset() {
    this.scope.abort()
    this.scope = new AbortController()
    this.read?.abort()
    this.operation?.abort()
    this.emailToken = undefined
    this.pendingCredential = undefined
    this.pendingRemoval = undefined
    this.store.setState({
      records: [],
      status: 'loading',
      dialog: null,
      busy: false,
      error: null,
      reauthentication: false,
      emailSent: false,
    })
  }
  startView = () => {
    void this.load()
    return this.close
  }
  load = async () => {
    this.read?.abort()
    const read = new AbortController()
    this.read = read
    const signal = AbortSignal.any([this.scope.signal, read.signal])
    this.store.setState({ status: 'loading', error: null })
    try {
      const result = await graphql(signal).query({
        passkeys: { edges: { node: passkeySelection } },
      } satisfies QueryGenqlSelection)
      if (!signal.aborted) this.store.setState({ records: result.passkeys.edges.map((e) => e.node), status: 'ready' })
    } catch (error) {
      if (!signal.aborted)
        this.store.setState({
          status: error instanceof ApiError && error.status === 403 ? 'denied' : 'error',
          error: apiErrorMessage(error),
        })
    }
  }
  openAdd = () => {
    this.close()
    this.store.setState({ dialog: { kind: 'add' } })
  }
  openRemove = (id: string) => {
    this.close()
    this.store.setState({ dialog: { kind: 'remove', id } })
  }
  close = () => {
    this.read?.abort()
    this.operation?.abort()
    this.emailToken = undefined
    this.store.setState({ dialog: null, busy: false, error: null, reauthentication: false, emailSent: false })
  }
  private async run(work: (signal: AbortSignal) => Promise<void>) {
    if (this.store.getState().busy) return false
    const operation = new AbortController()
    this.operation = operation
    const signal = AbortSignal.any([this.scope.signal, operation.signal])
    this.store.setState({ busy: true, error: null })
    try {
      await work(signal)
      return !signal.aborted
    } catch (error) {
      if (!signal.aborted) {
        if (error instanceof ApiError && error.code === 'REAUTHENTICATION_REQUIRED')
          this.store.setState({ reauthentication: true })
        else {
          this.store.setState({
            error:
              error instanceof DOMException && ['NotAllowedError', 'AbortError'].includes(error.name)
                ? 'Passkey verification was cancelled or timed out. Try again or use email.'
                : apiErrorMessage(error),
          })
        }
      }
      return false
    } finally {
      if (this.operation === operation) this.store.setState({ busy: false })
    }
  }
  add = async (name: string) => {
    return this.run(async (signal) => {
      if (this.pendingCredential) {
        const response = await graphql(signal).query({
          passkeys: { edges: { node: passkeySelection } },
        } satisfies QueryGenqlSelection)
        const records = response.passkeys.edges.map((e) => e.node)
        const confirmed = records.some((p) => p.credentialId === this.pendingCredential)
        if (signal.aborted) return
        this.store.setState({ records, status: 'ready', ...(confirmed ? { dialog: null } : {}) })
        this.pendingCredential = undefined
        if (confirmed) return
      }
      const input = PasskeyRegistrationInput.parse({ name })
      const {
        passkeyRegistrationStart: { node: ceremony },
      } = await graphql(signal).mutation({
        passkeyRegistrationStart: { __args: { input }, node: ceremonySelection },
      } satisfies MutationGenqlSelection)
      const response = PasskeyFinishInput.parse(await credential(ceremony.options, ceremony.token, true, signal))
      this.pendingCredential = response.rawId
      const {
        passkeyRegistrationFinish: { node },
      } = await graphql(signal).mutation({
        passkeyRegistrationFinish: { __args: { input: response }, node: passkeySelection },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) {
        this.pendingCredential = undefined
        this.store.setState((s) => ({
          records: [...s.records.filter((p) => p.id !== node.id), node],
          dialog: null,
          status: 'ready',
        }))
      }
    })
  }
  remove = async (id: string) => {
    return this.run(async (signal) => {
      if (this.pendingRemoval === id) {
        const response = await graphql(signal).query({
          passkeys: { edges: { node: passkeySelection } },
        } satisfies QueryGenqlSelection)
        const records = response.passkeys.edges.map((e) => e.node)
        if (signal.aborted) return
        const confirmed = !records.some((p) => p.id === id)
        this.store.setState({ records, status: 'ready', ...(confirmed ? { dialog: null } : {}) })
        this.pendingRemoval = undefined
        if (confirmed) return
      }
      this.pendingRemoval = id
      const {
        passkeyRemove: { node },
      } = await graphql(signal).mutation({ passkeyRemove: { __args: { id }, node: true } } satisfies MutationGenqlSelection)
      if (!signal.aborted) {
        this.pendingRemoval = undefined
        this.store.setState((s) => ({ records: s.records.filter((p) => p.id !== node), dialog: null }))
      }
    })
  }
  verifyPasskey = () =>
    this.run(async (signal) => {
      const {
        passkeyReauthenticationStart: { node: ceremony },
      } = await graphql(signal).mutation({
        passkeyReauthenticationStart: { node: ceremonySelection },
      } satisfies MutationGenqlSelection)
      const input = PasskeyFinishInput.parse(await credential(ceremony.options, ceremony.token, false, signal))
      await graphql(signal).mutation({
        passkeyReauthenticationFinish: { __args: { input }, node: true },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) this.store.setState({ reauthentication: false, emailSent: false })
    })
  sendEmail = () =>
    this.run(async (signal) => {
      const {
        emailReauthenticationRequest: { node },
      } = await graphql(signal).mutation({
        emailReauthenticationRequest: { node: { token: true } },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) {
        this.emailToken = node.token
        this.store.setState({ emailSent: true })
      }
    })
  verifyEmail = (code: string) =>
    this.run(async (signal) => {
      const input = EmailReauthenticationInput.parse({ token: this.emailToken, code })
      await graphql(signal).mutation({
        emailReauthenticationConfirm: { __args: { input }, node: true },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) {
        this.emailToken = undefined
        this.store.setState({ reauthentication: false, emailSent: false })
      }
    })
}
