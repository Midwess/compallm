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
const { lookupSessionLocation } = await server.ssrLoadModule('/src/settings/session-location.ts')
const { SettingsController } = await server.ssrLoadModule('/src/settings/controller.ts')
const { UserPreferenceController } = await server.ssrLoadModule('/src/preference/user/controller.ts')

const tick = () => new Promise((resolve) => setTimeout(resolve, 0))

test('location display sends only IP without credentials and silently ignores failures', async () => {
  const original = globalThis.fetch
  try {
    let requests = 0
    globalThis.fetch = async (url, init) => {
      requests++
      assert.equal(url, 'https://location.example/v1/locations')
      assert.equal(init.credentials, 'omit')
      assert.deepEqual(JSON.parse(init.body), { ip: '8.8.8.8' })
      return Response.json({ location: 'Mountain View, United States', attribution: 'https://db-ip.com' })
    }
    assert.equal(
      await lookupSessionLocation('8.8.8.8', new AbortController().signal, 'https://location.example'),
      'Mountain View, United States',
    )
    for (const response of [
      () => {
        throw new Error('offline')
      },
      () => new Response('', { status: 503 }),
      () => new Response('invalid JSON'),
      () => Response.json({ location: { unexpected: true } }),
    ]) {
      globalThis.fetch = response
      assert.equal(await lookupSessionLocation('8.8.8.8', new AbortController().signal, 'https://location.example'), null)
    }
    const abort = new AbortController()
    abort.abort()
    assert.equal(await lookupSessionLocation('8.8.8.8', abort.signal, 'https://location.example'), null)
    assert.equal(requests, 1)
  } finally {
    globalThis.fetch = original
  }
})

test('settings discard late location responses and never persist them into authenticated identity', async () => {
  const original = globalThis.fetch
  const user = { id: 'session', userId: 'user', organizationId: 'org', organizationName: 'Workspace', ipAddress: '8.8.8.8' }
  const auth = { signOut() {}, store: createStore(() => ({ currentUser: user })) }
  let reply
  globalThis.fetch = async (_url, init) => {
    if (init.credentials === 'omit')
      return new Promise((resolve) => {
        reply = resolve
      })
    return Response.json({
      data: { me: { node: { id: 'user', organizationOwner: false } }, roles: { edges: [] }, groups: { edges: [] } },
    })
  }
  const controller = new SettingsController('Workspace', { navigate() {} }, auth, 'access', new UserPreferenceController())
  const stop = controller.start()
  try {
    await tick()
    controller.setPage('general')
    reply(Response.json({ location: 'Old location' }))
    await tick()
    assert.equal(controller.store.getState().sessionLocation, null)
    controller.setPage('security')
    await tick()
    reply(Response.json({ location: 'Mountain View, United States' }))
    await tick()
    assert.deepEqual(controller.store.getState().sessionLocation, {
      sessionId: 'session',
      ipAddress: '8.8.8.8',
      location: 'Mountain View, United States',
    })
    assert.equal(auth.store.getState().currentUser, user)
    assert.equal('location' in user, false)
  } finally {
    stop()
    globalThis.fetch = original
  }
  assert.equal(controller.store.getState().sessionLocation, null)
})
