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
const { SettingsController } = await server.ssrLoadModule('/src/settings/controller.ts')
const { toast } = await import('sonner')

function until(store, predicate) {
  if (predicate(store.getState())) return Promise.resolve()
  return new Promise((resolve) => {
    const unsubscribe = store.subscribe((state) => {
      if (predicate(state)) {
        unsubscribe()
        resolve()
      }
    })
  })
}

test('loads scoped role and group summaries independently and reports denied reads', async () => {
  const original = globalThis.fetch
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = async (_input, init) => {
    assert.equal(init.credentials, 'same-origin')
    const { query } = JSON.parse(init.body)
    return query.includes('roles')
      ? Response.json({
          data: { roles: { edges: [{ node: { id: '1', name: 'Editor', description: 'Edit', color: '#c06040' } }] } },
        })
      : Response.json({ errors: [{ message: 'Denied', extensions: { code: 'FORBIDDEN', status: 403 } }] })
  }
  try {
    const stop = controller.start()
    await until(controller.store, (state) => state.roles.length === 1)
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.deepEqual(
      controller.store.getState().roles.map((role) => role.name),
      ['Editor'],
    )
    assert.deepEqual(controller.store.getState().groups, [])
    assert.equal(controller.store.getState().roles[0].color, '#c06040')
    assert.equal(controller.store.getState().error, null)
    assert.ok(toast.getToasts().some((entry) => entry.id === 'error:Denied'))
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('ignores responses from settings after leaving or restarting', async () => {
  const original = globalThis.fetch
  const pending = []
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = (_input, init) =>
    new Promise((resolve) => pending.push({ resolve, query: JSON.parse(init.body).query, signal: init.signal }))
  try {
    const stop = controller.start()
    stop()
    const stopNext = controller.start()
    assert.ok(pending.slice(0, 2).every(({ signal }) => signal.aborted))
    for (const [index, request] of pending.entries()) {
      const field = request.query.includes('roles') ? 'roles' : 'groups'
      request.resolve(
        Response.json({
          data: { [field]: { edges: [{ node: { id: String(index), name: index < 2 ? 'Old' : 'Current', description: '' } }] } },
        }),
      )
    }
    await until(controller.store, (state) => state.roles.length === 1 && state.groups.length === 1)
    assert.equal(controller.store.getState().roles[0].name, 'Current')
    assert.equal(controller.store.getState().groups[0].name, 'Current')
    stopNext()
  } finally {
    globalThis.fetch = original
  }
})

test('role and group add actions navigate to their creation screens', () => {
  const paths = []
  const url = { navigate: (path) => paths.push(path) }
  const controller = new SettingsController('Research team', url, { signOut() {} }, 'general')
  controller.navigate('new-group')
  controller.setPage('new-group')
  assert.equal(paths.at(-1), '/Research%20team/settings/new-group')
  assert.equal(controller.store.getState().location.page, 'new-group')
  assert.deepEqual(controller.group.store.getState().draft, {
    id: null,
    name: '',
    description: '',
    memberIds: [],
    roleIds: [],
  })
  controller.navigate('new-role')
  controller.setPage('new-role')
  assert.equal(paths.at(-1), '/Research%20team/settings/new-role')
  assert.equal(controller.store.getState().location.page, 'new-role')
  assert.equal(controller.role.store.getState().draft?.id, null)
})

test('direct visits initialize the requested creation screen', () => {
  for (const page of ['new-group', 'new-role']) {
    const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, page)
    const state = controller.store.getState()
    assert.equal(state.location.page, page)
    assert.ok(page === 'new-group' ? controller.group.store.getState().draft : controller.role.store.getState().draft)
  }
})

test('role and group drafts remain independent when navigating between pages', () => {
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'new-role')
  controller.role.store.setState((state) => ({ draft: { ...state.draft, name: 'Editor' } }))
  controller.setPage('new-group')
  controller.group.store.setState((state) => ({ draft: { ...state.draft, name: 'Design' } }))
  assert.equal(controller.role.store.getState().draft.name, 'Editor')
  assert.equal(controller.group.store.getState().draft.name, 'Design')
})
