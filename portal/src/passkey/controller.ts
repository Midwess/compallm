import {
  PasskeyPasskeysAndSessionStore,
  PasskeyPasskeyRegistrationStartStore,
  PasskeyPasskeyUpdateStore,
  PasskeyPasskeysStore,
  PasskeyPasskeyRegistrationFinishStore,
  PasskeyPasskeys2Store,
  PasskeyPasskeyRemoveStore,
} from '@compallm/contract/houdini'
import { createStore } from '@compallm/storybook/state'
import type { PasskeyController as Contract, PasskeyState } from '@compallm/storybook'
import type { AuthController } from '@compallm/storybook'
import type { PasskeyFinishInput as FinishInput } from '@compallm/contract/houdini'

import { PasskeyRegistrationInput, PasskeyFinishInput } from '@compallm/contract/validation'
import { graphqlClient, requireData, ApiError } from '../graphql-client'
import { apiErrorMessage, notify } from '../notifications/toast'
import { credential } from './browser'
export class PasskeyController implements Contract {
  readonly store = createStore<PasskeyState>(() => ({
    records: [],
    enrollment: 'device',
    editingId: null,
    currentPasskeyId: null,
    loaded: false,
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
  private readonly auth: Pick<AuthController, 'store' | 'requestVerification'>
  constructor(auth: Pick<AuthController, 'store' | 'requestVerification'>) {
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
      loaded: false,
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
  seed(result: import('@compallm/contract/houdini').PasskeyPasskeysAndSession$result) {
    this.store.setState({
      records: result.passkeys.edges.map(({ node }) => ({ ...node })),
      loaded: true,
      status: 'ready',
      currentPasskeyId: result.session.node.passkey?.id ?? null,
    })
    this.seeded = true
  }
  private seeded = false
  startView = () => {
    if (this.seeded) this.seeded = false
    else void this.load()
    return this.close
  }
  load = async () => {
    this.read?.abort()
    const read = new AbortController()
    this.read = read
    const signal = AbortSignal.any([this.scope.signal, read.signal])
    this.store.setState({ status: 'loading', error: null })
    try {
      const result = await new PasskeyPasskeysAndSessionStore()
        .fetch({ fetch: graphqlClient.fetch(signal), policy: 'NoCache', blocking: true })
        .then(requireData)
      if (!signal.aborted)
        this.store.setState({
          records: result.passkeys.edges.map((e) => e.node),
          loaded: true,
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
  private async run(work: (signal: AbortSignal) => Promise<void>): Promise<boolean> {
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
        if (error instanceof ApiError && error.code === 'REAUTHENTICATION_REQUIRED') {
          this.store.setState((s) => ({ reauthentication: true, dialog: s.dialog ?? { kind: 'verify' } }))
          if (this.auth.requestVerification) {
            const verified = await this.auth.requestVerification(signal)
            if (!signal.aborted) {
              this.store.setState((s) => ({
                reauthentication: false,
                dialog: verified && s.dialog?.kind !== 'verify' ? s.dialog : null,
              }))
              if (verified && this.store.getState().dialog?.kind === 'add' && this.store.getState().enrollment === 'device') {
                this.store.setState({ busy: false })
                return await this.prepare()
              }
            }
          }
        } else {
          this.store.setState({
            error:
              error instanceof DOMException && error.name === 'InvalidStateError'
                ? 'A passkey for this account already exists on this device. Try another device or password manager.'
                : error instanceof DOMException && ['NotAllowedError', 'AbortError'].includes(error.name)
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
  prepare = (): Promise<boolean> =>
    this.run(async (signal) => {
      const {
        passkeyRegistrationStart: { node: ceremony },
      } = await new PasskeyPasskeyRegistrationStartStore()
        .mutate(undefined, { fetch: graphqlClient.fetch(signal) })
        .then(requireData)
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
      } = await new PasskeyPasskeyUpdateStore()
        .mutate({ passkeyUpdateId: id, passkeyUpdateInput: input }, { fetch: graphqlClient.fetch(signal) })
        .then(requireData)
      if (!signal.aborted) this.store.setState((s) => ({ records: s.records.map((p) => (p.id === id ? node : p)) }))
    })
  add = async (name: string) => {
    return this.run(async (signal) => {
      if (this.pendingCredential) {
        const response = await new PasskeyPasskeysStore()
          .fetch({ fetch: graphqlClient.fetch(signal), policy: 'NoCache', blocking: true })
          .then(requireData)
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
      } = await new PasskeyPasskeyRegistrationFinishStore()
        .mutate(
          { passkeyRegistrationFinishInput: response, passkeyRegistrationFinishName: nameInput },
          { fetch: graphqlClient.fetch(signal) },
        )
        .then(requireData)
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
        const response = await new PasskeyPasskeys2Store()
          .fetch({ fetch: graphqlClient.fetch(signal), policy: 'NoCache', blocking: true })
          .then(requireData)
        const records = response.passkeys.edges.map((e) => e.node)
        if (signal.aborted) return
        const confirmed = !records.some((p) => p.id === id)
        this.store.setState({
          records,
          status: 'ready',
          ...(confirmed
            ? {
                dialog: null,
                currentPasskeyId: this.store.getState().currentPasskeyId === id ? null : this.store.getState().currentPasskeyId,
              }
            : {}),
        })
        this.pendingRemoval = undefined
        if (confirmed) return
      }
      this.pendingRemoval = id
      const {
        passkeyRemove: { node },
      } = await new PasskeyPasskeyRemoveStore()
        .mutate({ passkeyRemoveId: id }, { fetch: graphqlClient.fetch(signal) })
        .then(requireData)
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
}
