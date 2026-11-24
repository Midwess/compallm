import { compressedBody } from './compression'

export interface ValidationIssue {
  path: string[]
  message: string
  code: string
}
export interface SerializedNetworkError {
  message: string
  status: number
  code: string
  validation: ValidationIssue[]
  invalidAuthorization?: boolean
}
export interface NetworkOperation {
  name: string
  text: string
  variables: object
  kind: 'query' | 'mutation' | 'other'
  reusable: boolean
}
export interface NetworkRequest {
  id: string
  epoch: number
  revision: number
  operation: NetworkOperation
}
export interface NetworkResult {
  data: Record<string, unknown>
  errors: []
}
type Waiter = {
  entry: Entry
  resolve(result: NetworkResult): void
  reject(error: unknown): void
}
type Entry = {
  controller: AbortController
  waiters: Map<string, Waiter>
  epoch: number
  generation: number
  completed: boolean
  result?: NetworkResult
  expires: number
  key?: string
  reusable: boolean
  mutation: boolean
}

const reuseMilliseconds = 100
const maximumEntries = 128
const maximumRetainedBytes = 1024 * 1024
const message = 'Request failed. Please try again.'
const record = (value: unknown): value is Record<string, unknown> =>
  !!value && typeof value === 'object' && !Array.isArray(value)
const aborted = () => new DOMException('Cancelled', 'AbortError')

export class NetworkError extends Error {
  readonly status: number
  readonly code: string
  readonly validation: ValidationIssue[]
  readonly invalidAuthorization: boolean
  constructor(message: string, status: number, code: string, validation: ValidationIssue[] = [], invalidAuthorization = false) {
    super(message)
    this.name = 'ApiError'
    this.status = status
    this.code = code
    this.validation = validation
    this.invalidAuthorization = invalidAuthorization
  }
  get info() {
    return { message: this.message }
  }
}

export function serializeNetworkError(error: unknown): SerializedNetworkError {
  if (record(error) && error.name === 'AbortError')
    return { message: 'Cancelled', status: 499, code: 'ABORTED', validation: [] }
  if (error instanceof NetworkError)
    return {
      message: error.message,
      status: error.status,
      code: error.code,
      validation: error.validation,
      invalidAuthorization: error.invalidAuthorization,
    }
  return { message, status: 503, code: 'NETWORK_ERROR', validation: [] }
}

function canonical(value: unknown): unknown {
  if (Array.isArray(value)) return value.map(canonical)
  if (record(value))
    return Object.fromEntries(
      Object.keys(value)
        .sort()
        .filter((key) => value[key] !== undefined)
        .map((key) => [key, canonical(value[key])]),
    )
  return value
}

export async function readResponse(response: Response): Promise<NetworkResult> {
  let payload: unknown
  try {
    payload = await response.json()
  } catch {
    throw new NetworkError(message, response.ok ? 502 : response.status, 'INVALID_RESPONSE')
  }
  if (
    record(payload) &&
    payload.errors != null &&
    (!Array.isArray(payload.errors) || payload.errors.some((error) => !record(error) || typeof error.message !== 'string'))
  )
    throw new NetworkError(message, 502, 'INVALID_RESPONSE')
  const failures = record(payload) && Array.isArray(payload.errors) ? payload.errors.filter(record) : []
  const failure = failures[0]
  if (!response.ok || failure) {
    const extensions = record(failure?.extensions) ? failure.extensions : {}
    const status =
      response.ok && Number.isInteger(extensions.status) ? Number(extensions.status) : response.ok ? 502 : response.status
    const code = typeof extensions.code === 'string' ? extensions.code : 'REQUEST_FAILED'
    const validation = Array.isArray(extensions.validation)
      ? extensions.validation
          .filter(
            (issue): issue is ValidationIssue =>
              record(issue) &&
              Array.isArray(issue.path) &&
              issue.path.every((part) => typeof part === 'string') &&
              typeof issue.message === 'string' &&
              typeof issue.code === 'string',
          )
          .map(({ path, message, code }) => ({ path, message, code }))
      : []
    throw new NetworkError(
      typeof failure?.message === 'string' ? failure.message : message,
      status,
      code,
      validation,
      failures.some((error) => record(error.extensions) && error.extensions.code === 'INVALID_AUTHORIZATION_STATE'),
    )
  }
  if (!record(payload) || !record(payload.data)) throw new NetworkError(message, 502, 'INVALID_RESPONSE')
  return { data: payload.data, errors: [] }
}

export class NetworkService {
  private epoch = 0
  private generation = 0
  private writes = 0
  private entries = new Map<string, Entry>()
  private waiters = new Map<string, Waiter>()
  private running = new Set<Entry>()
  private readonly send: typeof fetch
  private readonly now: () => number
  constructor(send: typeof fetch = (...args) => globalThis.fetch(...args), now = () => performance.now()) {
    this.send = send
    this.now = now
  }
  get idle() {
    return this.running.size === 0
  }
  reset(epoch: number) {
    if (epoch < this.epoch) return
    this.epoch = epoch
    this.generation++
    this.entries.clear()
    for (const id of [...this.waiters.keys()]) this.cancel(id)
  }
  private invalidate() {
    this.generation++
    this.entries.clear()
  }
  cancel(id: string) {
    const waiter = this.waiters.get(id)
    if (!waiter) return
    this.waiters.delete(id)
    waiter.entry.waiters.delete(id)
    waiter.reject(aborted())
    if (!waiter.entry.waiters.size) waiter.entry.controller.abort()
  }
  execute(request: NetworkRequest, started: () => void = () => {}): Promise<NetworkResult> {
    if (request.epoch !== this.epoch) return Promise.reject(aborted())
    const operation = request.operation
    if (
      !operation ||
      typeof operation.name !== 'string' ||
      typeof operation.text !== 'string' ||
      !record(operation.variables) ||
      !['query', 'mutation', 'other'].includes(operation.kind)
    )
      return Promise.reject(new NetworkError(message, 400, 'INVALID_NETWORK_REQUEST'))
    const mutation = operation.kind === 'mutation'
    if (mutation) {
      this.invalidate()
      this.writes++
    }
    const reusable = operation.kind === 'query' && operation.reusable === true && !this.writes
    const key = reusable
      ? JSON.stringify([
          request.epoch,
          request.revision,
          this.generation,
          operation.name,
          operation.text,
          canonical(operation.variables),
        ])
      : undefined
    const now = this.now()
    for (const [name, value] of this.entries) if (value.completed && now >= value.expires) this.entries.delete(name)
    let entry = key ? this.entries.get(key) : undefined
    if (entry?.completed && entry.result) return Promise.resolve(entry.result)
    if (!entry || entry.controller.signal.aborted) {
      entry = {
        controller: new AbortController(),
        waiters: new Map(),
        epoch: this.epoch,
        generation: this.generation,
        completed: false,
        result: undefined,
        expires: 0,
        key,
        reusable,
        mutation,
      }
      this.running.add(entry)
      if (key) this.entries.set(key, entry)
      const current = entry
      queueMicrotask(() => {
        void this.run(current, operation)
      })
    }
    const current = entry
    return new Promise<NetworkResult>((resolve, reject) => {
      const waiter: Waiter = { entry: current, resolve, reject }
      this.waiters.set(request.id, waiter)
      current.waiters.set(request.id, waiter)
      started()
    })
  }
  private async run(entry: Entry, operation: NetworkOperation) {
    const { signal } = entry.controller
    let timedOut = false
    const timer = setTimeout(() => {
      timedOut = true
      entry.controller.abort()
    }, 45000)
    try {
      signal.throwIfAborted()
      const json = JSON.stringify({ operationName: operation.name, query: operation.text, variables: operation.variables })
      const outgoing = await compressedBody(json, signal)
      signal.throwIfAborted()
      const response = await this.send('/api/graphql', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          ...(outgoing.encoding ? { 'Content-Encoding': outgoing.encoding } : {}),
        },
        credentials: 'same-origin',
        cache: 'no-store',
        signal,
        body: outgoing.body,
      })
      const result = await readResponse(response)
      signal.throwIfAborted()
      if (entry.epoch !== this.epoch || (entry.reusable && entry.generation !== this.generation)) throw aborted()
      entry.result = result
      entry.completed = true
      entry.expires = this.now() + reuseMilliseconds
      const key = entry.key
      const retained =
        key &&
        !this.writes &&
        entry.generation === this.generation &&
        entry.waiters.size &&
        new TextEncoder().encode(JSON.stringify(result)).byteLength <= maximumRetainedBytes
      if (key && retained && this.entries.get(key) === entry) {
        while (this.entries.size > maximumEntries) {
          const completed = [...this.entries].find(([, value]) => value.completed)
          if (!completed) break
          this.entries.delete(completed[0])
        }
        setTimeout(() => {
          if (this.entries.get(key) === entry) this.entries.delete(key)
        }, reuseMilliseconds)
      } else if (entry.key && this.entries.get(entry.key) === entry) this.entries.delete(entry.key)
      this.finish(entry, undefined, result)
    } catch (error) {
      if (entry.key && this.entries.get(entry.key) === entry) this.entries.delete(entry.key)
      this.finish(entry, timedOut ? new NetworkError(message, 504, 'REQUEST_TIMEOUT') : error)
    } finally {
      clearTimeout(timer)
      this.running.delete(entry)
      if (entry.mutation) {
        this.writes--
        this.invalidate()
      }
    }
  }
  private finish(entry: Entry, error?: unknown, result?: NetworkResult) {
    for (const [id, waiter] of entry.waiters) {
      this.waiters.delete(id)
      if (error) waiter.reject(error)
      else if (result) waiter.resolve(result)
    }
    entry.waiters.clear()
  }
}
