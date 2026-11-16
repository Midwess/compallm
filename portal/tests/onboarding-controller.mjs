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
const { OnboardingController } = await server.ssrLoadModule('/src/onboarding/controller.ts')
const { OrganizationOnboardingController } = await server.ssrLoadModule('/src/settings/onboarding/controller.ts')
const { createAuthStore } = await server.ssrLoadModule('../libs/storybook/src/pages/auth/store.ts')
const context = (patch = {}) => ({
  required: true,
  hasBeenPresented: false,
  shouldPresent: true,
  flow: 'PERSONAL',
  step: 'PROFILE',
  settings: {
    welcome: 'Welcome',
    links: [],
    destination: 'CHAT',
    artworkColor: null,
    requireFullName: false,
    requireAvatar: false,
  },
  ...patch,
})
const profile = { username: 'alex', displayName: '', email: 'alex@example.com', avatarUrl: null }
const response = (data) => Response.json({ data })
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
  const auth = { store: createAuthStore() }
  auth.store.setState({
    status: 'authenticated',
    currentUser: { id: 'session', userId: '1', organizationId: 'org', organizationName: "alex's organization" },
  })
  const paths = []
  const url = {
    store: { getState: () => ({ pathname: '/alex/chat', search: '', hash: '' }) },
    navigate: (path) => paths.push(path),
  }
  const controller = new OnboardingController(auth, url)
  return { controller, paths, stop: controller.start() }
}
test('dismissal retains pending state and reload respects the session reminder', async (t) => {
  let presented = false
  let presents = 0
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('onboardingPresent')) {
      presented = true
      presents++
      return response({ onboardingPresent: { node: context({ shouldPresent: false }) } })
    }
    return response({
      onboardingContext: { node: context({ shouldPresent: !presented }) },
      userProfile: { node: profile },
      analyticUserProfile: null,
    })
  })
  const f = setup()
  t.after(f.stop)
  await until(() => f.controller.store.getState().visible)
  assert.equal(f.controller.store.getState().firstPresentation, true)
  f.controller.dismiss()
  assert.equal(f.controller.store.getState().visible, false)
  await f.controller.load(false)
  assert.equal(f.controller.store.getState().visible, false)
  assert.equal(presents, 1)
  f.controller.reopen()
  await until(() => f.controller.store.getState().visible)
  assert.equal(presents, 2)
  assert.equal(f.controller.store.getState().firstPresentation, false)
})
test('unchanged generated workspace name and skipped work send no profile or analytics write', async (t) => {
  const writes = []
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('onboardingPresent')) return response({ onboardingPresent: { node: context() } })
    if (query.includes('onboardingAdvance')) {
      writes.push('advance')
      return response({ onboardingAdvance: { node: context({ step: 'PREFERENCES' }) } })
    }
    if (query.includes('onboardingContext'))
      return response({ onboardingContext: { node: context() }, userProfile: { node: profile }, analyticUserProfile: null })
    throw new Error(`Unexpected write ${query}`)
  })
  const f = setup()
  t.after(f.stop)
  await until(() => f.controller.store.getState().visible)
  assert.equal(await f.controller.saveWorkspaceName("alex's organization", new AbortController().signal), true)
  f.controller.skip()
  await until(() => f.controller.store.getState().step === 'preferences')
  assert.deepEqual(writes, ['advance'])
})
test('cancelled onboarding reads never show an obsolete popup', async (t) => {
  const read = deferred()
  t.mock.method(globalThis, 'fetch', () => read.promise)
  const f = setup()
  f.stop()
  read.resolve(response({ onboardingContext: { node: context() }, userProfile: { node: profile }, analyticUserProfile: null }))
  await new Promise((resolve) => setTimeout(resolve, 5))
  assert.equal(f.controller.store.getState().visible, false)
})
test('confirmed completion opens the configured organization destination', async (t) => {
  const ready = context({ step: 'READY', flow: 'INVITED_EXISTING', settings: { ...context().settings, destination: 'MODELS' } })
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('onboardingComplete')) return response({ onboardingComplete: { node: { ...ready, required: false } } })
    if (query.includes('onboardingPresent')) return response({ onboardingPresent: { node: ready } })
    return response({ onboardingContext: { node: ready }, userProfile: { node: profile }, analyticUserProfile: null })
  })
  const f = setup()
  t.after(f.stop)
  await until(() => f.controller.store.getState().visible)
  await f.controller.finish()
  assert.equal(f.controller.store.getState().visible, false)
  assert.equal(f.paths.at(-1), "/alex's%20organization/models")
})
test('settings preview uses confirmed values and cancelled responses cannot replace them', async (t) => {
  const write = deferred()
  const saved = context().settings
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    if (query.includes('organizationOnboardingSettingsUpdate')) return write.promise
    return response({ organizationOnboardingSettings: { node: saved } })
  })
  const controller = new OrganizationOnboardingController()
  t.after(controller.cancel)
  await controller.load()
  controller.store.form.change('draft.welcome', 'Local draft')
  assert.equal(controller.store.getState().saved.welcome, 'Welcome')
  const saving = controller.update({ welcome: 'Submitted' }, new AbortController().signal)
  controller.cancel()
  write.resolve(response({ organizationOnboardingSettingsUpdate: { node: { ...saved, welcome: 'Submitted' } } }))
  await assert.rejects(saving)
  assert.equal(controller.store.getState().saved.welcome, 'Welcome')
  assert.equal(controller.store.form.get('draft.welcome'), 'Local draft')
})

test('invited new accounts receive the black first presentation and reminders use a popup', async (t) => {
  let seen = false
  t.mock.method(globalThis, 'fetch', async (_url, init) => {
    const { query } = JSON.parse(init.body)
    const current = context({ flow: 'INVITED_NEW', hasBeenPresented: seen })
    if (query.includes('onboardingPresent')) {
      seen = true
      return response({ onboardingPresent: { node: current } })
    }
    return response({ onboardingContext: { node: current }, userProfile: { node: profile }, analyticUserProfile: null })
  })
  const f = setup()
  t.after(f.stop)
  await until(() => f.controller.store.getState().visible)
  assert.equal(f.controller.store.getState().firstPresentation, true)
  f.controller.dismiss()
  await f.controller.load(false)
  assert.equal(f.controller.store.getState().firstPresentation, false)
})
