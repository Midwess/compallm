import { NetworkService, NetworkError, type NetworkRequest, type NetworkResult } from './service'
import { networkProtocol as protocol, type WorkerRequest, type WorkerReply } from './protocol'

const unavailable = () =>
  new NetworkError('The network worker stopped responding. Please try again.', 503, 'NETWORK_WORKER_LOST')
const cancelled = () => new DOMException('Cancelled', 'AbortError')

export class NetworkWorkerClient {
  private readonly client = globalThis.crypto.randomUUID()
  private readonly local: NetworkService
  private ready?: Promise<ServiceWorker | undefined>
  private worker?: ServiceWorker
  private readonly injected: boolean
  constructor(send?: typeof fetch) {
    this.injected = !!send
    this.local = new NetworkService(send)
  }

  private connect(): Promise<ServiceWorker | undefined> {
    if (this.injected) return Promise.resolve(undefined)
    if (this.worker?.state === 'redundant') this.ready = undefined
    return (this.ready ??= this.register().catch(() => undefined))
  }
  private async register(): Promise<ServiceWorker | undefined> {
    if (typeof navigator === 'undefined' || !('serviceWorker' in navigator) || !globalThis.isSecureContext) return
    const worker = await new Promise<ServiceWorker>((resolve, reject) => {
      const timeout = setTimeout(() => reject(unavailable()), 3000)
      void navigator.serviceWorker
        .register('/service-worker.js', { scope: '/', type: 'module', updateViaCache: 'none' })
        .then((registration) => {
          const candidate = registration.installing ?? registration.waiting ?? registration.active
          if (!candidate) throw unavailable()
          const changed = () => {
            if (candidate.state === 'activated') {
              clearTimeout(timeout)
              candidate.removeEventListener('statechange', changed)
              resolve(candidate)
            } else if (candidate.state === 'redundant') {
              clearTimeout(timeout)
              candidate.removeEventListener('statechange', changed)
              reject(unavailable())
            }
          }
          candidate.addEventListener('statechange', changed)
          changed()
        })
        .catch((error) => {
          clearTimeout(timeout)
          reject(error)
        })
    })
    await new Promise<void>((resolve, reject) => {
      const channel = new MessageChannel()
      const timer = setTimeout(() => {
        channel.port1.close()
        reject(unavailable())
      }, 1500)
      channel.port1.onmessage = (event) => {
        clearTimeout(timer)
        channel.port1.close()
        if (event.data?.type === 'ready' && event.data.protocol === protocol) resolve()
        else reject(unavailable())
      }
      try {
        worker.postMessage({ protocol, type: 'ping' } satisfies WorkerRequest, [channel.port2])
      } catch (error) {
        clearTimeout(timer)
        channel.port1.close()
        reject(error)
      }
    })
    this.worker = worker
    return worker
  }
  reset(epoch: number) {
    this.local.reset(epoch)
    if (this.worker?.state === 'activated') {
      const channel = new MessageChannel()
      try {
        this.worker.postMessage({ protocol, type: 'reset', client: this.client, epoch } satisfies WorkerRequest, [
          channel.port2,
        ])
      } catch {
        this.worker = undefined
        this.ready = undefined
      }
      channel.port1.close()
    }
  }
  async request(request: NetworkRequest, signal: AbortSignal, started: () => void): Promise<NetworkResult> {
    signal.throwIfAborted()
    const worker = await this.connect()
    signal.throwIfAborted()
    if (!worker) {
      const abort = () => this.local.cancel(request.id)
      signal.addEventListener('abort', abort, { once: true })
      try {
        return structuredClone(await this.local.execute(request, started))
      } finally {
        signal.removeEventListener('abort', abort)
      }
    }
    return new Promise<NetworkResult>((resolve, reject) => {
      const channel = new MessageChannel()
      let settled = false
      const cleanup = () => {
        settled = true
        clearTimeout(timeout)
        channel.port1.close()
        signal.removeEventListener('abort', abort)
        worker.removeEventListener('statechange', changed)
      }
      const abort = () => {
        if (settled) return
        channel.port1.postMessage({ type: 'cancel' })
        cleanup()
        reject(cancelled())
      }
      const changed = () => {
        if (worker.state !== 'redundant' || settled) return
        cleanup()
        reject(unavailable())
      }
      const timeout = setTimeout(() => {
        if (settled) return
        channel.port1.postMessage({ type: 'cancel' })
        cleanup()
        reject(unavailable())
      }, 60000)
      signal.addEventListener('abort', abort, { once: true })
      worker.addEventListener('statechange', changed)
      channel.port1.onmessage = ({ data }: MessageEvent<WorkerReply>) => {
        if (settled) return
        if (data.type === 'started') {
          started()
          return
        }
        cleanup()
        if (data.type === 'result') resolve(data.result)
        else if (data.type === 'error' && data.error.code === 'ABORTED') reject(cancelled())
        else if (data.type === 'error')
          reject(
            new NetworkError(
              data.error.message,
              data.error.status,
              data.error.code,
              data.error.validation,
              data.error.invalidAuthorization,
            ),
          )
        else reject(unavailable())
      }
      channel.port1.onmessageerror = () => {
        if (!settled) {
          cleanup()
          reject(unavailable())
        }
      }
      try {
        worker.postMessage(
          { protocol, type: 'request', client: this.client, epoch: request.epoch, request } satisfies WorkerRequest,
          [channel.port2],
        )
      } catch {
        cleanup()
        reject(unavailable())
      }
    })
  }
}
