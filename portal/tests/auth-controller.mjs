import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { createStore } from 'zustand/vanilla'

// Load application TypeScript without a browser, listener, or backend process.
const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, watch: null },
  appType: 'custom',
  ssr: { noExternal: ['@compallm/storybook'] },
})
after(() => server.close())
const { AuthController } = await server.ssrLoadModule('/src/auth/controller.ts')

function urlController() {
  const store = createStore(() => ({ pathname: '/signup', search: '', hash: '' }))
  const destinations = []
  return {
    store,
    destinations,
    navigate(href) {
      destinations.push(href)
      const url = new URL(href, 'http://localhost')
      store.setState({ pathname: url.pathname, search: url.search, hash: url.hash })
    },
  }
}
function until(store, predicate) {
  if (predicate(store.getState())) return Promise.resolve()
  return new Promise((resolve) => {
    const unsubscribe = store.subscribe((state) => {
      if (predicate(state)) {
        unsubscribe()
        resolve()
      }
    })
  })
}
const user = {
  id: '9007199254740995',
  userId: '9007199254740997',
  organizationId: '9007199254740999',
  organizationName: 'Personal',
}
const reply = (data) => Response.json({ data })

test('confirmed signup retries only identity loading after a temporary failure', { timeout: 10000 }, async () => {
  const original = globalThis.fetch
  const url = urlController()
  const controller = new AuthController(url)
  let confirmations = 0
  let identities = 0
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    assert.equal(init.credentials, 'same-origin')
    if (query.includes('signupEmailRequest'))
      return reply({ signupEmailRequest: { lifetime: { expiresInSeconds: '600', throttleSeconds: '30' } } })
    if (query.includes('signupEmailConfirm')) {
      confirmations++
      return reply({ signupEmailConfirm: { accessLifetime: { expiresInSeconds: '60' } } })
    }
    identities++
    return identities === 1
      ? Response.json(
          { errors: [{ message: 'Temporarily unavailable', extensions: { code: 'SERVICE_UNAVAILABLE', status: 503 } }] },
          { status: 200 },
        )
      : reply({ session: user })
  }
  try {
    controller.start('signup')
    controller.onEmailChanged('reader@example.com')
    await controller.requestCode()
    assert.ok(controller.store.getState().remaining > 0)
    controller.onCodeChanged('123456')
    await until(controller.store, (state) => state.pending === null && state.status === 'error')
    assert.equal(confirmations, 1)
    await controller.retry()
    assert.equal(confirmations, 1)
    assert.equal(identities, 2)
    assert.deepEqual(controller.store.getState().currentUser, user)
    assert.equal(controller.store.getState().step, 'complete')
  } finally {
    controller.stop()
    globalThis.fetch = original
  }
})

test('signout cancels deduplicated identity checks and ignores their late results', { timeout: 10000 }, async () => {
  const original = globalThis.fetch
  const controller = new AuthController(urlController())
  let resolveIdentity
  let queries = 0
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('sessionRevoke')) return reply({ sessionRevoke: true })
    queries++
    return new Promise((resolve) => {
      resolveIdentity = resolve
    })
  }
  try {
    const first = controller.ensureCurrentUser().catch((error) => error)
    const second = controller.ensureCurrentUser().catch((error) => error)
    assert.equal(queries, 1)
    await controller.signOut()
    resolveIdentity(reply({ session: user }))
    await Promise.all([first, second])
    assert.equal(controller.store.getState().currentUser, null)
    assert.equal(controller.store.getState().status, 'anonymous')
  } finally {
    controller.stop()
    globalThis.fetch = original
  }
})
