import { createStore } from '@compallm/storybook/state'
import type { TransportHandler } from '@compallm/contract/houdini/client'
import { getCache } from '@compallm/contract/houdini'

import { NetworkError as ApiError } from './network/service'
import { NetworkWorkerClient } from './network/worker-client'
export { NetworkError as ApiError } from './network/service'
export function errorMessage(error: unknown) {
  return error instanceof ApiError ? error.message : 'Request failed. Please try again.'
}
export class RequestActivity {
  private visible = false
  private requests = new Set<{ show(): void; hide(): void }>()
  constructor(private readonly activity: GraphqlClient['activity']) {}
  foreground() {
    this.visible = true
    for (const request of this.requests) request.show()
  }
  background() {
    this.visible = false
    for (const request of this.requests) request.hide()
  }
  track() {
    let counted = false
    const request = {
      show: () => {
        if (counted) return
        counted = true
        this.activity.setState((state) => ({ pending: state.pending + 1 }))
      },
      hide: () => {
        if (!counted) return
        counted = false
        this.activity.setState((state) => ({ pending: state.pending - 1 }))
      },
    }
    this.requests.add(request)
    if (this.visible) request.show()
    return () => {
      this.requests.delete(request)
      request.hide()
    }
  }
}

const requestSignal = Symbol.for('compallm.graphql.request-signal')
export class GraphqlClient {
  readonly activity = createStore(() => ({ pending: 0 }))
  private readonly network: NetworkWorkerClient
  private scope = new AbortController()
  private epoch = 0
  private revision = 0
  private preparationPublished = false
  acceptsPreparation(version?: string) {
    return (!this.preparationPublished && this.revision === 0) || version === this.preparationVersion
  }
  private readonly preparationScope = globalThis.crypto.randomUUID()
  get preparationVersion() {
    return `${this.preparationScope}:${this.revision}`
  }
  publishPreparationVersion() {
    if (typeof document !== 'undefined') {
      this.preparationPublished = true
      document.cookie = `compallm-read-version=${this.preparationVersion}; Path=/; SameSite=Lax${location.protocol === 'https:' ? '; Secure' : ''}`
    }
  }
  private invalidatePrepared() {
    this.revision++
    this.publishPreparationVersion()
  }
  private invalidAuthorization?: () => void
  private readonly send: typeof globalThis.fetch
  constructor(send?: typeof globalThis.fetch) {
    this.send = send ?? ((...args) => globalThis.fetch(...args))
    this.network = new NetworkWorkerClient(send)
  }
  get readVersion() {
    return this.revision
  }
  onInvalidAuthorization(handler: () => void) {
    this.invalidAuthorization = handler
    return () => {
      if (this.invalidAuthorization === handler) this.invalidAuthorization = undefined
    }
  }
  resetScope() {
    this.scope.abort()
    this.scope = new AbortController()
    this.epoch++
    this.invalidatePrepared()
    this.network.reset(this.epoch)
    getCache().reset()
  }
  request: TransportHandler = async ({ text, name, variables, metadata }, request, descriptor) => {
    const epoch = this.epoch
    const kind = descriptor?.kind ?? (/^\s*mutation\b/.test(text) ? 'mutation' : 'query')
    const mutation = kind === 'mutation'
    const reusable = descriptor?.reusable === true && !mutation
    const signal = AbortSignal.any(
      [
        request.signal,
        this.scope.signal,
        descriptor?.fetch && (descriptor.fetch as typeof globalThis.fetch & { [requestSignal]?: AbortSignal })[requestSignal],
      ].filter((value): value is AbortSignal => !!value),
    )
    signal.throwIfAborted()
    if (mutation) this.invalidatePrepared()
    const revision = this.revision
    const preparedActivity = mutation ? undefined : metadata?.requestActivity
    const activity = preparedActivity instanceof RequestActivity ? preparedActivity : new RequestActivity(this.activity)
    if (!(preparedActivity instanceof RequestActivity)) activity.foreground()
    let finish: (() => void) | undefined
    try {
      const result = await this.network.request(
        { id: globalThis.crypto.randomUUID(), epoch, revision, operation: { text, name, variables, kind, reusable } },
        signal,
        () => {
          finish ??= activity.track()
        },
      )
      signal.throwIfAborted()
      if (epoch !== this.epoch || (reusable && revision !== this.revision)) throw new DOMException('Cancelled', 'AbortError')
      return result
    } catch (error) {
      if (epoch === this.epoch && error instanceof ApiError && error.invalidAuthorization) this.invalidAuthorization?.()
      throw error
    } finally {
      if (mutation) this.invalidatePrepared()
      finish?.()
    }
  }
  fetch = (signal?: AbortSignal): typeof globalThis.fetch => {
    const owner = AbortSignal.any([this.scope.signal, signal].filter((value): value is AbortSignal => !!value))
    const fetch: typeof globalThis.fetch = (input, init) =>
      this.send(input, {
        ...init,
        signal: AbortSignal.any([owner, init?.signal].filter((value): value is AbortSignal => !!value)),
      })
    Object.defineProperty(fetch, requestSignal, { value: owner })
    return fetch
  }
}
export type Mutable<T> = T extends readonly (infer I)[]
  ? Mutable<I>[]
  : T extends object
    ? { -readonly [K in keyof T]: Mutable<T[K]> }
    : T
export const graphqlClient = new GraphqlClient()
export function requireData<T>(result: { data: T | null; errors?: readonly unknown[] | null; partial?: boolean }): Mutable<T> {
  if (!result.data || result.errors?.length || result.partial)
    throw new ApiError('Request failed. Please try again.', 502, 'INVALID_RESPONSE')
  return result.data as Mutable<T>
}
