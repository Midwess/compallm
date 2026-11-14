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
const { OrganizationController } = await server.ssrLoadModule('/src/settings/organization/controller.ts')
const { SettingsController } = await server.ssrLoadModule('/src/settings/controller.ts')
const { createSettingsStore } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/store.ts')
const { createAuthStore } = await server.ssrLoadModule('../libs/storybook/src/pages/auth/store.ts')
const { configureSettingsForm } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/form.ts')
const graph = (field, node) => Response.json({ data: { [field]: { node } } })
const node = (name = 'Original', avatarUrl = null, id = '84', canUpdateProfile = true) => ({
  id,
  name,
  avatarUrl,
  canUpdateProfile,
})
const deferred = () => {
  let resolve
  const promise = new Promise((done) => {
    resolve = done
  })
  return { promise, resolve }
}
async function until(predicate) {
  for (let i = 0; i < 200 && !predicate(); i++) await new Promise((resolve) => setTimeout(resolve, 1))
  assert.ok(predicate())
}
function auth() {
  const store = createAuthStore()
  store.setState({
    status: 'authenticated',
    currentUser: { id: 'session', userId: '1', organizationId: '84', organizationName: 'Original' },
  })
  return { store }
}
function setup() {
  const store = createSettingsStore({ location: { page: 'organization' } })
  const identity = auth()
  const renamed = []
  const errors = []
  const organization = new OrganizationController(store, identity, (name) => renamed.push(name))
  const stopForm = configureSettingsForm({
    store,
    pages: { organization },
    apiError: (error, retry) => errors.push({ error, retry }),
  })
  const stop = organization.start()
  return {
    store,
    identity,
    organization,
    renamed,
    errors,
    stop: () => {
      stop()
      stopForm()
    },
  }
}

test('organization name saves on blur and canonical response preserves newer typing', async (t) => {
  const pending = deferred()
  const writes = []
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const operation = JSON.parse(init.body)
    if (operation.query.includes('organizationProfileUpdate')) {
      writes.push(Object.values(operation.variables)[0])
      return pending.promise
    }
    return graph('organizationProfile', node())
  })
  const fixture = setup()
  t.after(fixture.stop)
  await until(() => fixture.store.getState().organizationProfileStatus === 'ready')
  fixture.store.form.change('organization.name', ' New-Team ')
  assert.equal(writes.length, 0)
  fixture.store.form.blur('organization.name')
  await until(() => writes.length === 1)
  assert.deepEqual(writes[0], { name: 'New-Team', avatarUploaded: false })
  fixture.store.form.change('organization.name', 'Newer-Draft')
  pending.resolve(graph('organizationProfileUpdate', node('New-Team')))
  await until(() => !fixture.store.form.pending)
  assert.equal(fixture.store.getState().organization.name, 'Newer-Draft')
  assert.equal(fixture.store.form.baseline.organization.name, 'New-Team')
  assert.deepEqual(fixture.renamed, ['New-Team'])
})

test('organization upload retries confirmation without reuploading or saving an unblurred name', async (t) => {
  let uploads = 0
  let confirmations = 0
  const inputs = []
  t.mock.method(globalThis, 'fetch', async (url, init) => {
    if (url === 'https://storage.example/assets') {
      uploads++
      assert.equal(init.credentials, 'omit')
      assert.deepEqual([...init.body.keys()], ['key', 'policy', 'file'])
      return new Response(null, { status: 204 })
    }
    const operation = JSON.parse(init.body)
    if (operation.query.includes('generateOrgAvatarUrl'))
      return graph('generateOrgAvatarUrl', {
        url: 'https://storage.example/assets',
        fields: [
          { name: 'key', value: 'avatars/84' },
          { name: 'policy', value: 'signed' },
        ],
        expiresAt: '2000000000',
      })
    if (operation.query.includes('organizationProfileUpdate')) {
      confirmations++
      inputs.push(Object.values(operation.variables)[0])
      if (confirmations === 1) return Response.json({ errors: [{ message: 'Unavailable', extensions: { status: 503 } }] })
      return graph('organizationProfileUpdate', node('Original', 'https://cdn/avatars/84?v=2'))
    }
    return graph('organizationProfile', node())
  })
  const fixture = setup()
  t.after(fixture.stop)
  await until(() => fixture.store.getState().organizationProfileStatus === 'ready')
  fixture.store.form.change('organization.name', 'Unblurred')
  const file = new File(['image'], 'logo.png', { type: 'image/png' })
  fixture.store.form.change('organizationUpload', file)
  await until(() => !fixture.store.form.pending)
  assert.equal(fixture.errors.length, 1)
  assert.equal(fixture.store.getState().organizationUpload, file)
  await fixture.store.form.retry()
  assert.equal(uploads, 1)
  assert.equal(confirmations, 2)
  assert.deepEqual(inputs, [{ avatarUploaded: true }, { avatarUploaded: true }])
  assert.equal(fixture.store.getState().organization.name, 'Unblurred')
  assert.equal(fixture.store.getState().organization.logo, 'https://cdn/avatars/84?v=2')
})

test('members without update authority cannot submit and stale organization reads are discarded', async (t) => {
  const first = deferred()
  let reads = 0
  let writes = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    if (JSON.parse(init.body).query.includes('mutation')) {
      writes++
      throw new Error('unauthorized write')
    }
    reads++
    return reads === 1 ? first.promise : graph('organizationProfile', node('Other', null, '99', false))
  })
  const fixture = setup()
  t.after(fixture.stop)
  fixture.identity.store.setState({ currentUser: { id: 'next', userId: '1', organizationId: '99', organizationName: 'Other' } })
  await until(() => fixture.store.getState().organizationProfileStatus === 'ready')
  first.resolve(graph('organizationProfile', node('Late')))
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(fixture.store.getState().organization.name, 'Other')
  assert.equal(fixture.store.getState().organizationCanUpdateProfile, false)
  assert.equal(await fixture.organization.saveProfile({ name: 'Denied' }, null, new AbortController().signal), false)
  assert.equal(writes, 0)
})

test('rename replaces the current route and updates future settings navigation', async (t) => {
  const identity = auth()
  const navigations = []
  const url = {
    store: { getState: () => ({ pathname: '/Original/settings/organization', search: '?view=identity', hash: '#avatar' }) },
    navigate: (...args) => navigations.push(args),
  }
  let persistedName = 'Original'
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const operation = JSON.parse(init.body)
    if (operation.query.includes('roles')) return Response.json({ data: { roles: { edges: [] }, groups: { edges: [] } } })
    if (operation.query.includes('organizationProfileUpdate')) {
      persistedName = 'New-Team'
      return graph('organizationProfileUpdate', node(persistedName))
    }
    return graph('organizationProfile', node(persistedName))
  })
  const settings = new SettingsController('Original', url, { ...identity, signOut() {} }, 'organization', {})
  const stopForm = configureSettingsForm(settings)
  const stop = settings.start()
  t.after(() => {
    stop()
    stopForm()
  })
  await until(() => settings.store.getState().organizationProfileStatus === 'ready')
  settings.store.form.change('organization.name', 'New-Team')
  settings.store.form.blur('organization.name')
  await until(() => !settings.store.form.pending)
  assert.equal(identity.store.getState().currentUser.organizationName, 'New-Team')
  assert.deepEqual(navigations[0], ['/New-Team/settings/organization?view=identity#avatar', { replace: true }])
  settings.navigate('members')
  assert.equal(navigations[1][0], '/New-Team/settings/members')
})
