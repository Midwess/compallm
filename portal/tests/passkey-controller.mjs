import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { createStore } from 'zustand/vanilla'
const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  ssr: { noExternal: ['@compallm/storybook'] },
})
after(() => server.close())
const { PasskeyController } = await server.ssrLoadModule('/src/passkey/controller.ts')
const reply = (data) => Response.json({ data })
function setup() {
  const store = createStore(() => ({
    currentUser: { id: 'session', userId: '42', organizationId: '84', organizationName: 'Research' },
  }))
  const controller = new PasskeyController({ store })
  return { controller, store, stop: controller.start() }
}
test('removal reconciles an uncertain response before retrying and preserves other credentials', async (t) => {
  const { controller, stop } = setup()
  t.after(stop)
  const keep = { id: '2', name: 'Other', registeredAt: '1', lastUsedAt: null, credentialId: 'AA' }
  controller.store.setState({ records: [{ ...keep, id: '1' }, keep], status: 'ready' })
  let writes = 0,
    reads = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('passkeyRemove')) {
      writes++
      throw new TypeError('Lost response after commit')
    }
    reads++
    return reply({ passkeys: { edges: [{ node: keep }] } })
  })
  controller.openRemove('1')
  assert.equal(await controller.remove('1'), false)
  assert.equal(controller.store.getState().dialog.kind, 'remove')
  assert.equal(await controller.remove('1'), true)
  assert.equal(writes, 1)
  assert.equal(reads, 1)
  assert.deepEqual(controller.store.getState().records, [keep])
  assert.equal(controller.store.getState().dialog, null)
})
test('identity changes cancel pending passkey reads and discard late results', async (t) => {
  const { controller, store, stop } = setup()
  t.after(stop)
  let resolve
  t.mock.method(
    globalThis,
    'fetch',
    () =>
      new Promise((done) => {
        resolve = done
      }),
  )
  const pending = controller.load()
  store.setState({ currentUser: { id: 'another', userId: '99', organizationId: '84', organizationName: 'Research' } })
  resolve(
    reply({
      passkeys: {
        edges: [{ node: { id: '1', name: 'Old account', credentialId: 'AA', registeredAt: '1', lastUsedAt: null } }],
      },
    }),
  )
  await pending
  assert.deepEqual(controller.store.getState().records, [])
  assert.equal(controller.store.getState().dialog, null)
})
