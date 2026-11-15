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
const { MembersController } = await server.ssrLoadModule('/src/settings/members/controller.ts')
const { createSettingsStore } = await server.ssrLoadModule('@compallm/storybook/pages/settings')
const tick = () => new Promise((resolve) => setTimeout(resolve, 10))
function page(id, next = false) {
  return {
    data: {
      me: { node: { id: 'current' } },
      members: {
        edges: [
          {
            node: {
              id,
              displayName: id,
              email: `${id}@example.com`,
              available: true,
              role: { id: 'member', name: 'Member', description: '', color: '', owner: false },
              groups: [
                { id: 'research', name: 'Research', description: '' },
                { id: 'design', name: 'Design', description: '' },
              ],
            },
          },
        ],
        pageInfo: { endCursor: `cursor-${id}`, hasNextPage: next },
      },
    },
  }
}

test('member pages use server search and discard obsolete reads while retaining confirmed rows', async () => {
  const original = globalThis.fetch,
    requests = []
  globalThis.fetch = (_input, init) => {
    const body = JSON.parse(init.body)
    return new Promise((resolve) => requests.push({ ...body, signal: init.signal, resolve }))
  }
  const store = createSettingsStore(),
    controller = new MembersController(store)
  try {
    const stop = controller.start()
    await tick()
    requests[0].resolve(Response.json(page('first', true)))
    await tick()
    assert.equal(requests[0].variables.v1, 20)
    assert.equal(store.getState().members[0].role.id, 'member')
    assert.equal(store.getState().members[0].groups.length, 2)
    controller.nextPage()
    await tick()
    assert.ok(JSON.stringify(requests[1]).includes('cursor-first'))
    store.setState({ query: 'new' })
    await tick()
    assert.equal(requests.length, 2)
    assert.equal(requests[1].signal.aborted, false)
    controller.search('new')
    await tick()
    assert.equal(store.getState().members[0].id, 'first')
    assert.equal(requests[1].signal.aborted, true)
    assert.ok(JSON.stringify(requests[2]).includes('%new%'))
    requests[2].resolve(Response.json(page('new')))
    await tick()
    requests[1].resolve(Response.json(page('old')))
    await tick()
    assert.equal(store.getState().members[0].id, 'new')
    assert.equal(store.getState().membersPage, 1)
    store.setState({ query: 'unsubmitted', filters: { memberRole: '123' } })
    await tick()
    assert.ok(JSON.stringify(requests[3]).includes('123'))
    assert.ok(JSON.stringify(requests[3]).includes('%new%'))
    assert.ok(!JSON.stringify(requests[3]).includes('unsubmitted'))
    requests[3].resolve(Response.json(page('role-result')))
    await tick()
    assert.equal(store.getState().members[0].id, 'role-result')
    stop()
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('failed page reads retain rows and pagination position and cancel after leaving', async () => {
  const original = globalThis.fetch
  let requests = 0,
    pending
  globalThis.fetch = async (_url, init) => {
    requests++
    if (requests === 1) return Response.json(page('confirmed', true))
    if (requests === 2) throw new Error('network unavailable')
    return new Promise((resolve) => {
      pending = { resolve, signal: init.signal }
    })
  }
  const store = createSettingsStore(),
    controller = new MembersController(store)
  try {
    controller.start()
    await tick()
    controller.nextPage()
    await tick()
    assert.equal(store.getState().members[0].id, 'confirmed')
    assert.equal(store.getState().membersPage, 1)
    assert.equal(store.getState().membersHasPreviousPage, false)
    assert.equal(store.getState().membersLoading, false)
    controller.nextPage()
    await tick()
    controller.cancel()
    assert.equal(pending.signal.aborted, true)
    pending.resolve(Response.json(page('obsolete')))
    await tick()
    assert.equal(store.getState().members[0].id, 'confirmed')
    assert.equal(requests, 3)
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})
