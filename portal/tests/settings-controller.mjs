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
const { UserPreferenceController } = await server.ssrLoadModule('/src/preference/user/controller.ts')
function createSettingsController(...args) {
  return new SettingsController(...args, new UserPreferenceController())
}

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
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
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
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
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
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
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

test('group add creates an empty persisted group once and focuses its editor', async () => {
  const original = globalThis.fetch,
    paths = []
  let creates = 0
  const controller = createSettingsController(
    'Research team',
    { navigate: (path) => paths.push(path) },
    { signOut() {} },
    'general',
  )
  globalThis.fetch = async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('groupCreate')) {
      creates++
      const input = Object.values(variables).find((value) => value && typeof value === 'object' && 'roleIds' in value)
      assert.match(input.name, /^New group at \d\d:\d\d:\d\d\.\d{3}$/)
      assert.deepEqual(input.roleIds, [])
      return Response.json({ data: { groupCreate: { node: { id: '52', name: input.name } } } })
    }
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
  }
  try {
    const stop = controller.start()
    controller.navigate('new-group')
    controller.navigate('new-group')
    await until(controller.store, (state) => state.groups.some((group) => group.id === '52'))
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(creates, 1)
    assert.equal(paths.at(-1), '/Research%20team/settings/group/52')
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('role add creates an empty policy once and opens the persisted role', async () => {
  const original = globalThis.fetch
  const paths = []
  let writes = 0
  const controller = createSettingsController(
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
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
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
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
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
  const controller = createSettingsController(
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

test('group detail owns paged Include state and sends only relationship deltas', async () => {
  const original = globalThis.fetch
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  const option = (id) => ({ node: { id, displayName: `Member ${id}`, email: `${id}@example.test`, available: true } })
  const writes = []
  let roleStage = 0
  globalThis.fetch = async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('groupUpdate')) {
      const input = Object.values(variables).find(
        (value) => value && typeof value === 'object' && ('addMemberIds' in value || 'addRoleIds' in value),
      )
      writes.push(input)
      if ('addRoleIds' in input) {
        roleStage++
        const roles = (roleStage === 1 ? ['B', 'C'] : ['B', 'C', 'D']).map((id) => ({
          node: { id, name: id, description: '', color: '#123456' },
        }))
        return Response.json({
          data: {
            groupUpdate: { node: { id: '52', name: 'Design', description: '', memberCount: '3', roles: { edges: roles } } },
          },
        })
      }
      return Response.json({
        data: { groupUpdate: { node: { id: '52', name: 'Design', description: '', memberCount: '3', roles: { edges: [] } } } },
      })
    }
    if (query.includes('members(')) {
      const more = Object.values(variables).some((value) => value === 'cursor-1')
      const members = {
        edges: more ? [option('2'), option('3')] : [option('1')],
        pageInfo: { endCursor: more ? 'cursor-2' : 'cursor-1', hasNextPage: !more },
      }
      return Response.json({
        data: {
          group: {
            node: { id: '52', name: 'Design', description: '', memberCount: '2', memberIds: ['1', '2'], roles: { edges: [] } },
          },
          roles: { edges: [] },
          members,
        },
      })
    }
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
  }
  try {
    const stop = controller.start()
    controller.setPage('group', '52')
    await until(controller.group.store, (state) => state.draft?.id === '52')
    controller.group.loadMore()
    await until(controller.group.store, (state) => state.members?.length === 3)
    assert.deepEqual(controller.group.store.getState().draft.memberIds, ['1', '2'])
    await controller.group.updateGroup(
      { ...controller.group.store.getState().draft, memberSelection: { 1: true, 2: true, 3: true } },
      ['draft.memberSelection.3'],
      new AbortController().signal,
    )
    assert.deepEqual(writes, [{ addMemberIds: ['3'], removeMemberIds: [] }])
    controller.group.rolesChanged(['B'])
    const first = controller.group.updateGroup(
      { ...controller.group.store.getState().draft, roleIds: ['B'] },
      ['draft.roleIds'],
      new AbortController().signal,
    )
    controller.group.rolesChanged(['B', 'D'])
    await first
    await controller.group.updateGroup(
      { ...controller.group.store.getState().draft, roleIds: ['B', 'D'] },
      ['draft.roleIds'],
      new AbortController().signal,
    )
    assert.deepEqual(writes.at(-1), { addRoleIds: ['D'], removeRoleIds: [] })
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('assigned group deletion retains the dialog and group', async () => {
  const original = globalThis.fetch
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  globalThis.fetch = async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('groupDelete'))
      return Response.json({ errors: [{ message: 'Remove all members', extensions: { code: 'GROUP_ASSIGNED', status: 409 } }] })
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [{ node: { id: '52', name: 'Design' } }] } } })
  }
  try {
    const stop = controller.start()
    await until(controller.store, (state) => state.groups.length === 1)
    controller.pages.dialogs.openDialog('delete-group', '52')
    controller.pages.dialogs.confirmDialog({ ...controller.store.getState().dialog })
    await until(controller.store, (state) => !state.busy)
    assert.equal(controller.store.getState().groups.length, 1)
    assert.equal(controller.store.getState().dialog.kind, 'delete-group')
    stop()
  } finally {
    globalThis.fetch = original
  }
})

test('role deletion navigates to blank settings after confirmation', async () => {
  const original = globalThis.fetch
  const paths = []
  let deletes = 0
  const controller = createSettingsController(
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
    controller.pages.dialogs.confirmDialog(structuredClone(controller.store.getState().dialog))
    controller.pages.dialogs.confirmDialog(structuredClone(controller.store.getState().dialog))
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
  const controller = createSettingsController(
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
    controller.pages.dialogs.confirmDialog(structuredClone(controller.store.getState().dialog))
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
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
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
    await assert.rejects(controller.role.updateMetadata({ name: 'Changed', description: 'Changed' }))
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

test('group navigation ignores obsolete detail and write responses', async () => {
  const original = globalThis.fetch
  const controller = createSettingsController('Research team', { navigate() {} }, { signOut() {} }, 'general')
  const pending = []
  const node = (id) => ({
    id,
    name: id,
    description: '',
    memberCount: '0',
    roles: { edges: [] },
    memberIds: [],
  })
  globalThis.fetch = async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('groupUpdate')) return new Promise((resolve) => pending.push({ write: true, resolve }))
    if (query.includes('group(')) {
      const id = Object.values(variables).find((value) => typeof value === 'string')
      if (id === 'old') return new Promise((resolve) => pending.push({ id, resolve }))
      return Response.json({
        data: {
          group: { node: node(id) },
          roles: { edges: [] },
          members: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } },
        },
      })
    }
    return Response.json({ data: { roles: { edges: [] }, groups: { edges: [{ node: { id: 'current', name: 'current' } }] } } })
  }
  try {
    const stop = controller.start()
    controller.setPage('group', 'old')
    controller.setPage('group', 'current')
    await until(controller.group.store, (state) => state.draft?.id === 'current')
    pending
      .find((item) => item.id === 'old')
      .resolve(
        Response.json({
          data: {
            group: { node: node('old') },
            roles: { edges: [] },
            members: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } },
          },
        }),
      )
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(controller.group.store.getState().draft.id, 'current')
    const write = controller.group.updateGroup(
      { ...controller.group.store.getState().draft, name: 'Edited' },
      ['draft.name'],
      new AbortController().signal,
    )
    await new Promise((resolve) => setTimeout(resolve, 0))
    controller.setPage('group', 'next')
    await until(controller.group.store, (state) => state.draft?.id === 'next')
    pending
      .find((item) => item.write)
      .resolve(Response.json({ data: { groupUpdate: { node: { ...node('current'), name: 'Saved old' } } } }))
    assert.equal(await write, false)
    assert.equal(controller.group.store.getState().draft.id, 'next')
    assert.equal(controller.store.getState().groups[0].name, 'current')
    stop()
  } finally {
    globalThis.fetch = original
  }
})
