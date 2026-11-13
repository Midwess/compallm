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
const { ProfileController } = await server.ssrLoadModule('/src/settings/profile/controller.ts')
const { createSettingsStore } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/store.ts')
const { createAuthStore: createSignupStore } = await server.ssrLoadModule('../libs/storybook/src/pages/auth/store.ts')
const { configureSettingsForm } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/form.ts')

const node = (displayName = 'Saved Name', avatarUrl = null, id = '1') => ({
  id,
  displayName,
  avatarUrl,
  username: 'saved-user',
  email: 'me@example.com',
})
const graph = (field, value) => Response.json({ data: { [field]: value } })
const failure = () => Response.json({ errors: [{ message: 'Storage unavailable', extensions: { status: 503 } }] })
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
function setup() {
  const store = createSettingsStore({ location: { page: 'profile' } })
  const auth = { store: createSignupStore() }
  auth.store.setState({
    status: 'authenticated',
    currentUser: { id: 'session', userId: '1', organizationId: 'org', organizationName: 'Workspace' },
  })
  const profile = new ProfileController(store, auth)
  const errors = []
  const settings = { store, pages: { profile }, apiError: (error, retry) => errors.push({ error, retry }) }
  const stopForm = configureSettingsForm(settings)
  const stop = profile.start()
  return {
    store,
    auth,
    profile,
    errors,
    stop: () => {
      stop()
      stopForm()
    },
  }
}

test('username saves on blur without committing an unblurred full name and email stays read-only', async (t) => {
  const writes = []
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const operation = JSON.parse(init.body)
    if (operation.query.includes('userProfileUpdate')) {
      writes.push(Object.values(operation.variables)[0])
      return graph('userProfileUpdate', { node: { ...node(), username: 'updated-user' } })
    }
    return graph('userProfile', { node: node() })
  })
  const fixture = setup()
  t.after(fixture.stop)
  await until(() => fixture.store.getState().profileStatus === 'ready')
  assert.equal(fixture.store.getState().profile.username, 'saved-user')
  assert.equal(fixture.store.getState().profile.email, 'me@example.com')
  fixture.store.form.change('profile.name', 'Unblurred full name')
  fixture.store.form.change('profile.username', '  Updated-User  ')
  assert.equal(writes.length, 0)
  fixture.store.form.blur('profile.username')
  await until(() => !fixture.store.form.pending)
  assert.deepEqual(writes, [{ username: 'updated-user', avatarUploaded: false }])
  assert.equal(fixture.store.getState().profile.name, 'Unblurred full name')
  assert.equal(fixture.store.getState().profile.username, 'updated-user')
  assert.equal(fixture.store.getState().profile.email, 'me@example.com')
})

test('loads persisted profile, saves on blur, and preserves newer typing during confirmation', async (t) => {
  const write = deferred()
  const operations = []
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const operation = JSON.parse(init.body)
    operations.push(operation)
    if (operation.query.includes('userProfileUpdate')) return write.promise
    return graph('userProfile', { node: node() })
  })
  const fixture = setup()
  t.after(fixture.stop)
  await until(() => fixture.store.getState().profileStatus === 'ready')
  assert.equal(fixture.store.getState().profile.name, 'Saved Name')
  fixture.store.form.change('profile.name', '  Submitted  ')
  assert.equal(operations.length, 1)
  fixture.store.form.blur('profile.name')
  await until(() => operations.length === 2)
  assert.deepEqual(Object.values(operations[1].variables)[0], { displayName: 'Submitted', avatarUploaded: false })
  fixture.store.form.change('profile.name', 'Newer typing')
  write.resolve(graph('userProfileUpdate', { node: node('Submitted') }))
  await until(() => !fixture.store.form.pending)
  assert.equal(fixture.store.getState().profile.name, 'Newer typing')
  assert.equal(fixture.store.form.baseline.profile.name, 'Submitted')
  assert.equal(fixture.store.form.dirty, true)
})

test('uploads directly before confirmation and retries failed confirmation without reuploading', async (t) => {
  const calls = []
  let failed = false
  t.mock.method(globalThis, 'fetch', async (url, init) => {
    calls.push({ url, init })
    if (url === 'https://storage.example.com/assets') return new Response(null, { status: 204 })
    const { query } = JSON.parse(init.body)
    if (query.includes('generateUrlAvatarUrl'))
      return graph('generateUrlAvatarUrl', {
        node: {
          url: 'https://storage.example.com/assets',
          fields: [
            { name: 'key', value: 'avatars/1' },
            { name: 'policy', value: 'signed' },
          ],
          expiresAt: '2000000000',
        },
      })
    if (query.includes('userProfileUpdate')) {
      if (JSON.parse(init.body).variables && Object.values(JSON.parse(init.body).variables)[0].avatarUploaded && !failed) {
        failed = true
        return failure()
      }
      return graph('userProfileUpdate', { node: node('Updated', 'https://cdn.example.com/avatars/1?v=2') })
    }
    return graph('userProfile', { node: node() })
  })
  const fixture = setup()
  t.after(fixture.stop)
  await until(() => fixture.store.getState().profileStatus === 'ready')
  const file = new File(['image bytes'], 'photo.png', { type: 'image/png' })
  fixture.store.form.change('profile.name', 'Updated')
  fixture.store.form.blur('profile.name')
  await until(() => !fixture.store.form.pending)
  fixture.store.form.change('profile.name', 'Unblurred draft')
  fixture.store.form.change('profileUpload', file)
  await until(() => !fixture.store.form.pending)
  assert.equal(fixture.errors.length, 1)
  assert.equal(fixture.store.getState().profileUpload, file)
  assert.equal(fixture.store.getState().profile.name, 'Unblurred draft')
  const confirmation = calls
    .filter(({ url, init }) => url === '/api/graphql' && JSON.parse(init.body).query.includes('userProfileUpdate'))
    .at(-1)
  assert.deepEqual(Object.values(JSON.parse(confirmation.init.body).variables)[0], { avatarUploaded: true })
  const direct = calls.find(({ url }) => url === 'https://storage.example.com/assets')
  assert.equal(direct.init.credentials, 'omit')
  assert.equal(direct.init.headers, undefined)
  assert.deepEqual([...direct.init.body.keys()], ['key', 'policy', 'file'])
  await fixture.store.form.retry()
  assert.equal(calls.filter(({ url }) => url === 'https://storage.example.com/assets').length, 1)
  assert.equal(fixture.store.getState().profileUpload, null)
  assert.equal(fixture.store.getState().profile.avatar, 'https://cdn.example.com/avatars/1?v=2')
  assert.equal(fixture.auth.store.getState().currentUser.avatarUrl, 'https://cdn.example.com/avatars/1?v=2')
  assert.equal(fixture.store.form.dirty, true)
})

test('failed direct upload never confirms the profile and retains its draft', async (t) => {
  let confirmations = 0
  t.mock.method(globalThis, 'fetch', async (url, init) => {
    if (url === 'https://storage.example.com/assets') return new Response(null, { status: 403 })
    const { query } = JSON.parse(init.body)
    if (query.includes('userProfileUpdate')) {
      confirmations++
      throw new Error('unexpected confirmation')
    }
    if (query.includes('generateUrlAvatarUrl'))
      return graph('generateUrlAvatarUrl', {
        node: {
          url: 'https://storage.example.com/assets',
          fields: [],
          expiresAt: '2000000000',
        },
      })
    return graph('userProfile', { node: node() })
  })
  const fixture = setup()
  t.after(fixture.stop)
  await until(() => fixture.store.getState().profileStatus === 'ready')
  const file = new File(['image'], 'photo.png', { type: 'image/png' })
  fixture.store.form.change('profileUpload', file)
  await until(() => !fixture.store.form.pending)
  assert.equal(confirmations, 0)
  assert.equal(fixture.store.getState().profileUpload, file)
  assert.equal(fixture.errors.length, 1)
})

test('account changes discard stale reads and aborted writes cannot update current state', async (t) => {
  const oldRead = deferred()
  const write = deferred()
  let requests = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('userProfileUpdate')) return write.promise
    requests++
    return requests === 1 ? oldRead.promise : graph('userProfile', { node: node('Second User', null, '2') })
  })
  const fixture = setup()
  t.after(fixture.stop)
  fixture.auth.store.setState({
    currentUser: { id: 'second', userId: '2', organizationId: 'org', organizationName: 'Workspace' },
  })
  await until(() => fixture.store.getState().profileStatus === 'ready')
  oldRead.resolve(graph('userProfile', { node: node('Old') }))
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(fixture.store.getState().profile.name, 'Second User')
  fixture.store.form.change('profile.name', 'Attempt')
  fixture.store.form.blur('profile.name')
  await until(() => fixture.store.getState().profileSaving)
  fixture.stop()
  write.resolve(graph('userProfileUpdate', { node: node('Late', 'https://cdn/late', '2') }))
  await new Promise((resolve) => setTimeout(resolve, 0))
  assert.equal(fixture.store.getState().profile.name, 'Attempt')
  assert.equal(fixture.auth.store.getState().currentUser.avatarUrl, undefined)
})
