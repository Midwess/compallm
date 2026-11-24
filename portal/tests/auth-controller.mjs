import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'

const server = await createServer({
  root: fileURLToPath(new URL('..', import.meta.url)),
  configFile: false,
  plugins: [svelte({ configFile: false, compilerOptions: { experimental: { async: true } } })],
  cacheDir: join(tmpdir(), 'compallm-svelte-controller-tests'),
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  resolve: {
    alias: { $houdini: fileURLToPath(new URL('../../libs/contract/houdini/generated', import.meta.url)) },
    dedupe: ['svelte'],
  },
  ssr: { noExternal: ['@compallm/storybook', '@compallm/contract'] },
})
after(() => server.close())
const { AuthController } = await server.ssrLoadModule('/src/auth/controller.ts')
const { GraphqlClient } = await server.ssrLoadModule('/src/graphql-client.ts')
const { UrlController } = await server.ssrLoadModule('/src/url/controller.ts')
const { configureTransport } = await server.ssrLoadModule(
  new URL('../../libs/contract/houdini/client.ts', import.meta.url).pathname,
)
const user = {
  id: '9007199254740995',
  userId: '9007199254740997',
  organizationId: '9007199254740999',
  organizationName: 'Research team',
  avatarUrl: null,
  ipAddress: null,
  userAgent: null,
  signedInAt: '0',
  lastSeenAt: '0',
}
const reply = (data) => Response.json({ data })
const unavailable = () =>
  Response.json({
    data: null,
    errors: [{ message: 'Temporarily unavailable', extensions: { code: 'UNAVAILABLE', status: 503 } }],
  })
function fixture(send, path = '/login') {
  const originalWindow = globalThis.window
  globalThis.window = { location: new URL('http://localhost' + path) }
  const url = new UrlController()
  url.synchronize(globalThis.window.location)
  const destinations = []
  url.connect(async (href) => {
    destinations.push(href)
    url.synchronize(new URL(href, 'http://localhost'))
  })
  const client = new GraphqlClient(async (_input, init) => send(JSON.parse(init.body), init.signal))
  const release = configureTransport(client.request)
  const controller = new AuthController(url, client)
  return {
    controller,
    client,
    destinations,
    close() {
      controller.dispose()
      release()
      globalThis.window = originalWindow
    },
  }
}

test('confirmation retry restores identity without replaying an accepted or uncertain write', async () => {
  for (const uncertain of [false, true]) {
    let confirmations = 0,
      identityReads = 0
    let profileEmail = uncertain ? 'someone-else@example.com' : 'alex@example.com'
    const app = fixture((operation) => {
      if (operation.operationName === 'AuthEmailSigninConfirm2') {
        confirmations++
        if (uncertain) throw new TypeError('Connection lost after commit')
        return reply({
          emailSigninConfirm: {
            __typename: 'SessionCredentialsPayload',
            node: {
              __typename: 'SessionCredentials',
              accessLifetime: { __typename: 'TokenLifetime', expiresInSeconds: '600' },
            },
          },
        })
      }
      if (operation.operationName === 'AuthSession2')
        return ++identityReads === 1 ? unavailable() : reply({ session: { node: user } })
      if (operation.operationName === 'AuthUserProfile') return reply({ userProfile: { node: { email: profileEmail } } })
      throw new Error('Unexpected operation')
    }, '/login/confirmation?email=alex%40example.com')
    try {
      app.controller.start('signin')
      app.controller.form.set('code', '123456')
      await app.controller.verifyCode('123456')
      assert.equal(app.controller.store.getState().step, 'code')
      assert.equal(app.controller.form.get('code'), '123456')
      await app.controller.retry()
      if (uncertain) {
        await app.controller.retry()
        assert.equal(app.controller.store.getState().currentUser, null)
        assert.equal(app.controller.store.getState().step, 'code')
        profileEmail = 'alex@example.com'
        await app.controller.retry()
      }
      assert.equal(confirmations, 1)
      assert.equal(app.controller.store.getState().currentUser.id, user.id)
      assert.equal(app.controller.store.getState().step, 'complete')
      assert.equal(app.destinations.at(-1), '/Research%20team')
      assert.equal(app.client.activity.getState().pending, 0)
    } finally {
      app.close()
    }
  }
})

test('signout retains identity on rejection and cannot be undone by an older session read', async () => {
  let identityReads = 0,
    failRevoke = true,
    releaseIdentity
  const app = fixture((operation) => {
    if (operation.operationName === 'AuthSession2') {
      if (++identityReads === 2)
        return new Promise((resolve) => {
          releaseIdentity = () => resolve(reply({ session: { node: user } }))
        })
      return reply({ session: { node: user } })
    }
    if (operation.operationName === 'AuthSessionRevoke')
      return failRevoke ? unavailable() : reply({ sessionRevoke: { __typename: 'BooleanPayload', node: true } })
    throw new Error('Unexpected operation')
  })
  try {
    await app.controller.refreshCurrentUser()
    await app.controller.signOut()
    assert.equal(app.controller.store.getState().currentUser.id, user.id)
    assert.equal(app.controller.store.getState().status, 'authenticated')
    const pending = app.controller.refreshCurrentUser().catch((error) => error)
    while (!releaseIdentity) await new Promise((resolve) => setImmediate(resolve))
    failRevoke = false
    await app.controller.signOut()
    releaseIdentity()
    assert.equal((await pending).name, 'AbortError')
    assert.equal(app.controller.store.getState().currentUser, null)
    assert.equal(app.controller.store.getState().status, 'anonymous')
    assert.equal(app.destinations.at(-1), '/login')
  } finally {
    app.close()
  }
})

test('confirmation restoration preserves the OTP challenge without a user-session probe', async () => {
  const operations = []
  const app = fixture((operation) => {
    operations.push(operation.operationName)
    if (operation.operationName === 'AuthEmailSigninConfirm2')
      return reply({
        emailSigninConfirm: {
          __typename: 'SessionCredentialsPayload',
          node: {
            __typename: 'SessionCredentials',
            accessLifetime: { __typename: 'TokenLifetime', expiresInSeconds: '600' },
          },
        },
      })
    if (operation.operationName === 'AuthSession2') return reply({ session: { node: user } })
    throw new Error('Unexpected operation')
  }, '/login/confirmation?email=alex%40example.com')
  try {
    app.controller.start('signin')
    assert.equal(await app.controller.ensureGuestAccess(), null)
    assert.deepEqual(operations, [])
    await app.controller.verifyCode('123456')
    assert.deepEqual(operations, ['AuthEmailSigninConfirm2', 'AuthSession2'])
    assert.equal(app.controller.store.getState().currentUser.id, user.id)
  } finally {
    app.close()
  }
})
