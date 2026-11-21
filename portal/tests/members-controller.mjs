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
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
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
    assert.equal(store.getState().membersLoaded, true)
    assert.equal(store.getState().invitationsLoaded, true)
    assert.equal(requests[0].variables.v1, 20)
    assert.equal(store.getState().members[0].role.id, 'member')
    assert.equal(store.getState().members[0].groups.length, 2)
    controller.loadMore()
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

test('failed append retains rows and its confirmed cursor for retry and cancellation', async () => {
  const original = globalThis.fetch
  let requests = 0,
    pending
  globalThis.fetch = async (_url, init) => {
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
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
    controller.loadMore()
    await tick()
    assert.equal(store.getState().members[0].id, 'confirmed')
    assert.equal(store.getState().membersLoading, false)
    assert.equal(store.getState().membersLoadError, true)
    assert.equal(store.getState().membersLoaded, true)
    assert.equal(store.getState().membersEndCursor, 'cursor-confirmed')
    controller.loadMore()
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

test('group changes submit explicit deltas and deselecting all removes confirmed groups', async () => {
  const original = globalThis.fetch
  const writes = []
  const memberships = { research: true, design: true }
  globalThis.fetch = async (_input, init) => {
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
    const body = JSON.parse(init.body)
    if (body.query.includes('groupUpdate')) {
      const values = Object.values(body.variables)
      const id = values.find((value) => typeof value === 'string')
      const input = values.find((value) => value && typeof value === 'object')
      writes.push({ id, input })
      memberships[id] = Boolean(input.addMemberIds?.length)
      return Response.json({ data: { groupUpdate: { node: { id, memberIds: memberships[id] ? ['person'] : [] } } } })
    }
    return Response.json(page('person'))
  }
  const store = createSettingsStore({
    groups: [
      { id: 'research', name: 'Research', description: '', roleIds: [], memberIds: [], color: '' },
      { id: 'design', name: 'Design', description: '', roleIds: [], memberIds: [], color: '' },
    ],
  })
  const controller = new MembersController(store)
  try {
    controller.start()
    await tick()
    assert.deepEqual(await controller.setMemberGroups('person', []), [])
    assert.deepEqual(
      writes.map(({ id }) => id),
      ['design', 'research'],
    )
    assert.ok(writes.every(({ input }) => input.removeMemberIds.includes('person')))
    assert.equal(store.getState().members[0].groups.length, 0)
    assert.deepEqual(await controller.setMemberGroups('person', ['design', 'research']), ['design', 'research'])
    assert.equal(writes.length, 4)
    assert.equal(store.getState().members[0].groups.length, 2)
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('role selection replaces one required role through one mutation without changing groups', async () => {
  const original = globalThis.fetch,
    writes = []
  let reads = 0
  let currentRole = { id: 'member', name: 'Member', description: '', color: '', owner: false }
  globalThis.fetch = async (_url, init) => {
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
    const body = JSON.parse(init.body)
    if (body.query.includes('invitations')) return Response.json({ data: { invitations: { edges: [], pageInfo: {} } } })
    if (body.query.includes('roleAssign')) {
      writes.push(body)
      currentRole = { id: 'admin', name: 'Admin', description: '', color: '', owner: false }
      return Response.json({ data: { roleAssign: { node: currentRole } } })
    }
    reads++
    const result = page('target')
    result.data.members.edges[0].node.role = currentRole
    return Response.json(result)
  }
  const store = createSettingsStore(),
    controller = new MembersController(store)
  try {
    controller.start()
    await tick()
    const other = { ...store.getState().members[0], id: 'other' }
    store.setState((state) => ({
      members: [...state.members, other],
      membersEndCursor: 'loaded-page-two',
      membersHasNextPage: true,
    }))
    const readCount = reads
    await controller.setMemberRole('target', 'admin')
    await tick()
    assert.equal(store.getState().members[0].role.id, 'admin')
    assert.equal(store.getState().members[0].groups.length, 2)
    assert.equal(store.getState().members[1], other)
    assert.equal(store.getState().membersEndCursor, 'loaded-page-two')
    assert.equal(store.getState().membersHasNextPage, true)
    assert.equal(reads, readCount)
    assert.equal(writes.length, 1)
    assert.ok(writes[0].query.includes('roleAssign'))
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('ownership changes reconcile owners and member ancestry without reloading other pages', async (t) => {
  const { createPermissionStore, permissionKey } = await server.ssrLoadModule('../libs/storybook/src/permission.ts')
  const permissionStore = createPermissionStore()
  const facts = []
  const permissions = {
    store: permissionStore,
    async evaluate(checks) {
      permissionStore.setState((state) => ({
        entries: {
          ...state.entries,
          ...Object.fromEntries(checks.map((check) => [permissionKey(check), { check, status: 'allowed' }])),
        },
      }))
    },
    setFacts: (next) => facts.push(...next),
  }
  let reads = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.includes('invitations'))
      return Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } })
    if (query.includes('roleAssign')) {
      const owner = Object.values(variables).includes('owner')
      return Response.json({
        data: { roleAssign: { node: { id: owner ? 'owner' : 'admin', name: owner ? 'Owner' : 'Admin', owner } } },
      })
    }
    reads++
    const result = page('target')
    result.data.me.node.organizationOwner = true
    result.data.organization = { node: { ownerMemberIds: ['current'] } }
    return Response.json(result)
  })
  const store = createSettingsStore({
    groups: [{ id: 'research', name: 'Research', description: '', roleIds: ['inherited'], memberIds: ['target'], color: '' }],
  })
  const controller = new MembersController(store, permissions)
  controller.start()
  t.after(controller.cancel)
  await tick()
  const readCount = reads
  await controller.setMemberRole('target', 'owner')
  assert.deepEqual(store.getState().ownerMemberIds, ['current', 'target'])
  assert.deepEqual(facts.at(-1), {
    resourceType: 'Member',
    id: 'target',
    groupIds: ['research', 'design'],
    roleIds: ['owner', 'inherited'],
  })
  await controller.setMemberRole('target', 'admin')
  assert.deepEqual(store.getState().ownerMemberIds, ['current'])
  assert.deepEqual(facts.at(-1).roleIds, ['admin', 'inherited'])
  assert.equal(reads, readCount)
})

test('failed or obsolete role writes retain confirmed member data without replay', async () => {
  const original = globalThis.fetch
  let finish,
    writeCount = 0
  globalThis.fetch = async (_url, init) => {
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
    const body = JSON.parse(init.body)
    if (body.query.includes('roleAssign')) {
      writeCount++
      return new Promise((resolve) => {
        finish = resolve
      })
    }
    if (body.query.includes('invitations')) return Response.json({ data: { invitations: { edges: [], pageInfo: {} } } })
    return Response.json(page('target'))
  }
  const store = createSettingsStore(),
    controller = new MembersController(store)
  try {
    controller.start()
    await tick()
    const pending = controller.setMemberRole('target', 'admin')
    await tick()
    assert.equal(await controller.setMemberRole('target', 'other'), false)
    controller.cancel()
    finish(Response.json({ data: { roleAssign: { node: { id: 'admin', name: 'Admin' } } } }))
    assert.equal(await pending, false)
    assert.equal(store.getState().members[0].role.id, 'member')
    assert.equal(writeCount, 1)
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('self-role changes are rejected before a write even when permissions allow it', async () => {
  const original = globalThis.fetch
  let owner = true,
    refreshes = 0
  globalThis.fetch = async (_url, init) => {
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
    const { query } = JSON.parse(init.body)
    if (query.includes('roleAssign')) {
      owner = false
      return Response.json({
        data: { roleAssign: { node: { id: 'admin', name: 'Admin', owner: false, description: '', color: '' } } },
      })
    }
    const result = page('target')
    result.data.me = { node: { id: 'target', organizationOwner: owner } }
    result.data.organization = { node: { ownerMemberIds: owner ? ['target', 'other'] : ['other'] } }
    result.data.members.edges[0].node.role = {
      id: owner ? 'owner' : 'admin',
      name: owner ? 'Owner' : 'Admin',
      owner,
      description: '',
    }
    return Response.json(result)
  }
  const permissions = {
    store: { getState: () => ({ entries: {} }) },
    async evaluate(checks) {
      this.store.getState = () => ({
        entries: Object.fromEntries(
          checks.map((check) => [
            JSON.stringify([
              check.action,
              check.resourceType,
              check.resourceId ?? null,
              check.roleId ?? null,
              check.memberId ?? null,
              check.providerModelId ?? null,
            ]),
            { check, status: 'allowed' },
          ]),
        ),
      })
    },
    refresh() {
      refreshes++
    },
  }
  const store = createSettingsStore(),
    controller = new MembersController(store, permissions, {
      store: { getState: () => ({ currentUser: { userId: 'target' } }) },
    })
  try {
    controller.start()
    await tick()
    assert.equal(await controller.setMemberRole('target', 'admin'), false)
    await tick()
    assert.equal(refreshes, 0)
    assert.equal(owner, true)
    assert.equal(store.getState().actorIsOwner, true)
    assert.equal(store.getState().members[0].role.id, 'owner')
    assert.equal(await controller.setMemberRole('target', 'owner'), undefined)
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('a later group failure preserves the first confirmed change for explicit retry', async () => {
  const original = globalThis.fetch,
    writes = []
  let fail = true
  globalThis.fetch = async (_url, init) => {
    if (JSON.parse(init.body).query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { endCursor: null, hasNextPage: false } } } }),
      )
    const body = JSON.parse(init.body)
    if (!body.query.includes('groupUpdate')) return Response.json(page('person'))
    const id = Object.values(body.variables).find((value) => typeof value === 'string')
    writes.push(id)
    if (id === 'research' && fail) throw new Error('group unavailable')
    return Response.json({ data: { groupUpdate: { node: { id, memberIds: [] } } } })
  }
  const store = createSettingsStore({
      groups: [
        { id: 'research', name: 'Research', description: '', roleIds: [], memberIds: [], color: '' },
        { id: 'design', name: 'Design', description: '', roleIds: [], memberIds: [], color: '' },
      ],
    }),
    controller = new MembersController(store)
  try {
    controller.start()
    await tick()
    assert.equal(await controller.setMemberGroups('person', []), false)
    assert.deepEqual(
      store.getState().members[0].groups.map((group) => group.id),
      ['research'],
    )
    fail = false
    assert.deepEqual(await controller.setMemberGroups('person', []), [])
    assert.deepEqual(writes, ['design', 'research', 'research'])
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('mail admission failure retains a copyable invitation and revoke is explicit', async () => {
  const original = globalThis.fetch
  let writes = 0
  let sent = false
  globalThis.fetch = async (_input, init) => {
    const query = JSON.parse(init.body).query
    if (query.includes('invitationCreateMany')) {
      writes++
      sent = true
      return Response.json({
        data: {
          invitationCreateMany: {
            edges: [
              {
                node: {
                  invitation: { id: '1', email: 'new@example.com', status: 'pending', expiresAt: 2000000000 },
                  url: 'http://localhost/api/invitations/open?token=secret',
                  emailAdmitted: false,
                },
              },
            ],
          },
        },
      })
    }
    if (query.includes('invitationCreate')) {
      writes++
      assert.equal(Object.values(JSON.parse(init.body).variables).find((value) => value?.email)?.email, 'new@example.com')
      return Response.json({
        data: {
          invitationCreate: {
            node: {
              invitation: { id: '2', email: 'new@example.com', status: 'pending', expiresAt: 2000000000 },
              url: 'http://localhost/api/invitations/open?token=fresh',
              emailAdmitted: true,
            },
          },
        },
      })
    }
    if (query.includes('invitationRevoke')) {
      writes++
      return Response.json({
        data: { invitationRevoke: { node: { id: '2', email: 'new@example.com', status: 'revoked', expiresAt: 2000000000 } } },
      })
    }
    if (query.includes('invitations'))
      return Response.json({
        data: {
          invitations: {
            edges: sent
              ? [{ node: { id: writes >= 2 ? '2' : '1', email: 'new@example.com', status: 'pending', expiresAt: 2000000000 } }]
              : [],
            pageInfo: { hasNextPage: false, endCursor: null },
          },
        },
      })
    return Response.json(page('existing'))
  }
  const store = createSettingsStore(),
    controller = new MembersController(store)
  try {
    controller.start()
    await tick()
    assert.equal(await controller.invite('new@example.com'), true)
    await tick()
    assert.equal(store.getState().invitationEmailAdmitted, false)
    assert.match(store.getState().invitationLink, /token=secret/)
    assert.equal(writes, 1)
    assert.equal(await controller.resend('1'), true)
    await tick()
    assert.match(store.getState().invitationLink, /token=fresh/)
    await controller.revoke('2')
    assert.equal(writes, 3)
    assert.equal(store.getState().invitationLink, null)
  } finally {
    controller.cancel()
    globalThis.fetch = original
  }
})

test('invitation text splits all supported separators and rejects invalid recipients together', async () => {
  const { invitationBatchInput, invitationTextSchema } = await import('../../libs/contract/dist/invitation.js')
  const result = invitationBatchInput(
    ' One@example.com\ntwo@example.com;three@example.com,four@example.com|five@example.com one@example.com ',
  )
  assert.equal(result.invitations.length, 5)
  assert.equal(result.invitations[0].email, 'one@example.com')
  assert.equal(invitationTextSchema.safeParse({ email: 'valid@example.com;broken' }).success, false)
})

test('scroll appends deduplicate rows, gate concurrent reads, and reset on ordering changes', async (t) => {
  const requests = []
  t.mock.method(globalThis, 'fetch', (_url, init) => {
    const body = JSON.parse(init.body)
    if (body.query.includes('invitations'))
      return Promise.resolve(
        Response.json({ data: { invitations: { edges: [], pageInfo: { hasNextPage: false, endCursor: null } } } }),
      )
    return new Promise((resolve) => requests.push({ ...body, resolve, signal: init.signal }))
  })
  const store = createSettingsStore()
  const controller = new MembersController(store)
  t.after(controller.cancel)
  controller.start()
  await tick()
  requests[0].resolve(Response.json(page('first', true)))
  await tick()
  controller.loadMore()
  controller.loadMore()
  await tick()
  assert.equal(requests.length, 2)
  const second = page('second')
  second.data.members.edges.unshift(page('first').data.members.edges[0])
  requests[1].resolve(Response.json(second))
  await tick()
  assert.deepEqual(
    store.getState().members.map((row) => row.id),
    ['first', 'second'],
  )
  assert.equal(store.getState().membersComplete, true)
  controller.loadMore()
  await tick()
  assert.equal(requests.length, 2)
  controller.setOrder('DESC_NULLS_LAST')
  await tick()
  assert.ok(!JSON.stringify(requests[2].variables).includes('cursor-'))
  requests[2].resolve(Response.json(page('sorted')))
  await tick()
  assert.deepEqual(
    store.getState().members.map((row) => row.id),
    ['sorted'],
  )
})

test('invitation scrolling has an independent cursor, one active read, and recoverable append failure', async (t) => {
  const requests = []
  t.mock.method(globalThis, 'fetch', (_url, init) => {
    const body = JSON.parse(init.body)
    if (!body.query.includes('invitations')) return Promise.resolve(Response.json(page('member')))
    return new Promise((resolve, reject) => requests.push({ ...body, resolve, reject }))
  })
  const invitePage = (id, next) =>
    Response.json({
      data: {
        invitations: {
          edges: [{ node: { id, email: id + '@example.com', status: 'pending', expiresAt: 2000000000 } }],
          pageInfo: { endCursor: 'invite-' + id, hasNextPage: next },
        },
      },
    })
  const store = createSettingsStore()
  const controller = new MembersController(store)
  t.after(controller.cancel)
  controller.start()
  await tick()
  requests[0].resolve(invitePage('first', true))
  await tick()
  controller.loadMoreInvitations()
  controller.loadMoreInvitations()
  await tick()
  assert.equal(requests.length, 2)
  requests[1].reject(new Error('offline'))
  await tick()
  assert.equal(store.getState().invitationsLoadError, true)
  assert.equal(store.getState().invitations[0].id, 'first')
  controller.loadMoreInvitations()
  await tick()
  assert.ok(JSON.stringify(requests[2].variables).includes('invite-first'))
  requests[2].resolve(invitePage('second', false))
  await tick()
  assert.deepEqual(
    store.getState().invitations.map((row) => row.id),
    ['first', 'second'],
  )
  assert.equal(store.getState().invitationsLoadError, false)
  assert.equal(store.getState().members[0].id, 'member')
})
