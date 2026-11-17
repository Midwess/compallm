const minimumBytes = 16 * 1024
const maximumBufferedBytes = 1024 * 1024

self.addEventListener('install', (event) => {
  event.waitUntil(self.skipWaiting())
})

self.addEventListener('activate', (event) => {
  event.waitUntil(self.clients.claim())
})

async function prepare(request) {
  const reader = request.clone().body.getReader()
  const chunks = []
  let size = 0
  try {
    while (true) {
      request.signal.throwIfAborted()
      const { done, value } = await reader.read()
      if (done) break
      size += value.byteLength
      if (size > maximumBufferedBytes) return request
      chunks.push(value)
    }
    if (size < minimumBytes) return request
    const body = new Blob(chunks)
    const compressed = await new Response(
      body.stream().pipeThrough(new CompressionStream('gzip'), { signal: request.signal }),
    ).arrayBuffer()
    request.signal.throwIfAborted()
    if (compressed.byteLength >= size) return request
    const headers = new Headers(request.headers)
    headers.set('Content-Encoding', 'gzip')
    headers.delete('Content-Length')
    return new Request(request, { headers, body: compressed, signal: request.signal })
  } finally {
    void reader.cancel().catch(() => undefined)
    reader.releaseLock()
  }
}

async function forward(request) {
  let outgoing = request
  try {
    outgoing = await prepare(request)
  } catch (error) {
    if (request.signal.aborted) throw error
  }
  return fetch(outgoing)
}

self.addEventListener('fetch', (event) => {
  const request = event.request
  const url = new URL(request.url)
  if (
    url.origin !== self.location.origin ||
    url.pathname !== '/api/graphql' ||
    request.method !== 'POST' ||
    request.headers.has('Content-Encoding') ||
    request.headers.get('Content-Type')?.split(';')[0].trim().toLowerCase() !== 'application/json' ||
    !request.body ||
    typeof CompressionStream === 'undefined'
  ) {
    return
  }
  event.respondWith(forward(request))
})
