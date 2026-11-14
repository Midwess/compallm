import type { CedarOperation, CedarOperations, CedarRequest, CedarResponse, CedarResult } from './cedar-protocol'

type Pending = { resolve(result: CedarResult<CedarOperation>): void; reject(error: Error): void; cleanup(): void }
export class CedarWorkerClient {
  private worker?: Worker
  private ready?: Promise<void>
  private resolveReady?: () => void
  private rejectReady?: (error: Error) => void
  private nextId = 0
  private readonly pending = new Map<number, Pending>()
  private readonly createWorker: () => Worker
  constructor(createWorker = () => new Worker(new URL('./cedar.ts', import.meta.url), { type: 'module', name: 'cedar' })) {
    this.createWorker = createWorker
  }
  start = (): Promise<void> => {
    if (this.ready) return this.ready
    try {
      this.worker = this.createWorker()
    } catch {
      return Promise.reject(new Error('Cedar worker could not start. Retry the operation.'))
    }
    const worker = this.worker
    this.ready = new Promise<void>((resolve, reject) => {
      this.resolveReady = resolve
      this.rejectReady = reject
    })
    worker.onmessage = (event: MessageEvent<CedarResponse>) => {
      if (this.worker !== worker) return
      const message = event.data
      if (message.type === 'fatal') {
        this.stop(new Error(message.message))
        return
      }
      if (message.type === 'ready') {
        this.resolveReady?.()
        return
      }
      const pending = this.pending.get(message.id)
      if (!pending) return
      this.pending.delete(message.id)
      pending.cleanup()
      if (message.type === 'error') pending.reject(new Error(message.message))
      else pending.resolve(message.result)
    }
    worker.onerror = (event) => {
      event.preventDefault()
      if (this.worker === worker) this.stop(new Error('Cedar worker failed. Retry the operation.'))
    }
    worker.onmessageerror = () => {
      if (this.worker === worker) this.stop(new Error('Cedar worker response could not be read. Retry the operation.'))
    }
    return this.ready
  }
  request<K extends CedarOperation>(
    operation: K,
    args: CedarOperations[K]['args'],
    signal?: AbortSignal,
  ): Promise<CedarResult<K>> {
    if (signal?.aborted) return Promise.reject(new DOMException('Cancelled', 'AbortError'))
    const id = ++this.nextId
    return new Promise((resolve, reject) => {
      const abort = () => {
        const pending = this.pending.get(id)
        if (!pending) return
        this.pending.delete(id)
        pending.cleanup()
        this.worker?.postMessage({ type: 'cancel', id } satisfies CedarRequest)
        reject(new DOMException('Cancelled', 'AbortError'))
      }
      this.pending.set(id, {
        resolve: (result) => resolve(result as CedarResult<K>),
        reject,
        cleanup: () => signal?.removeEventListener('abort', abort),
      })
      signal?.addEventListener('abort', abort, { once: true })
      const ready = this.start()
      const worker = this.worker
      void ready
        .then(() => {
          if (!this.pending.has(id) || this.worker !== worker) return
          worker!.postMessage({ type: 'request', id, operation, args } as CedarRequest)
        })
        .catch((error: unknown) => {
          const pending = this.pending.get(id)
          if (!pending) return
          this.pending.delete(id)
          pending.cleanup()
          pending.reject(error instanceof Error ? error : new Error('Cedar worker is unavailable.'))
        })
    })
  }
  stop = (error = new Error('Cedar worker stopped. Retry the operation.')) => {
    const worker = this.worker
    this.worker = undefined
    this.ready = undefined
    this.rejectReady?.(error)
    this.resolveReady = undefined
    this.rejectReady = undefined
    for (const pending of this.pending.values()) {
      pending.cleanup()
      pending.reject(error)
    }
    this.pending.clear()
    if (worker) {
      worker.onmessage = null
      worker.onerror = null
      worker.onmessageerror = null
      worker.terminate()
    }
  }
}
export const cedarWorker = new CedarWorkerClient()
export function startCedarWorker() {
  return cedarWorker.start()
}
if (import.meta.hot) import.meta.hot.dispose(() => cedarWorker.stop())
