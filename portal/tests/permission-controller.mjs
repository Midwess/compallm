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
const { PermissionController } = await server.ssrLoadModule('/src/permission/controller.ts')
const { permissionKey } = await server.ssrLoadModule('/../libs/storybook/src/permission.ts')
const { cedarWorker } = await server.ssrLoadModule('/src/workers/cedar-client.ts')
const reply = () =>
  Response.json({
    data: { me: { node: { id: '11', organizationId: '7', cedarSource: 'verified policies', cedarEntities: '[]' } } },
  })
const role = (id) => ({ action: 'role.policy.update', resourceType: 'Role', resourceId: id })

test('one me read supplies local worker checks and allow/deny results stay cached', async () => {
  const original = globalThis.fetch,
    request = cedarWorker.request
  let reads = 0
  const checks = []
  globalThis.fetch = async () => {
    reads++
    return reply()
  }
  cedarWorker.request = async (operation, [authority, requested]) => {
    assert.equal(operation, 'evaluatePermissions')
    assert.equal(authority.source, 'verified policies')
    checks.push(...requested)
    return requested.map((check) => (check.resourceId === '11' ? 'allowed' : 'denied'))
  }
  const controller = new PermissionController()
  try {
    await Promise.all([controller.ensure([role('11'), role('12')]), controller.ensure([role('11')])])
    assert.equal(reads, 1)
    assert.equal(checks.length, 2)
    assert.equal(controller.store.getState().entries[permissionKey(role('11'))].status, 'allowed')
    assert.equal(controller.store.getState().entries[permissionKey(role('12'))].status, 'denied')
    await controller.ensure([role('11'), role('12')])
    assert.equal(checks.length, 2)
    assert.equal(reads, 1)
  } finally {
    controller.stop()
    globalThis.fetch = original
    cedarWorker.request = request
  }
})

test('stopped worker checks cannot restore old authority and failures require explicit retry', async () => {
  const original = globalThis.fetch,
    request = cedarWorker.request
  globalThis.fetch = async () => reply()
  let complete
  cedarWorker.request = async () =>
    new Promise((resolve) => {
      complete = resolve
    })
  const controller = new PermissionController()
  try {
    const pending = controller.ensure([role('11')])
    await new Promise((resolve) => setImmediate(resolve))
    controller.stop()
    complete(['allowed'])
    await pending
    assert.deepEqual(controller.store.getState().entries, {})
    const next = new PermissionController()
    let attempts = 0
    cedarWorker.request = async () => {
      attempts++
      if (attempts === 1) throw new Error('worker failed')
      return ['allowed']
    }
    await next.ensure([role('11')])
    assert.equal(next.store.getState().entries[permissionKey(role('11'))].status, 'error')
    await next.ensure([role('11')])
    assert.equal(attempts, 1)
    await next.retry([role('11')])
    assert.equal(attempts, 2)
    assert.equal(next.store.getState().entries[permissionKey(role('11'))].status, 'allowed')
    next.stop()
  } finally {
    controller.stop()
    globalThis.fetch = original
    cedarWorker.request = request
  }
})
