import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'

const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, watch: null },
  appType: 'custom',
  ssr: { noExternal: ['@compallm/storybook'] },
})
after(() => server.close())
const { PermissionController } = await server.ssrLoadModule('/src/permission/controller.ts')
const { permissionKey } = await server.ssrLoadModule('/../libs/storybook/src/permission.ts')
const { RoleController } = await server.ssrLoadModule('/src/settings/role/controller.ts')
const { GroupController } = await server.ssrLoadModule('/src/settings/group/controller.ts')
const { createSettingsStore } = await server.ssrLoadModule('/../libs/storybook/src/pages/settings/store.ts')
const decisionResponse = (checks, allow) =>
  Response.json({
    data: {
      permissionDecisions: {
        edges: checks.map((check, index) => ({ node: { index, allowed: allow(check) } })),
      },
    },
  })
const role = (id) => ({ action: 'ROLE_POLICY_UPDATE', resourceType: 'ROLE', resourceId: id })

test('first-use permission batches deduplicate, stay exact, and cache allow and deny', async () => {
  const original = globalThis.fetch
  const requests = []
  globalThis.fetch = async (_url, init) => {
    const checks = Object.values(JSON.parse(init.body).variables).find(Array.isArray)
    requests.push(checks)
    return decisionResponse(checks, (check) => check.resourceId === '11')
  }
  const controller = new PermissionController()
  try {
    await Promise.all([controller.ensure([role('11'), role('12')]), controller.ensure([role('11')])])
    assert.equal(requests.length, 1)
    assert.equal(requests[0].length, 2)
    assert.equal(controller.ability.can(role('11').action, permissionKey(role('11'))), true)
    assert.equal(controller.ability.can(role('12').action, permissionKey(role('12'))), false)
    assert.equal(controller.ability.can(role('11').action, permissionKey(role('13'))), false)
    await controller.ensure([role('11'), role('12')])
    assert.equal(requests.length, 1)
    const many = Array.from({ length: 205 }, (_, index) => role(String(1000 + index)))
    await controller.ensure(many)
    assert.deepEqual(
      requests.slice(1).map((batch) => batch.length),
      [100, 100, 5],
    )
  } finally {
    controller.stop()
    globalThis.fetch = original
  }
})

test('context identities and stopped work cannot grant unrelated or newer authority', async () => {
  const original = globalThis.fetch
  let complete
  globalThis.fetch = async (_url, init) =>
    new Promise((resolve) => {
      complete = () => resolve(decisionResponse(Object.values(JSON.parse(init.body).variables).find(Array.isArray), () => true))
    })
  const controller = new PermissionController()
  const check = { action: 'GROUP_ROLE_ASSIGN', resourceType: 'GROUP', resourceId: '5', roleId: '9' }
  try {
    const waiting = controller.ensure([check])
    await new Promise((resolve) => setImmediate(resolve))
    controller.stop()
    complete()
    await waiting
    await new Promise((resolve) => setImmediate(resolve))
    assert.deepEqual(controller.store.getState().entries, {})
    assert.equal(controller.ability.can(check.action, permissionKey(check)), false)
    const next = new PermissionController()
    globalThis.fetch = async (_url, init) =>
      decisionResponse(Object.values(JSON.parse(init.body).variables).find(Array.isArray), () => true)
    await next.ensure([check])
    assert.equal(next.ability.can(check.action, permissionKey(check)), true)
    assert.equal(next.ability.can(check.action, permissionKey({ ...check, roleId: '10' })), false)
    next.stop()
  } finally {
    controller.stop()
    globalThis.fetch = original
  }
})

test('transport failures stay unavailable until explicit retry', async () => {
  const original = globalThis.fetch
  let attempts = 0
  globalThis.fetch = async (_url, init) => {
    attempts++
    if (attempts === 1) throw new TypeError('offline')
    return decisionResponse(Object.values(JSON.parse(init.body).variables).find(Array.isArray), () => true)
  }
  const controller = new PermissionController()
  try {
    await controller.ensure([role('11')])
    assert.equal(controller.store.getState().entries[permissionKey(role('11'))].status, 'error')
    await controller.ensure([role('11')])
    assert.equal(attempts, 1)
    await controller.retry([role('11')])
    assert.equal(attempts, 2)
    assert.equal(controller.store.getState().entries[permissionKey(role('11'))].status, 'allowed')
  } finally {
    controller.stop()
    globalThis.fetch = original
  }
})

test('metadata loading omits protected policy source and inaccessible group relationships', async () => {
  const original = globalThis.fetch
  const queries = []
  let phase = 'role'
  globalThis.fetch = async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('permissionDecisions'))
      return decisionResponse(
        Object.values(variables).find(Array.isArray),
        (check) => check.action === 'GROUP_READ' || (phase === 'role' && check.action === 'ROLE_READ'),
      )
    queries.push(query)
    if (query.includes('organizationPolicyActions'))
      return Response.json({ data: { organizationPolicyActions: { edges: [] } } })
    if (query.includes('role('))
      return Response.json({
        data: { role: { node: { id: '9', name: 'Metadata', description: '', color: '', owner: false } } },
      })
    if (query.includes('group('))
      return Response.json({ data: { group: { node: { id: '7', name: 'Group', description: '' } } } })
    throw new Error('Unexpected request')
  }
  const permissions = new PermissionController()
  const settings = createSettingsStore()
  const roleController = new RoleController(settings, permissions)
  try {
    await roleController.load('9')
    assert.equal(roleController.store.getState().draft.name, 'Metadata')
    assert.equal(queries.find((query) => query.includes('role(')).includes('cedarSource'), false)
    permissions.stop()
    phase = 'group'
    const groupPermissions = new PermissionController()
    const groupController = new GroupController(settings, groupPermissions)
    try {
      await groupController.load('7')
      assert.equal(groupController.store.getState().draft.name, 'Group')
      const query = queries.find((query) => query.includes('group('))
      assert.equal(query.includes('members'), false)
      assert.equal(query.includes('roles'), false)
      assert.equal(query.includes('memberIds'), false)
    } finally {
      groupController.cancel()
      groupPermissions.stop()
    }
  } finally {
    roleController.cancel()
    permissions.stop()
    globalThis.fetch = original
  }
})
