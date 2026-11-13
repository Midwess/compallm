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

test('loads role and group summaries in one request', async () => {
  const original = globalThis.fetch
  let requests = 0
  let selection = ''
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = async (_input, init) => {
    assert.equal(init.credentials, 'same-origin')
    const { query } = JSON.parse(init.body)
    requests++
    assert.ok(query.includes('roles') && query.includes('groups'))
    selection = query
    return Response.json({
      data: {
        roles: { edges: [{ node: { id: '1', name: 'Editor', color: '#c06040' } }] },
        groups: { edges: [{ node: { id: '2', name: 'Design', description: '' } }] },
      },
    })
  }
  try {
    const stop = controller.start()
    await until(controller.store, (state) => state.roles.length === 1 && state.groups.length === 1)
    assert.deepEqual(
      controller.store.getState().roles.map((role) => role.name),
      ['Editor'],
    )
    assert.equal(controller.store.getState().groups[0].name, 'Design')
    assert.equal(requests, 1)
    assert.match(selection, /roles\s*\{\s*edges\s*\{\s*node\s*\{\s*id[\s,]+name[\s,]+color\s*\}/)
    assert.equal(controller.store.getState().roles[0].color, '#c06040')
    assert.equal(controller.store.getState().error, null)
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('reports denied combined reads without presenting partial results', async () => {
  const original = globalThis.fetch
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = async () =>
    Response.json({ errors: [{ message: 'Denied', extensions: { code: 'FORBIDDEN', status: 403 } }] })
  try {
    const stop = controller.start()
    await new Promise((resolve) => setTimeout(resolve, 20))
    assert.deepEqual(controller.store.getState().roles, [])
    assert.deepEqual(controller.store.getState().groups, [])
    assert.ok(toast.getToasts().some((entry) => String(entry.id).startsWith('error:Denied:action:')))
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
    assert.equal(pending.length, 2)
    assert.ok(pending[0].signal.aborted)
    for (const [index, request] of pending.entries()) {
      request.resolve(
        Response.json({
          data: {
            roles: {
              edges: [
                { node: { id: String(index), name: index === 0 ? 'Old' : 'Current', description: '', color: '#c06040' } },
              ],
            },
            groups: { edges: [{ node: { id: String(index), name: index === 0 ? 'Old' : 'Current', description: '' } }] },
          },
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

test('group add action navigates to its creation screen', () => {
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
})

test('role add creates an empty policy once and opens the persisted role', async () => {
  const original = globalThis.fetch
  const paths = []
  let writes = 0
  const controller = new SettingsController(
    'Research team',
    { navigate: (path) => paths.push(path) },
    { signOut() {} },
    'general',
  )
  globalThis.fetch = async (_input, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('organizationPolicyActions'))
      return Response.json({ data: { organizationPolicyActions: { edges: [] } } })
    if (query.includes('role('))
      return Response.json({
        data: { role: { node: { id: '42', name: 'New role', description: '', color: '#c06040', cedarSource: '' } } },
      })
    if (!query.includes('roleCreate')) return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
    writes++
    const input = Object.values(variables).find((value) => value && typeof value === 'object' && 'cedarSource' in value)
    assert.match(input.name, /^New role at \d\d:\d\d:\d\d\.\d{3}$/)
    assert.equal(input.cedarSource, '')
    return Response.json({ data: { roleCreate: { node: { id: '42', name: input.name, description: '', color: '#c06040' } } } })
  }
  try {
    const stop = controller.start()
    controller.navigate('new-role')
    controller.navigate('new-role')
    await until(controller.store, (state) => state.roles.some((role) => role.id === '42'))
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(writes, 1)
    assert.equal(controller.store.getState().roles[0].id, '42')
    assert.equal(paths.at(-1), '/Research%20team/settings/role/42')
    controller.setPage('role', '42')
    await until(controller.role.store, (state) => state.draft?.id === '42')
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('selecting a role reads its detail and saved policy', async () => {
  const original = globalThis.fetch
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('organizationPolicyActions'))
      return Response.json({ data: { organizationPolicyActions: { edges: [] } } })
    if (query.includes('role('))
      return Response.json({
        data: {
          role: {
            node: {
              id: '9',
              name: 'Editor',
              description: 'Example',
              color: '#c06040',
              cedarSource: 'permit(principal, action, resource);',
            },
          },
        },
      })
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
  }
  try {
    const stop = controller.start()
    controller.setPage('role', '9')
    await until(controller.role.store, (state) => state.draft?.id === '9')
    assert.equal(controller.role.store.getState().draft.source, 'permit(principal, action, resource);')
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('reports policy access denial without presenting partial role data', async () => {
  const original = globalThis.fetch
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('organizationPolicyActions'))
      return Response.json({ data: { organizationPolicyActions: { edges: [] } } })
    if (query.includes('role('))
      return Response.json({
        data: { role: { node: { id: '9', name: 'Editor', description: 'Example', color: '#c06040', cedarSource: null } } },
        errors: [{ message: 'Policy access denied', extensions: { code: 'FORBIDDEN', status: 403 } }],
      })
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
  }
  try {
    const stop = controller.start()
    controller.setPage('role', '9')
    await until(controller.role.store, (state) => !state.loading)
    assert.equal(controller.role.store.getState().draft, null)
    assert.ok(toast.getToasts().some((entry) => String(entry.id).startsWith('error:Policy access denied:action:')))
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('failed role creation keeps the current page and does not add a role', async () => {
  const original = globalThis.fetch
  const paths = []
  const controller = new SettingsController(
    'Research team',
    { navigate: (path) => paths.push(path) },
    { signOut() {} },
    'general',
  )
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    return query.includes('roleCreate')
      ? Response.json({ errors: [{ message: 'Name already exists', extensions: { code: 'CONFLICT', status: 409 } }] })
      : Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
  }
  try {
    const stop = controller.start()
    controller.navigate('new-role')
    await new Promise((resolve) => setTimeout(resolve, 20))
    assert.deepEqual(paths, [])
    assert.deepEqual(controller.store.getState().roles, [])
    assert.ok(toast.getToasts().some((entry) => entry.id === 'error:Name already exists'))
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('direct visits initialize the group creation screen', () => {
  for (const page of ['new-group']) {
    const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, page)
    const state = controller.store.getState()
    assert.equal(state.location.page, page)
    assert.ok(page === 'new-group' ? controller.group.store.getState().draft : controller.role.store.getState().draft)
  }
})

test('role and group drafts remain independent when navigating between pages', () => {
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  controller.role.open()
  controller.role.store.setState((state) => ({ draft: { ...state.draft, name: 'Editor' } }))
  controller.setPage('new-group')
  controller.group.store.setState((state) => ({ draft: { ...state.draft, name: 'Design' } }))
  assert.equal(controller.role.store.getState().draft.name, 'Editor')
  assert.equal(controller.group.store.getState().draft.name, 'Design')
})

test('role deletion navigates to blank settings after confirmation', async () => {
  const original = globalThis.fetch
  const paths = []
  let deletes = 0
  const controller = new SettingsController(
    'Research team',
    { navigate: (path) => paths.push(path) },
    { signOut() {} },
    'general',
  )
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('roleDelete')) {
      deletes++
      return Response.json({ data: { roleDelete: { node: true } } })
    }
    if (query.includes('organizationPolicyActions'))
      return Response.json({ data: { organizationPolicyActions: { edges: [] } } })
    if (query.includes('role('))
      return Response.json({
        data: { role: { node: { id: '9', name: 'Editor', description: '', color: '#c06040', cedarSource: '' } } },
      })
    return Response.json({
      data: { roles: { edges: [{ node: { id: '9', name: 'Editor', color: '#c06040' } }] }, groups: { edges: [] } },
    })
  }
  try {
    const stop = controller.start()
    controller.setPage('role', '9')
    await until(controller.role.store, (state) => state.draft?.id === '9')
    controller.pages.dialogs.openDialog('delete-role', '9')
    controller.pages.dialogs.confirmDialog()
    controller.pages.dialogs.confirmDialog()
    await until(controller.store, (state) => !state.busy)
    assert.equal(deletes, 1)
    assert.equal(controller.store.getState().dialog, null)
    assert.equal(
      controller.store.getState().roles.some((role) => role.id === '9'),
      false,
    )
    assert.equal(paths.at(-1), '/Research%20team/settings/blank')
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('assigned role deletion retains the dialog and role', async () => {
  const original = globalThis.fetch
  const paths = []
  const controller = new SettingsController(
    'Research team',
    { navigate: (path) => paths.push(path) },
    { signOut() {} },
    'general',
  )
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('roleDelete'))
      return Response.json({
        errors: [{ message: 'Role has assignments', extensions: { code: 'ROLE_ASSIGNED', status: 409 } }],
      })
    return Response.json({
      data: { roles: { edges: [{ node: { id: '9', name: 'Editor', color: '#c06040' } }] }, groups: { edges: [] } },
    })
  }
  try {
    const stop = controller.start()
    await until(controller.store, (state) => state.roles.length === 1)
    controller.pages.dialogs.openDialog('delete-role', '9')
    controller.pages.dialogs.confirmDialog()
    await until(controller.store, (state) => !state.busy)
    assert.equal(controller.store.getState().dialog?.kind, 'delete-role')
    assert.equal(controller.store.getState().roles[0].id, '9')
    assert.deepEqual(paths, [])
    assert.ok(toast.getToasts().some((entry) => String(entry.id).startsWith('error:Role has assignments:action:')))
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('owner role metadata is read-only', async () => {
  const original = globalThis.fetch
  const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  let writes = 0
  globalThis.fetch = async (_input, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('roleUpdate')) {
      writes++
      return Response.json({ data: { roleUpdate: { node: { id: '9' } } } })
    }
    if (query.includes('organizationPolicyActions'))
      return Response.json({ data: { organizationPolicyActions: { edges: [] } } })
    if (query.includes('role('))
      return Response.json({
        data: {
          role: { node: { id: '9', name: 'Owner', description: 'Owner role', color: '#c06040', cedarSource: '', owner: true } },
        },
      })
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
  }
  try {
    const stop = controller.start()
    controller.setPage('role', '9')
    await until(controller.role.store, (state) => state.draft?.id === '9')
    assert.equal(controller.role.store.getState().draft.builtin, true)
    controller.role.updateRole({ name: 'Changed', description: 'Changed' })
    controller.role.commitRoleField('name')
    controller.role.setPolicySource('permit(principal, action, resource);')
    controller.role.setPolicyActions(['role.read'], true)
    assert.equal(controller.role.store.getState().draft.name, 'Owner')
    assert.equal(controller.role.store.getState().draft.source, '')
    assert.equal(writes, 0)
    stop()
  } finally {
    globalThis.fetch = original
  }
})
