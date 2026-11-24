import { self } from '$app/service-worker'
import { NetworkService, serializeNetworkError } from '../network/service'
import { compressedRequest, supportsStreamingUploads } from '../network/compression'
import { networkProtocol as protocol, type WorkerRequest, type WorkerReply, type WorkerCancellation } from '../network/protocol'

self.addEventListener('install', (event) => event.waitUntil(self.skipWaiting()))
self.addEventListener('activate', (event) => event.waitUntil(self.clients.claim()))

const canForwardRequests = supportsStreamingUploads(self.location.origin)
self.addEventListener('fetch', (event) => {
  const request = event.request
  const url = new URL(request.url)
  if (
    !canForwardRequests ||
    url.origin !== self.location.origin ||
    url.pathname !== '/api/graphql' ||
    request.method !== 'POST' ||
    request.headers.has('Content-Encoding') ||
    request.headers.get('Content-Type')?.split(';')[0].trim().toLowerCase() !== 'application/json' ||
    !request.body ||
    typeof CompressionStream === 'undefined'
  )
    return
  event.respondWith(
    (async () => {
      let outgoing = request
      try {
        outgoing = await compressedRequest(request)
      } catch (error) {
        if (request.signal.aborted) throw error
      }
      return fetch(outgoing, { duplex: 'half' } as RequestInit & { duplex: 'half' })
    })(),
  )
})
const services = new Map<string, { service: NetworkService; used: number }>()
self.addEventListener('message', (event) => {
  const data = event.data as WorkerRequest
  const port = event.ports[0]
  const source = event.source
  if (
    !port ||
    data?.protocol !== protocol ||
    !source ||
    !('id' in source) ||
    new URL(source.url).origin !== self.location.origin
  )
    return
  if (data.type === 'ping') {
    port.postMessage({ type: 'ready', protocol } satisfies WorkerReply)
    port.close()
    return
  }
  if (typeof data.client !== 'string' || !Number.isSafeInteger(data.epoch) || data.epoch < 0) {
    port.close()
    return
  }
  const now = Date.now()
  for (const [key, value] of services) if (value.service.idle && now - value.used > 1000) services.delete(key)
  const key = source.id + ':' + data.client
  let owner = services.get(key)
  if (!owner) {
    owner = { service: new NetworkService(), used: now }
    owner.service.reset(data.epoch)
    services.set(key, owner)
  }
  owner.used = now
  if (data.type === 'reset') {
    owner.service.reset(data.epoch)
    port.close()
    return
  }
  if (data.type !== 'request' || typeof data.request?.id !== 'string') {
    port.close()
    return
  }
  const currentOwner = owner
  const service = currentOwner.service
  const request = data.request
  if (request.epoch !== data.epoch) {
    port.close()
    return
  }
  port.onmessage = (message: MessageEvent<WorkerCancellation>) => {
    if (message.data?.type === 'cancel') service.cancel(request.id)
  }
  port.start()
  const work = service
    .execute(request, () => port.postMessage({ type: 'started' } satisfies WorkerReply))
    .then((result) => port.postMessage({ type: 'result', result } satisfies WorkerReply))
    .catch((error) => port.postMessage({ type: 'error', error: serializeNetworkError(error) } satisfies WorkerReply))
    .finally(() => {
      currentOwner.used = Date.now()
      port.close()
    })
  event.waitUntil(work)
})
