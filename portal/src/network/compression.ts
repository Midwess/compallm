const minimumBytes = 16 * 1024
const maximumBytes = 1024 * 1024

export async function compressedBody(body: string | Blob, signal: AbortSignal) {
  const blob = typeof body === 'string' ? new Blob([body]) : body
  if (typeof CompressionStream === 'undefined' || blob.size < minimumBytes || blob.size > maximumBytes) return { body }
  try {
    const compressed = await new Response(blob.stream().pipeThrough(new CompressionStream('gzip'), { signal })).arrayBuffer()
    signal.throwIfAborted()
    return compressed.byteLength < blob.size ? { body: compressed, encoding: 'gzip' } : { body }
  } catch (error) {
    if (signal.aborted) throw error
    return { body }
  }
}

export async function compressedRequest(request: Request): Promise<Request> {
  const body = request.clone().body
  if (!body) return request
  const reader = body.getReader()
  const chunks: Uint8Array<ArrayBuffer>[] = []
  let size = 0
  try {
    while (true) {
      request.signal.throwIfAborted()
      const { done, value } = await reader.read()
      if (done) break
      size += value.byteLength
      if (size > maximumBytes) return request
      chunks.push(value)
    }
    const outgoing = await compressedBody(new Blob(chunks), request.signal)
    if (!outgoing.encoding) return request
    const headers = new Headers(request.headers)
    headers.set('Content-Encoding', outgoing.encoding)
    headers.delete('Content-Length')
    return new Request(request, { headers, body: outgoing.body, signal: request.signal })
  } finally {
    void reader.cancel().catch(() => undefined)
    reader.releaseLock()
  }
}

export function supportsStreamingUploads(origin: string): boolean {
  let duplexRead = false
  try {
    const options: RequestInit & { duplex: 'half' } = {
      method: 'POST',
      body: new ReadableStream(),
      get duplex(): 'half' {
        duplexRead = true
        return 'half'
      },
    }
    const request = new Request(origin, options)
    return duplexRead && !request.headers.has('Content-Type')
  } catch {
    return false
  }
}
