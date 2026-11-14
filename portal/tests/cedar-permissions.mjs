import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { createRequire } from 'node:module'
const require = createRequire(import.meta.url)
const cedarPath = require.resolve('@cedar-policy/cedar-wasm/nodejs')
const originalSelf = globalThis.self
let receive, readyResolve, readyReject
const ready = new Promise((resolve, reject) => {
  readyResolve = resolve
  readyReject = reject
})
const pending = new Map()
let sequence = 0
// Exercise the worker's domain logic with the official Node WASM build and the same message protocol.
globalThis.self = {
  addEventListener(_name, callback) {
    receive = callback
  },
  postMessage(message) {
    if (message.type === 'fatal') {
      readyReject(new Error(message.message))
      return
    }
    if (message.type === 'ready') {
      readyResolve()
      return
    }
    const request = pending.get(message.id)
    if (request) {
      pending.delete(message.id)
      if (message.type === 'result') request.resolve(message.result)
      else request.reject(new Error(message.message))
    }
  },
}
const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  ssr: { noExternal: ['@cedar-policy/cedar-wasm'] },
  plugins: [
    {
      name: 'node-cedar-worker',
      enforce: 'pre',
      resolveId(id) {
        if (id === '@cedar-policy/cedar-wasm/web') return 'virtual:node-cedar'
      },
      load(id) {
        if (id !== 'virtual:node-cedar') return
        return [
          "import {createRequire} from 'node:module';",
          `const c=createRequire(${JSON.stringify(import.meta.url)})(${JSON.stringify(cedarPath)});`,
          'export default async function(){};',
          'export const preparsePolicySet=c.preparsePolicySet, statefulIsAuthorized=c.statefulIsAuthorized;',
        ].join('\n')
      },
    },
  ],
})
after(async () => {
  await server.close()
  if (originalSelf === undefined) delete globalThis.self
  else globalThis.self = originalSelf
})
await server.ssrLoadModule('/src/workers/cedar.ts')
await ready
function evaluate(source, checks, facts = []) {
  const authority = {
    userId: '11',
    organizationId: '7',
    source,
    entities: JSON.stringify([
      { uid: { type: 'Compallm::Organization', id: '7' }, attrs: {}, parents: [] },
      {
        uid: { type: 'Compallm::Role', id: '7:1' },
        attrs: { organization: { __entity: { type: 'Compallm::Organization', id: '7' } } },
        parents: [{ type: 'Compallm::Organization', id: '7' }],
      },
      {
        uid: { type: 'Compallm::User', id: '11' },
        attrs: {},
        parents: [
          { type: 'Compallm::Organization', id: '7' },
          { type: 'Compallm::Role', id: '7:1' },
        ],
      },
    ]),
  }
  const id = ++sequence
  return new Promise((resolve, reject) => {
    pending.set(id, { resolve, reject })
    receive({ data: { type: 'request', id, operation: 'evaluatePermissions', args: [authority, checks, facts] } })
  })
}
test('Cedar worker preserves principal grants, explicit denies, scope, and model availability', async () => {
  const source =
    'permit(principal in Compallm::Role::"7:1",action,resource);' +
    'forbid(principal,action == Compallm::Action::"model.use",resource) unless {resource.enabled};'
  const checks = [
    { action: 'role.create', resourceType: 'Organization' },
    { action: 'role.create', resourceType: 'Organization', resourceId: '8' },
    ...['1', '2', '3'].map((resourceId) => ({ action: 'model.use', resourceType: 'Model', resourceId })),
  ]
  assert.deepEqual(
    await evaluate(source, checks, [
      { resourceType: 'Model', id: '1', enabled: true },
      { resourceType: 'Model', id: '2', enabled: false },
    ]),
    ['allowed', 'denied', 'allowed', 'denied', 'error'],
  )
  const failed = 'permit(principal,action,resource); forbid(principal,action,resource) when {9223372036854775807 + 1 > 0};'
  assert.deepEqual(await evaluate(failed, [checks[0]]), ['error'])
})
test('Cedar worker checks contextual role delegation and does not infer missing group ancestry', async () => {
  const source =
    'permit(principal,action == Compallm::Action::"group.role.assign",resource)' +
    ' when {context.role == Compallm::Role::"7:9"};' +
    'permit(principal,action == Compallm::Action::"role.assign",resource);' +
    'forbid(principal,action == Compallm::Action::"role.assign",resource == Compallm::Role::"7:10");'
  const checks = ['9', '10'].map((roleId) => ({ action: 'group.role.assign', resourceType: 'Group', resourceId: '23', roleId }))
  assert.deepEqual(await evaluate(source, checks, [{ resourceType: 'Group', id: '23', roleIds: [] }]), ['allowed', 'denied'])
  assert.deepEqual(await evaluate(source, checks), ['error', 'error'])
})
