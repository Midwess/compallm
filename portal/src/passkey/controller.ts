import { createStore } from 'zustand/vanilla'
import type { PasskeyController as Contract, PasskeyState } from '@compallm/storybook'
import type { AuthController } from '@compallm/storybook'
import type { MutationGenqlSelection, QueryGenqlSelection, PasskeyFinishInput as FinishInput } from '@compallm/contract/graphql'
import { PasskeyRegistrationInput, PasskeyFinishInput, EmailInput, EmailCodeInput } from '@compallm/contract/validation'
import { graphql, ApiError } from '../graphql-client'
import { apiErrorMessage, notify } from '../notifications/toast'
import { credential, ceremonySelection, passkeySelection } from './browser'
export class PasskeyController implements Contract {
  readonly store = createStore<PasskeyState>(() => ({
    records: [],
    enrollment: 'device',
    editingId: null,
    currentPasskeyId: null,
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
  private registration?: { input: FinishInput; expiresAt: number }
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
    this.registration = undefined
    this.pendingCredential = undefined
    this.pendingRemoval = undefined
    this.store.setState({
      records: [],
      enrollment: 'device',
      editingId: null,
      currentPasskeyId: null,
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
        session: { node: { passkey: { id: true } } },
      } satisfies QueryGenqlSelection)
      if (!signal.aborted)
        this.store.setState({
          records: result.passkeys.edges.map((e) => e.node),
          status: 'ready',
          currentPasskeyId: result.session.node.passkey?.id ?? null,
        })
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
    this.store.setState({ dialog: { kind: 'add' }, enrollment: 'device' })
    void this.prepare()
  }
  openRemove = (id: string) => {
    this.close()
    this.store.setState({ dialog: { kind: 'remove', id } })
  }
  close = () => {
    this.read?.abort()
    this.operation?.abort()
    this.registration = undefined
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
          this.store.setState((s) => ({ reauthentication: true, dialog: s.dialog ?? { kind: 'verify' } }))
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
  prepare = () =>
    this.run(async (signal) => {
      const {
        passkeyRegistrationStart: { node: ceremony },
      } = await graphql(signal).mutation({
        passkeyRegistrationStart: { node: ceremonySelection },
      } satisfies MutationGenqlSelection)
      const input = PasskeyFinishInput.parse(await credential(ceremony.options, ceremony.token, true, signal))
      if (!signal.aborted) {
        this.registration = { input, expiresAt: Number(ceremony.expiresAt) * 1000 }
        this.store.setState({ enrollment: 'name' })
      }
    })
  edit = (id: string | null) => {
    this.store.setState({ editingId: id, error: null })
  }
  rename = (id: string, name: string) =>
    this.run(async (signal) => {
      const input = PasskeyRegistrationInput.parse({ name })
      const {
        passkeyUpdate: { node },
      } = await graphql(signal).mutation({
        passkeyUpdate: { __args: { id, input }, node: passkeySelection },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) this.store.setState((s) => ({ records: s.records.map((p) => (p.id === id ? node : p)) }))
    })
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
      const nameInput = PasskeyRegistrationInput.parse({ name })
      const registration = this.registration
      if (!registration || registration.expiresAt <= Date.now()) {
        this.registration = undefined
        this.store.setState({ enrollment: 'device' })
        throw new Error('This verification expired. Verify your device again, then save your passkey name.')
      }
      const response = registration.input
      this.pendingCredential = response.rawId
      const {
        passkeyRegistrationFinish: { node },
      } = await graphql(signal).mutation({
        passkeyRegistrationFinish: { __args: { input: response, name: nameInput }, node: passkeySelection },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) {
        this.pendingCredential = undefined
        this.registration = undefined
        this.store.setState((s) => ({
          records: [...s.records.filter((p) => p.id !== node.id), node],
          dialog: null,
          status: 'ready',
          currentPasskeyId: node.id,
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
        notify.info('Passkey revoked')
        this.store.setState((s) => ({
          records: s.records.filter((p) => p.id !== node),
          dialog: null,
          currentPasskeyId: s.currentPasskeyId === node ? null : s.currentPasskeyId,
        }))
      }
    })
  }
  verifyPasskey = () =>
    this.run(async (signal) => {
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
      if (!signal.aborted)
        this.store.setState((s) => ({
          reauthentication: false,
          emailSent: false,
          dialog: s.dialog?.kind === 'verify' ? null : s.dialog,
        }))
    })
  sendEmail = () =>
    this.run(async (signal) => {
      const user = this.auth.store.getState().currentUser
      if (!user) return
      const profile = await graphql(signal).query({
        userProfile: { __args: { id: user.userId }, node: { email: true } },
      } satisfies QueryGenqlSelection)
      const input = EmailInput.parse({ email: profile.userProfile?.node.email, purpose: 'PASSKEY_CHANGE' })
      await graphql(signal).mutation({
        emailSigninRequest: { __args: { input }, node: { lifetime: { expiresAt: true } } },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) this.store.setState({ emailSent: true })
    })
  verifyEmail = (code: string) =>
    this.run(async (signal) => {
      const input = EmailCodeInput.parse({ code, purpose: 'PASSKEY_CHANGE' })
      await graphql(signal).mutation({
        emailSigninConfirm: { __args: { input }, node: { purpose: true } },
      } satisfies MutationGenqlSelection)
      if (!signal.aborted) {
        this.store.setState((s) => ({
          reauthentication: false,
          emailSent: false,
          dialog: s.dialog?.kind === 'verify' ? null : s.dialog,
        }))
      }
    })
}
