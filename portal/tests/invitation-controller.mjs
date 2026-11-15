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
test('landing waits for Continue and confirmed acceptance retries identity without replaying the write', async () => {
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
