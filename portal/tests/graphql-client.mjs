import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'

const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
})
after(() => server.close())
const { graphql, ApiError, onInvalidAuthorization } = await server.ssrLoadModule('/src/graphql-client.ts')
const { requestActivity } = await server.ssrLoadModule('/src/request-activity.ts')

test('transport keeps activity pending through parsing and preserves cancellation without replaying writes', async (t) => {
  const abort = new AbortController()
  let release
  let writes = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    writes++
    assert.equal(init.credentials, 'same-origin')
    assert.equal(init.signal, abort.signal)
    return {
      ok: true,
      status: 200,
      json: () =>
        new Promise((resolve) => {
          release = resolve
        }),
    }
  })
  const pending = graphql(abort.signal).mutation({ sessionRevoke: { node: true } })
  await new Promise((resolve) => setImmediate(resolve))
  assert.equal(requestActivity.getState().pending, 1)
  release({ data: { sessionRevoke: { node: true } } })
  assert.deepEqual(await pending, { sessionRevoke: { node: true } })
  assert.equal(requestActivity.getState().pending, 0)
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    writes++
    init.signal.throwIfAborted()
  })
  abort.abort()
  await assert.rejects(graphql(abort.signal).mutation({ sessionRevoke: { node: true } }), { name: 'AbortError' })
  assert.equal(writes, 2)
  assert.equal(requestActivity.getState().pending, 0)
})

test('partial GraphQL results and invalid responses never establish successful state', async (t) => {
  let invalidated = 0
  onInvalidAuthorization(() => invalidated++)
  t.after(() => onInvalidAuthorization(undefined))
  t.mock.method(globalThis, 'fetch', async () =>
    Response.json({
      data: { session: { node: { id: 'untrusted' } } },
      errors: [{ message: 'Sign in again', extensions: { code: 'INVALID_AUTHORIZATION_STATE', status: 401 } }],
    }),
  )
  await assert.rejects(
    graphql().query({ session: { node: { id: true } } }),
    (error) => error instanceof ApiError && error.code === 'INVALID_AUTHORIZATION_STATE' && error.status === 401,
  )
  assert.equal(invalidated, 1)
  for (const response of [new Response('unavailable', { status: 503 }), Response.json({ unexpected: true })]) {
    t.mock.method(globalThis, 'fetch', async () => response)
    await assert.rejects(graphql().query({ session: { node: { id: true } } }), (error) => error instanceof ApiError)
    assert.equal(requestActivity.getState().pending, 0)
  }
})
