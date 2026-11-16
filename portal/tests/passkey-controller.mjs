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

test('enrollment does not persist before naming and rename reconciles only the confirmed record', async (t) => {
  const { controller, stop } = setup()
  t.after(stop)
  const record = { id: '1', name: 'Macbook', registeredAt: '1', lastUsedAt: null, credentialId: 'AQ' }
  let finishWrites = 0,
    renameWrites = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('passkeyRegistrationFinish')) {
      finishWrites++
      assert.ok(Object.values(variables).some((value) => value?.name === 'Macbook'))
      return reply({ passkeyRegistrationFinish: { node: record } })
    }
    if (query.includes('passkeyUpdate')) {
      renameWrites++
      return reply({ passkeyUpdate: { node: { ...record, name: 'Work Macbook' } } })
    }
    throw new Error('Unexpected request')
  })
  controller.registration = {
    input: { token: 'ceremony', id: 'AQ', rawId: 'AQ', clientDataJson: 'AQ', attestationObject: 'AQ', delivery: 'COOKIE' },
    expiresAt: Date.now() + 300000,
  }
  controller.store.setState({ dialog: { kind: 'add' }, enrollment: 'name' })
  assert.equal(finishWrites, 0)
  assert.deepEqual(controller.store.getState().records, [])
  assert.equal(await controller.add('Macbook'), true)
  assert.equal(finishWrites, 1)
  controller.edit('1')
  assert.equal(await controller.rename('1', 'Work Macbook'), true)
  assert.equal(renameWrites, 1)
  assert.equal(controller.store.getState().records[0].name, 'Work Macbook')
  assert.equal(controller.store.getState().editingId, '1')
})

test('cancel discards the pending credential response and expired naming requires another ceremony', async (t) => {
  const { controller, stop } = setup()
  t.after(stop)
  let writes = 0
  t.mock.method(globalThis, 'fetch', async () => {
    writes++
    throw new Error('Unexpected write')
  })
  controller.registration = { input: { token: 'expired' }, expiresAt: 0 }
  controller.store.setState({ dialog: { kind: 'add' }, enrollment: 'name' })
  assert.equal(await controller.add('Macbook'), false)
  assert.equal(controller.store.getState().enrollment, 'device')
  assert.equal(writes, 0)
  controller.registration = { input: { token: 'pending' }, expiresAt: Date.now() + 300000 }
  controller.close()
  assert.equal(controller.registration, undefined)
  assert.equal(controller.store.getState().dialog, null)
})

test('management delegates verification to auth and never replays a rename', async (t) => {
  const store = createStore(() => ({ currentUser: { id: 'session', userId: '42' } }))
  let finish,
    verificationSignal,
    writes = 0
  const controller = new PasskeyController({
    store,
    requestVerification: (signal) => {
      verificationSignal = signal
      return new Promise((resolve) => {
        finish = resolve
      })
    },
  })
  t.after(controller.start())
  t.mock.method(globalThis, 'fetch', async () => {
    writes++
    return Response.json({
      data: null,
      errors: [
        {
          message: 'Verify',
          extensions: {
            code: 'REAUTHENTICATION_REQUIRED',
            status: 401,
          },
        },
      ],
    })
  })
  const pending = controller.rename('1', 'Work')
  while (!finish) await new Promise((resolve) => setImmediate(resolve))
  assert.equal(verificationSignal.aborted, false)
  finish(true)
  assert.equal(await pending, false)
  assert.equal(writes, 1)
  assert.equal(controller.store.getState().reauthentication, false)
  assert.equal(controller.store.getState().dialog, null)
})

test('native duplicate credential rejection explains the existing passkey and allows another device', async (t) => {
  const { controller, stop } = setup()
  t.after(stop)
  const originalSecure = Object.getOwnPropertyDescriptor(globalThis, 'isSecureContext')
  const originalCredential = Object.getOwnPropertyDescriptor(globalThis, 'PublicKeyCredential')
  Object.defineProperty(globalThis, 'isSecureContext', { configurable: true, value: true })
  Object.defineProperty(globalThis, 'PublicKeyCredential', { configurable: true, value: class {} })
  t.after(() => {
    for (const [key, descriptor] of [
      ['isSecureContext', originalSecure],
      ['PublicKeyCredential', originalCredential],
    ]) {
      if (descriptor) Object.defineProperty(globalThis, key, descriptor)
      else delete globalThis[key]
    }
  })
  const originalNavigator = Object.getOwnPropertyDescriptor(globalThis, 'navigator')
  Object.defineProperty(globalThis, 'navigator', {
    configurable: true,
    value: {
      credentials: {
        create: async () => {
          throw new DOMException('Excluded credential', 'InvalidStateError')
        },
      },
    },
  })
  t.after(() => {
    if (originalNavigator) Object.defineProperty(globalThis, 'navigator', originalNavigator)
  })
  t.mock.method(globalThis, 'fetch', async () =>
    reply({
      passkeyRegistrationStart: {
        node: {
          token: 'ceremony',
          expiresAt: '9999999999',
          options: {
            challenge: 'AQ',
            rpId: 'localhost',
            rpName: 'Compallm',
            userId: 'AQ',
            userName: 'owner@example.com',
            userDisplayName: 'Owner',
            algorithms: [-7],
            credentials: [],
            timeout: 300000,
            userVerification: 'required',
            residentKey: 'required',
            extensions: {},
          },
        },
      },
    }),
  )
  assert.equal(await controller.prepare(), false)
  assert.match(controller.store.getState().error, /already exists/)
})
