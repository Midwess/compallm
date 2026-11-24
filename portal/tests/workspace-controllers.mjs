import assert from 'node:assert/strict'
import { after, afterEach, test } from 'node:test'
import { createServer } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
import { fileURLToPath } from 'node:url'
import { createRequire } from 'node:module'
import { readFileSync } from 'node:fs'

const server = await createServer({
  root: fileURLToPath(new URL('..', import.meta.url)),
  configFile: false,
  plugins: [svelte({ configFile: false })],
  cacheDir: join(tmpdir(), 'compallm-svelte-workspace-tests'),
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  resolve: {
    alias: { $houdini: fileURLToPath(new URL('../../libs/contract/houdini/generated', import.meta.url)) },
    dedupe: ['svelte'],
  },
  ssr: { noExternal: ['@compallm/storybook', '@compallm/contract'] },
})
after(() => server.close())
const load = (path) => server.ssrLoadModule(path)
const { graphqlClient } = await load('/src/graphql-client.ts')
const { configureTransport } = await load(new URL('../../libs/contract/houdini/client.ts', import.meta.url).pathname)
const { createStore } = await load('@compallm/storybook/state')
const { createSettingsStore } = await load('@compallm/storybook/pages/settings')
const { permissionKey } = await load('@compallm/storybook')
const { MembersController } = await load('/src/settings/members/controller.ts')
const { RoleController } = await load('/src/settings/role/controller.ts')
const { GroupController } = await load('/src/settings/group/controller.ts')
const { PermissionController } = await load('/src/permission/controller.ts')
const { UserPreferenceController } = await load('/src/preference/user/controller.ts')
const { CreateProviderController } = await load('/src/provider/controller.ts')
const { AppController } = await load('/src/app/controller.ts')
const { ProfileController } = await load('/src/settings/profile/controller.ts')
const { PasskeyController } = await load('/src/passkey/controller.ts')
const { OnboardingController } = await load('/src/onboarding/controller.ts')
const { OrganizationOnboardingController } = await load('/src/settings/onboarding/controller.ts')
const { InvitationController } = await load('/src/auth/invitation-controller.ts')
const { AuthController } = await load('/src/auth/controller.ts')
const { UrlController } = await load('/src/url/controller.ts')
const { cedarWorker } = await load('/src/workers/cedar-client.ts')
const { clearNotices } = await load('/src/notifications/toast.ts')
const { userPreferenceDefaults } = await load('@compallm/contract/preference')
const releaseTransport = configureTransport(graphqlClient.request)
after(releaseTransport)
afterEach(() => {
  graphqlClient.resetScope()
  clearNotices()
})
const { buildSchema, getNamedType } = createRequire(new URL('../../libs/contract/package.json', import.meta.url))('graphql')
const schema = buildSchema(readFileSync(new URL('../../libs/contract/schema.graphql', import.meta.url), 'utf8'))
function typed(value, type) {
  if (value == null) return value
  if (Array.isArray(value)) return value.map((node) => typed(node, type))
  if (typeof value !== 'object') return value
  const fields = type.getFields()
  return {
    __typename: type.name,
    ...Object.fromEntries(
      Object.entries(value).map(([key, node]) => [key, fields[key] ? typed(node, getNamedType(fields[key].type)) : node]),
    ),
  }
}
const reply = (data) =>
  Response.json({
    data: Object.fromEntries(
      Object.entries(data).map(([key, value]) => {
        const field = schema.getQueryType().getFields()[key] ?? schema.getMutationType().getFields()[key]
        return [key, typed(value, getNamedType(field.type))]
      }),
    ),
  })
const connection = (nodes, next = false, cursor = null) => ({
  edges: nodes.map((node, index) => ({ cursor: String(index), node })),
  pageInfo: { endCursor: cursor, hasNextPage: next, startCursor: null, hasPreviousPage: false },
})
const user = {
  id: '1',
  userId: '2',
  organizationId: '10',
  organizationName: 'Research',
  avatarUrl: null,
  ipAddress: null,
  userAgent: null,
  signedInAt: '0',
  lastSeenAt: '0',
}
const role = { id: '3', name: 'Member', description: '', color: '', owner: false }
const member = (id) => ({
  id,
  displayName: id,
  email: `${id}@example.com`,
  available: true,
  joinedAt: 1,
  role,
  groups: [],
  effectiveRoleIds: ['3'],
  groupIds: [],
})
const tick = () => new Promise((resolve) => setImmediate(resolve))
async function until(check) {
  for (let i = 0; i < 200; i++) {
    if (check()) return
    await tick()
  }
  assert.ok(check(), 'controller did not settle')
}
function transport(t, send) {
  t.mock.method(globalThis, 'fetch', async (_input, init) => send(JSON.parse(init.body), init.signal))
}
function permissions(allowed) {
  const store = createStore(() => ({ entries: {} }))
  return {
    store,
    async evaluate(checks) {
      store.setState((state) => ({
        entries: {
          ...state.entries,
          ...Object.fromEntries(
            checks.map((check) => [permissionKey(check), { check, status: allowed(check) ? 'allowed' : 'denied' }]),
          ),
        },
      }))
    },
  }
}

test('member paging retains confirmed rows, ignores obsolete search responses, and stops after leaving', async (t) => {
  const requests = []
  transport(t, (operation, signal) => {
    if (operation.operationName === 'SettingsMembersInvitations') return reply({ invitations: connection([]) })
    return new Promise((resolve) => requests.push({ ...operation, signal, resolve }))
  })
  const store = createSettingsStore(),
    controller = new MembersController(store)
  const stop = controller.start()
  t.after(stop)
  await until(() => requests.length === 1)
  const resolve = (request, id, next = false) =>
    request.resolve(
      reply({
        me: { node: { id: '2', organizationOwner: true } },
        organization: { node: { ownerMemberIds: ['2'] } },
        members: connection([member(id)], next, `cursor-${id}`),
      }),
    )
  resolve(requests[0], 'first', true)
  await until(() => !store.getState().membersLoading)
  controller.loadMore()
  await until(() => requests.length === 2)
  assert.equal(requests[1].variables.membersAfter, 'cursor-first')
  controller.search('new')
  await until(() => requests.length === 3)
  assert.equal(requests[1].signal.aborted, true)
  assert.equal(store.getState().members[0].id, 'first')
  resolve(requests[2], 'new', true)
  await until(() => !store.getState().membersLoading)
  resolve(requests[1], 'obsolete')
  await tick()
  assert.deepEqual(
    store.getState().members.map((row) => row.id),
    ['new'],
  )
  controller.loadMore()
  await until(() => requests.length === 4)
  stop()
  assert.equal(requests[3].signal.aborted, true)
  resolve(requests[3], 'late')
  await tick()
  assert.deepEqual(
    store.getState().members.map((row) => row.id),
    ['new'],
  )
})

test('denied member permission sends no protected read or mutation', async (t) => {
  let calls = 0
  transport(t, () => {
    calls++
    throw new Error('unexpected request')
  })
  const store = createSettingsStore(),
    controller = new MembersController(
      store,
      permissions(() => false),
    )
  const stop = controller.start()
  t.after(stop)
  await until(() => !store.getState().membersLoading && !store.getState().invitationsLoading)
  assert.equal(store.getState().membersLoadError, true)
  assert.equal(await controller.invite('alex@example.com'), false)
  assert.equal(calls, 0)
})

test('member role changes reconcile only the confirmed row and retain the loaded cursor', async (t) => {
  const operations = []
  transport(t, (operation) => {
    operations.push(operation)
    if (operation.operationName === 'SettingsMembersInvitations') return reply({ invitations: connection([]) })
    if (operation.operationName === 'SettingsMembersRoleAssign')
      return reply({ roleAssign: { node: { ...role, id: '9', name: 'Operator' } } })
    return reply({
      me: { node: { id: '2', organizationOwner: true } },
      organization: { node: { ownerMemberIds: ['2'] } },
      members: connection([member('4'), member('5')], true, 'cursor-5'),
    })
  })
  const store = createSettingsStore(),
    controller = new MembersController(store)
  t.after(controller.start())
  await until(() => store.getState().membersLoaded)
  const other = store.getState().members[1]
  await controller.setMemberRole('4', '9')
  assert.equal(store.getState().members[0].role.id, '9')
  assert.equal(store.getState().members[1], other)
  assert.equal(store.getState().membersEndCursor, 'cursor-5')
  assert.equal(operations.filter((op) => op.operationName === 'SettingsMembersMeAndOrganizationAndMembers').length, 1)
})

test('role reads omit denied policy source and an unmounted editor cannot submit', async (t) => {
  const operations = []
  transport(t, (operation) => {
    operations.push(operation)
    if (operation.operationName === 'SettingsRoleRole') return reply({ role: { node: role } })
    if (operation.operationName === 'SettingsRoleOrganizationPolicyActions')
      return reply({ organizationPolicyActions: connection([]) })
    throw new Error('unexpected write')
  })
  const controller = new RoleController(
    createSettingsStore(),
    permissions((check) => check.action === 'role.read'),
  )
  t.after(controller.cancel)
  await controller.load('3')
  assert.equal(operations.find((op) => op.operationName === 'SettingsRoleRole').variables.includeRoleNodeCedarSource, false)
  assert.equal(controller.store.getState().draft.id, '3')
  controller.cancel()
  await assert.rejects(controller.updateMetadata({ name: 'Changed' }))
  assert.equal(operations.length, 2)
})

test('SSR role hydration parses the prepared policy without treating it as a draft edit or fetching again', async (t) => {
  transport(t, () => {
    throw new Error('hydration must reuse server data')
  })
  t.mock.method(cedarWorker, 'request', async (operation) => {
    assert.equal(operation, 'projectPolicy')
    return { projection: [{ action: 'model.read', effect: 'permit', editable: true, source: 'policy' }], diagnostics: [] }
  })
  const controller = new RoleController(createSettingsStore())
  t.after(controller.cancel)
  controller.seed(
    { role: { node: { ...role, cedarSource: 'policy', canReadPolicy: true } } },
    {
      organizationPolicyActions: connection([
        { name: 'model.read', displayName: 'Read models', description: '', entityType: 'Model', resourceType: 'Organization' },
      ]),
    },
  )
  assert.equal(controller.store.form.dirty, false)
  await controller.load(role.id)
  assert.equal(controller.store.getState().draft.mode, 'visual')
  assert.equal(controller.store.getState().draft.parsing, false)
  assert.equal(controller.store.getState().draft.projection[0].action, 'model.read')
  assert.equal(controller.store.form.dirty, false)
})

test('SSR authority supports contextual checks before the workspace mount and still denies forbidden actions', async (t) => {
  transport(t, () => {
    throw new Error('authority must reuse server data')
  })
  t.mock.method(cedarWorker, 'request', async (operation, args) => {
    if (operation === 'clearPermissions') return true
    assert.equal(args[0].userId, user.userId)
    return args[1].map((check) => (check.action === 'role.policy.read' ? 'allowed' : 'denied'))
  })
  const controller = new PermissionController()
  t.after(controller.stop)
  controller.seed(user, {
    id: user.userId,
    organizationId: user.organizationId,
    cedarSource: 'policy',
    cedarEntities: '[]',
    organizationPermissions: [],
  })
  const read = { action: 'role.policy.read', resourceType: 'Role', resourceId: role.id }
  const write = { ...read, action: 'role.policy.update' }
  await controller.evaluate([read, write])
  controller.start(user)
  assert.equal(controller.store.getState().entries[permissionKey(read)].status, 'allowed')
  assert.equal(controller.store.getState().entries[permissionKey(write)].status, 'denied')
})

test('group metadata remains available without relationship permissions', async (t) => {
  const operations = []
  transport(t, (operation) => {
    operations.push(operation)
    return reply({ group: { node: { id: '4', name: 'Research', description: '' } } })
  })
  const controller = new GroupController(
    createSettingsStore(),
    permissions((check) => check.action === 'group.read'),
  )
  t.after(controller.cancel)
  await controller.load('4')
  assert.equal(controller.store.getState().draft.name, 'Research')
  assert.deepEqual(controller.store.getState().members, [])
  assert.equal(operations[0].variables.includeGroupNode, false)
  assert.equal(operations[0].variables.includeGroupNodeMemberIds, false)
  controller.cancel()
  assert.equal(
    await controller.updateGroup(controller.store.getState().draft, ['draft.name'], new AbortController().signal),
    false,
  )
  assert.equal(operations.length, 1)
})

test('permission results settle against the latest member facts and never publish after scope disposal', async (t) => {
  transport(t, () => reply({ me: { node: { id: '2', organizationId: '10', cedarSource: 'policy', cedarEntities: '[]' } } }))
  let release
  t.mock.method(cedarWorker, 'request', async (operation, args) => {
    if (operation === 'clearPermissions') return true
    if (!args[1].length) return []
    if (args[2][0]?.roleIds.includes('new')) return ['allowed']
    return new Promise((resolve) => {
      release = resolve
    })
  })
  const controller = new PermissionController(),
    check = { action: 'member.role.assign', resourceType: 'Member', resourceId: '4', roleId: '9' }
  controller.start()
  t.after(controller.stop)
  controller.setFacts([{ resourceType: 'Member', id: '4', roleIds: ['old'], groupIds: [] }])
  const pending = controller.evaluate([check])
  await until(() => !!release)
  controller.setFacts([{ resourceType: 'Member', id: '4', roleIds: ['new'], groupIds: [] }])
  release(['denied'])
  await pending
  assert.equal(controller.store.getState().entries[permissionKey(check)].status, 'allowed')
  release = undefined
  controller.setFacts([{ resourceType: 'Member', id: '4', roleIds: ['old'], groupIds: [] }])
  const obsolete = controller.evaluate([check])
  await until(() => !!release)
  controller.stop()
  release(['allowed'])
  await obsolete
  assert.deepEqual(controller.store.getState().entries, {})
})

test('preferences hydrate without writing and commit only the changed response category', async (t) => {
  const operations = []
  transport(t, (operation) => {
    operations.push(operation)
    if (operation.operationName === 'PreferenceUserUserPreferences')
      return reply({ userPreferences: { node: structuredClone(userPreferenceDefaults) } })
    return reply({ userPreferencesUpdate: { node: { navigation: { home: 'MODELS' } } } })
  })
  const controller = new UserPreferenceController()
  const auth = { store: createStore(() => ({ currentUser: user, status: 'authenticated' })) }
  t.after(controller.start(auth))
  await until(() => controller.store.getState().status === 'ready')
  assert.equal(operations.length, 1)
  const typography = controller.store.getState().saved.typography
  await controller.update({ home: 'MODELS' }, new AbortController().signal)
  assert.equal(controller.store.getState().saved.navigation.home, 'MODELS')
  assert.equal(controller.store.getState().saved.typography, typography)
  assert.deepEqual(operations[1].variables.userPreferencesUpdateInput, { navigation: { home: 'MODELS' } })
  assert.equal(operations[1].variables.includeNavigation, true)
  assert.equal(operations[1].variables.includeTypography, false)
})

test('invitation retries reconcile an uncertain confirmation without replaying the write or accepting another identity', async (t) => {
  const originalWindow = globalThis.window
  globalThis.window = { location: new URL('http://localhost/invitation?step=code') }
  t.after(() => {
    globalThis.window = originalWindow
  })
  let confirmations = 0,
    email = 'other@example.com'
  transport(t, (operation) => {
    if (operation.operationName === 'AuthInvitationControllerInvitationContext')
      return reply({
        invitationContext: {
          node: { organizationName: 'Research', email: 'alex@example.com', signup: false, expiresAt: 2000000000 },
        },
      })
    if (operation.operationName === 'AuthInvitationControllerEmailSigninConfirm') {
      confirmations++
      throw new TypeError('lost response')
    }
    if (operation.operationName === 'AuthSession2') return reply({ session: { node: user } })
    if (operation.operationName === 'AuthUserProfile') return reply({ userProfile: { node: { email } } })
    throw new Error('unexpected operation')
  })
  const url = new UrlController(),
    destinations = []
  url.synchronize(globalThis.window.location)
  url.connect(async (href) => {
    destinations.push(href)
    url.synchronize(new URL(href, 'http://localhost'))
  })
  const auth = new AuthController(url, graphqlClient),
    controller = new InvitationController(auth, url)
  t.after(() => auth.dispose())
  t.after(controller.start())
  await until(() => !controller.store.getState().loading)
  assert.equal(await controller.confirm('123456'), false)
  assert.equal(await controller.confirm('123456'), false)
  assert.equal(auth.store.getState().currentUser, null)
  email = 'alex@example.com'
  assert.equal(await controller.confirm('123456'), true)
  assert.equal(confirmations, 1)
  assert.equal(auth.store.getState().currentUser.id, user.id)
  assert.equal(destinations.at(-1), '/security')
})

test('management email verification preserves identity and does not replay an uncertain proof', async (t) => {
  let proofs = 0
  transport(t, (operation) => {
    if (operation.operationName === 'AuthSession2') return reply({ session: { node: user } })
    if (operation.operationName === 'AuthUserProfile') return reply({ userProfile: { node: { email: 'alex@example.com' } } })
    if (operation.operationName === 'AuthEmailSigninRequest') {
      assert.equal(operation.variables.emailSigninRequestInput.purpose, 'PASSKEY_CHANGE')
      return reply({ emailSigninRequest: { node: { lifetime: { expiresInSeconds: '600', throttleSeconds: '0' } } } })
    }
    if (operation.operationName === 'AuthEmailSigninConfirm') {
      proofs++
      throw new TypeError('lost response')
    }
    throw new Error('unexpected operation')
  })
  const url = {
    store: createStore(() => ({ pathname: '/Research/settings/security', search: '', hash: '' })),
    navigate() {
      throw new Error('unexpected navigation')
    },
  }
  const auth = new AuthController(url, graphqlClient)
  t.after(auth.dispose)
  await auth.refreshCurrentUser()
  const verification = auth.requestVerification()
  await auth.useEmailCode()
  await auth.requestCode('')
  await auth.verifyCode('123456')
  await auth.retry()
  assert.equal(proofs, 1)
  assert.equal(auth.store.getState().currentUser.id, user.id)
  auth.cancelVerification()
  assert.equal(await verification, false)
})

test('provider creation publishes its confirmed resource and ignores completion after disposal', async (t) => {
  const origin = 'http://localhost',
    originalWindow = globalThis.window
  globalThis.window = { location: new URL(origin + '/Research/providers/new/configure-provider?provider=openai') }
  t.after(() => {
    globalThis.window = originalWindow
  })
  let release,
    deferred = false
  const writes = []
  const provider = {
    id: 'openai',
    name: 'OpenAI',
    baseUrl: 'https://api.openai.com/v1',
    logoUrl: null,
    description: null,
    connection: {
      baseUrl: 'https://api.openai.com/v1',
      path: null,
      timeoutMs: null,
      query: [],
      authentication: { method: 'NONE', configuration: null },
      endpoint: { mode: 'CUSTOM', configuration: null },
    },
    headers: [],
    authentication: { method: 'NONE', configuration: null },
    models: connection([]),
  }
  transport(t, (operation) => {
    if (operation.operationName === 'ProviderCatalogEffectiveProviderCatalog')
      return reply({ effectiveProviderCatalog: { node: provider } })
    if (operation.operationName === 'ProviderProviderCreate') {
      writes.push(operation)
      const response = reply({ providerCreate: { node: { id: '21', name: 'Research provider', logoUrl: null } } })
      return deferred
        ? new Promise((resolve) => {
            release = () => resolve(response)
          })
        : response
    }
    throw new Error('unexpected operation')
  })
  const url = new UrlController()
  url.synchronize(globalThis.window.location)
  const created = []
  const controller = new CreateProviderController(
    url,
    { getCurrentOrganization: () => ({ id: '10', name: 'Research' }), onProviderCreated: (value) => created.push(value) },
    new AppController(0),
  )
  t.after(controller.start())
  await until(() => controller.store.getState().providerLoaded)
  controller.store.setState({ models: [{ id: 'model-1', enabled: true, protocol: 'openai_chat_completions' }] })
  const values = { name: 'Research provider', apiKey: 'synthetic', baseUrl: provider.baseUrl }
  assert.deepEqual(await controller.onCreateProvider(values), { apiKey: '' })
  assert.equal(created.length, 1)
  assert.equal(writes[0].variables.providerCreateInput.providerCatalogId, 'openai')
  deferred = true
  const pending = controller.onCreateProvider(values)
  await until(() => !!release)
  controller.stop()
  release()
  assert.equal(await pending, false)
  assert.equal(created.length, 1)
  assert.equal(await controller.onCreateProvider(values), false)
  assert.equal(writes.length, 2)
})

test('profile saves return authoritative values and ignore responses for a disposed identity', async (t) => {
  const profile = { id: user.userId, displayName: 'Alex', username: 'alex', email: 'alex@example.com', avatarUrl: null }
  let release
  transport(t, (operation) => {
    if (operation.operationName === 'SettingsProfileUserProfile') return reply({ userProfile: { node: profile } })
    if (operation.operationName === 'SettingsProfileUserProfileUpdate')
      return new Promise((resolve) => {
        release = () =>
          resolve(reply({ userProfileUpdate: { node: { ...profile, displayName: 'Confirmed', avatarUrl: '/new-avatar' } } }))
      })
    throw new Error('unexpected operation')
  })
  const store = createSettingsStore(),
    auth = { store: createStore(() => ({ currentUser: user, status: 'authenticated' })) }
  const controller = new ProfileController(store, auth)
  const stop = controller.start()
  t.after(stop)
  await until(() => store.getState().profileLoaded)
  assert.equal(store.form.get('profile.name'), 'Alex')
  const pending = controller.saveProfile({ name: 'Changed' }, null, new AbortController().signal)
  await until(() => !!release)
  stop()
  release()
  assert.equal(await pending, false)
  assert.equal(auth.store.getState().currentUser.avatarUrl, null)
})

test('passkey removal reconciles a lost response from a fresh list before any retry', async (t) => {
  const passkey = { id: '31', credentialId: 'synthetic-credential', name: 'Laptop', createdAt: '0', lastUsedAt: null }
  let removals = 0
  transport(t, (operation) => {
    if (operation.operationName === 'PasskeyPasskeysAndSession')
      return reply({ passkeys: connection([passkey]), session: { node: { passkey } } })
    if (operation.operationName === 'PasskeyPasskeyRemove') {
      removals++
      throw new TypeError('lost response')
    }
    if (operation.operationName === 'PasskeyPasskeys2') return reply({ passkeys: connection([]) })
    throw new Error('unexpected operation')
  })
  const auth = { store: createStore(() => ({ currentUser: user, status: 'authenticated' })) }
  const controller = new PasskeyController(auth)
  t.after(controller.start())
  await controller.load()
  controller.openRemove('31')
  assert.equal(await controller.remove('31'), false)
  assert.equal(await controller.remove('31'), true)
  assert.equal(removals, 1)
  assert.deepEqual(controller.store.getState().records, [])
  auth.store.setState({ currentUser: null, status: 'anonymous' })
  assert.equal(controller.store.getState().loaded, false)
})

test('completed onboarding reads do not create a presentation or advance the flow', async (t) => {
  const operations = []
  transport(t, (operation) => {
    operations.push(operation.operationName)
    if (operation.operationName !== 'OnboardingOnboardingContextAndUserProfileAndAnalyticUserProfile')
      throw new Error('unexpected write')
    return reply({
      onboardingContext: {
        node: {
          required: false,
          hasBeenPresented: true,
          flow: 'PERSONAL',
          step: 'READY',
          settings: {
            welcome: '',
            links: [],
            destination: 'CHAT',
            artworkColor: null,
            requireFullName: false,
            requireAvatar: false,
          },
        },
      },
      userProfile: null,
      analyticUserProfile: null,
    })
  })
  const controller = new OnboardingController(
    { store: createStore(() => ({ currentUser: user })) },
    {
      navigate() {
        throw new Error('unexpected navigation')
      },
    },
  )
  t.after(controller.start())
  await until(() => controller.store.getState().completed)
  assert.equal(controller.store.getState().visible, false)
  assert.equal(operations.length, 1)
})

test('organization onboarding loads and reconciles GraphQL resources into editable form values', async (t) => {
  const saved = {
    welcome: 'Welcome',
    links: [{ label: 'Handbook', href: 'https://example.com/handbook' }],
    destination: 'CHAT',
    artworkColor: null,
    requireFullName: false,
    requireAvatar: false,
  }
  let writes = 0
  transport(t, (operation) => {
    if (operation.operationName === 'SettingsOnboardingOrganizationOnboardingSettings')
      return reply({ organizationOnboardingSettings: { node: saved } })
    writes++
    assert.deepEqual(operation.variables.organizationOnboardingSettingsUpdateInput.links, saved.links)
    return reply({ organizationOnboardingSettingsUpdate: { node: { ...saved, welcome: 'Updated welcome' } } })
  })
  const controller = new OrganizationOnboardingController()
  t.after(controller.cancel)
  await controller.load()
  assert.equal(controller.store.getState().status, 'ready')
  assert.equal(writes, 0)
  assert.deepEqual(controller.store.getState().saved, saved)
  const result = await controller.update({ welcome: 'Updated welcome' }, new AbortController().signal)
  assert.equal(result.welcome, 'Updated welcome')
  assert.equal(controller.store.getState().saved.welcome, result.welcome)
  assert.equal(writes, 1)
})

test('network grouping preserves remaining callers and expires successful reuse after 100 ms', async () => {
  const { NetworkService } = await load('/src/network/service.ts')
  let now = 0
  let calls = 0
  let resolve
  const service = new NetworkService(
    async () => {
      calls++
      if (calls === 1)
        await new Promise((done) => {
          resolve = done
        })
      return Response.json({ data: { count: calls } })
    },
    () => now,
  )
  const request = (id) => ({
    id,
    epoch: 0,
    revision: 0,
    operation: { name: 'Directory', text: 'query Directory { count }', variables: {}, kind: 'query', reusable: true },
  })
  const first = service.execute(request('first'))
  const cancelled = assert.rejects(first, { name: 'AbortError' })
  const second = service.execute(request('second'))
  await new Promise((done) => setImmediate(done))
  assert.equal(calls, 1)
  service.cancel('first')
  resolve()
  await cancelled
  assert.equal((await second).data.count, 1)
  now = 99
  assert.equal((await service.execute(request('reused'))).data.count, 1)
  now = 100
  assert.equal((await service.execute(request('fresh'))).data.count, 2)
})

test('an uncertain write invalidates prior query reuse without replaying the mutation', async () => {
  const { NetworkService } = await load('/src/network/service.ts')
  let reads = 0
  let writes = 0
  const service = new NetworkService(async (_url, init) => {
    const operation = JSON.parse(init.body)
    if (operation.operationName === 'Change') {
      writes++
      throw new TypeError('Response lost')
    }
    return Response.json({ data: { count: ++reads } })
  })
  const read = (id) => ({
    id,
    epoch: 0,
    revision: 0,
    operation: { name: 'Directory', text: 'query Directory { count }', variables: {}, kind: 'query', reusable: true },
  })
  await service.execute(read('before'))
  await assert.rejects(
    service.execute({
      id: 'write',
      epoch: 0,
      revision: 0,
      operation: { name: 'Change', text: 'mutation Change { change }', variables: {}, kind: 'mutation', reusable: false },
    }),
  )
  assert.equal((await service.execute(read('after'))).data.count, 2)
  assert.equal(writes, 1)
})
