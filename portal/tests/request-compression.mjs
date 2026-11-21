import assert from 'node:assert/strict'
import { readFile } from 'node:fs/promises'
import { test } from 'node:test'
import { runInNewContext } from 'node:vm'

const source = await readFile(new URL('../public/request-compression-sw.js', import.meta.url), 'utf8')

function worker(fetch, compression = CompressionStream, requestType = Request) {
  const handlers = new Map()
  runInNewContext(source, {
    self: { location: { origin: 'https://portal.example' }, addEventListener: (name, handler) => handlers.set(name, handler) },
    Blob,
    Response,
    Request: requestType,
    ReadableStream,
    Headers,
    URL,
    CompressionStream: compression,
    fetch,
  })
  return (request) => {
    let response
    handlers.get('fetch')({ request, respondWith: (value) => (response = value) })
    return response
  }
}

function request(size, init = {}) {
  return new Request('https://portal.example/api/graphql', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json', Authorization: 'Bearer synthetic' },
    credentials: 'same-origin',
    body: JSON.stringify({ query: 'mutation { sessionRevoke { node } }', variables: { padding: 'x'.repeat(size) } }),
    ...init,
  })
}

test('worker compresses large GraphQL bodies while retaining request identity and plain fallbacks', async () => {
  const forwarded = []
  const intercept = worker(async (outgoing, options) => {
    assert.equal(options.duplex, 'half')
    forwarded.push(outgoing)
    return Response.json({ data: { sessionRevoke: { node: true } } })
  })
  const original = request(20000)
  await intercept(original)
  const outgoing = forwarded.pop()
  assert.equal(outgoing.headers.get('Content-Encoding'), 'gzip')
  assert.equal(outgoing.headers.get('Authorization'), 'Bearer synthetic')
  assert.equal(outgoing.credentials, original.credentials)
  assert.equal(outgoing.method, original.method)
  const decoded = await new Response(outgoing.body.pipeThrough(new DecompressionStream('gzip'))).text()
  assert.equal(decoded, await original.text())
  for (const size of [0, 1024 * 1024 + 1]) {
    const plain = request(size)
    await intercept(plain)
    assert.equal(forwarded.pop(), plain)
  }
  const encoded = request(20000, { headers: { 'Content-Type': 'application/json', 'Content-Encoding': 'gzip' } })
  assert.equal(intercept(encoded), undefined)
  assert.equal(intercept(new Request('https://elsewhere.example/api/graphql', { method: 'POST', body: '{}' })), undefined)
})

test('compression failures fall back before sending and network failures never replay a write', async () => {
  let attempts = 0
  class BrokenCompression {
    constructor() {
      throw new Error('Compression unavailable')
    }
  }
  const original = request(20000)
  const intercept = worker(async (outgoing) => {
    attempts++
    assert.equal(outgoing, original)
    throw new TypeError('Network failure')
  }, BrokenCompression)
  await assert.rejects(intercept(original), TypeError)
  assert.equal(attempts, 1)
  const controller = new AbortController()
  controller.abort()
  await assert.rejects(intercept(request(20000, { signal: controller.signal })), { name: 'AbortError' })
  assert.equal(attempts, 1)
})

test('browsers without streaming uploads leave GraphQL requests to the browser network stack', () => {
  class UnsupportedRequest {
    constructor(url, options) {
      if (options.body instanceof ReadableStream) throw new DOMException('Unsupported upload', 'NotSupportedError')
    }
  }
  class StringifiedStreamRequest {
    constructor() {
      this.headers = new Headers({ 'Content-Type': 'text/plain' })
    }
  }
  for (const requestType of [UnsupportedRequest, StringifiedStreamRequest]) {
    let forwarded = 0
    const intercept = worker(
      () => {
        forwarded++
      },
      CompressionStream,
      requestType,
    )
    for (const size of [0, 20000, 1024 * 1024 + 1]) assert.equal(intercept(request(size)), undefined)
    assert.equal(forwarded, 0)
  }
})
