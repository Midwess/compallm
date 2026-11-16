import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  ssr: { noExternal: ['@compallm/storybook'] },
})
after(() => server.close())
const { InvitationController } = await server.ssrLoadModule('/src/auth/invitation-controller.ts')
const { authController } = await server.ssrLoadModule('/src/auth/controller.ts')
const tick = () => new Promise((resolve) => setTimeout(resolve, 10))
test('landing waits for Continue and confirmed acceptance retries identity without replaying the write', async (t) => {
  const original = globalThis.fetch
  let writes = 0,
    sessions = 0
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('invitationContext'))
      return Response.json({
        data: {
          invitationContext: {
            node: { organizationName: 'Research', email: 'fixed@example.com', signup: true, expiresAt: 2000000000 },
          },
        },
      })
    if (query.includes('emailSignupRequest')) {
      assert.ok(Object.values(JSON.parse(init.body).variables).some((value) => value?.invitation === true))
      writes++
      return Response.json({ data: { emailSignupRequest: { node: { lifetime: { expiresInSeconds: 600 } } } } })
    }
    if (query.includes('emailSignupConfirm')) {
      writes++
      return Response.json({ data: { emailSignupConfirm: { node: { accessLifetime: { expiresInSeconds: 1800 } } } } })
    }
    sessions++
    return Response.json({
      errors: [{ message: 'Temporarily unavailable', extensions: { code: 'SERVICE_UNAVAILABLE', status: 503 } }],
    })
  }
  const controller = new InvitationController()
  try {
    authController.store.setState({
      currentUser: { id: 'old', userId: 'old', organizationId: 'old', organizationName: 'Old' },
      status: 'authenticated',
    })
    const stop = controller.start()
    await tick()
    assert.equal(authController.store.getState().currentUser, null)
    assert.equal(writes, 0)
    assert.equal(controller.store.getState().context.email, 'fixed@example.com')
    const { urlController } = await server.ssrLoadModule('/src/url/controller.ts')
    urlController.store.setState({ search: '' })
    t.mock.method(urlController, 'navigate', () => true)
    await controller.continue()
    assert.equal(writes, 1)
    assert.equal(controller.store.getState().step, 'code')
    await controller.confirm('123456')
    assert.equal(controller.store.getState().step, 'confirmed')
    await controller.confirm('')
    assert.equal(writes, 2)
    assert.equal(sessions, 2)
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('reloading invitation confirmation retains code entry without requesting another email', async (t) => {
  const { urlController } = await server.ssrLoadModule('/src/url/controller.ts')
  const previous = urlController.store.getState()
  urlController.store.setState({ pathname: '/invitation', search: '?step=code' })
  t.after(() => urlController.store.setState(previous))
  let writes = 0
  t.mock.method(globalThis, 'fetch', async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (!query.includes('invitationContext')) writes++
    return Response.json({
      data: {
        invitationContext: {
          node: { organizationName: 'Research', email: 'fixed@example.com', signup: true, expiresAt: 2000000000 },
        },
      },
    })
  })
  const controller = new InvitationController()
  const stop = controller.start()
  t.after(stop)
  await tick()
  assert.equal(controller.store.getState().step, 'code')
  assert.equal(writes, 0)
})

test('new and existing invitation recipients reach the security offer after OTP confirmation', async (t) => {
  const { urlController } = await server.ssrLoadModule('/src/url/controller.ts')
  const destinations = []
  t.mock.method(urlController, 'navigate', (href) => {
    destinations.push(href)
  })
  for (const signup of [true, false]) {
    const user = {
      id: signup ? 'new-session' : 'existing-session',
      userId: '42',
      organizationId: '84',
      organizationName: 'Research',
    }
    t.mock.method(globalThis, 'fetch', async (_url, init) => {
      const { query } = JSON.parse(init.body)
      if (query.includes('invitationContext'))
        return Response.json({
          data: {
            invitationContext: {
              node: { organizationName: 'Research', email: 'reader@example.com', signup, expiresAt: 2000000000 },
            },
          },
        })
      if (query.includes('emailSignupConfirm'))
        return Response.json({ data: { emailSignupConfirm: { node: { accessLifetime: { expiresInSeconds: '60' } } } } })
      if (query.includes('emailSigninConfirm'))
        return Response.json({ data: { emailSigninConfirm: { node: { accessLifetime: { expiresInSeconds: '60' } } } } })
      return Response.json({ data: { session: { node: user } } })
    })
    const controller = new InvitationController()
    const stop = controller.start()
    await tick()
    await controller.confirm('123456')
    assert.equal(destinations.at(-1), '/security')
    assert.equal(authController.securityOfferSession, user.id)
    stop()
  }
})
