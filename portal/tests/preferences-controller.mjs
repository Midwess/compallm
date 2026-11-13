import merge from 'lodash-es/merge.js'
import { userPreferenceDefaults } from '@compallm/contract/preference'
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
const { resolveCodeAppearance } = await server.ssrLoadModule('../libs/storybook/src/components/code-appearance.ts')

function createSettingsController(...args) {
  return new SettingsController(...args, new UserPreferenceController())
}

const { configureUserPreferenceForm } = await server.ssrLoadModule(
  '../libs/storybook/src/pages/settings/preferences/user/form.ts',
)
const { parsePreferencePatch } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/preferences/user/values.ts')

function setup() {
  const controller = createSettingsController('Research', { navigate() {} }, { signOut() {} }, 'general')
  controller.pages.preferences.user.store.setState({ status: 'ready', saved: structuredClone(userPreferenceDefaults) })
  configureUserPreferenceForm(controller.pages.preferences.user, (error, retry) => controller.apiError(error, retry))
  return controller
}
function response(node) {
  return Response.json({
    data: {
      userPreferencesUpdate: {
        node: Object.fromEntries(
          Object.entries(node).map(([category, values]) => [category, merge({}, userPreferenceDefaults[category], values)]),
        ),
      },
    },
  })
}
async function settled(form) {
  for (let i = 0; i < 100 && form.pending; i++) await new Promise((resolve) => setTimeout(resolve, 1))
  assert.equal(form.pending, false)
}

test('code appearance distinguishes interface and system modes and applies font preferences', () => {
  assert.deepEqual(resolveCodeAppearance('FOLLOW_INTERFACE', 'LIGHT', 'MONOSPACE', 'PX16', false), {
    theme: 'light',
    fontSize: 16,
    fontFamily: 'monospace',
  })
  assert.equal(resolveCodeAppearance('FOLLOW_INTERFACE', 'SYSTEM', 'DEFAULT', 'PX12', true).theme, 'light')
  assert.equal(resolveCodeAppearance('SYSTEM', 'LIGHT', 'DEFAULT', 'PX12', false).theme, 'dark')
  assert.equal(resolveCodeAppearance('LINEAR_DARK', 'LIGHT', 'DEFAULT', 'PX12', true).theme, 'dark')
  assert.equal(resolveCodeAppearance('LINEAR_LIGHT', 'DARK', 'DEFAULT', 'PX12', false).theme, 'light')
  assert.equal(resolveCodeAppearance('SYSTEM', 'LIGHT', 'DEFAULT', 'PX12', true).fontFamily, 'var(--font-code)')
})

test('app controller waits for explicit completions and the one-second minimum', async (t) => {
  const { AppController } = await server.ssrLoadModule('/src/app/controller.ts')
  t.mock.timers.enable({ apis: ['Date', 'setTimeout'], now: 10000 })
  const app = new AppController()
  const session = app.beginLoading('session')
  const preferences = app.beginLoading('preferences')
  session.complete()
  t.mock.timers.tick(500)
  assert.equal(app.store.getState().loading, true)
  preferences.complete()
  t.mock.timers.tick(499)
  assert.equal(app.store.getState().loading, true)
  t.mock.timers.tick(1)
  assert.equal(app.store.getState().loading, false)
  const slow = app.beginLoading('slow')
  t.mock.timers.tick(3000)
  assert.equal(app.store.getState().loading, true)
  slow.complete()
  assert.equal(app.store.getState().loading, false)
})

test('app controller retains failed loads for retry and ignores obsolete completions', async (t) => {
  const { AppController } = await server.ssrLoadModule('/src/app/controller.ts')
  t.mock.timers.enable({ apis: ['Date', 'setTimeout'], now: 10000 })
  const app = new AppController()
  const old = app.beginLoading('preferences')
  let current = app.beginLoading('preferences')
  old.complete()
  current.fail({ info: { message: 'Unavailable' } }, async () => {
    current = app.beginLoading('preferences')
    current.complete()
  })
  t.mock.timers.tick(3000)
  assert.equal(app.store.getState().loading, true)
  assert.notEqual(app.store.getState().error, null)
  await app.retry()
  assert.equal(app.store.getState().loading, false)
  const cancelled = app.beginLoading('cancelled')
  cancelled.cancel()
  t.mock.timers.tick(1000)
  assert.equal(app.store.getState().loading, false)
})

test('app preference ownership survives settings navigation and clears when the user leaves', async () => {
  const { createStore } = await import('zustand/vanilla')
  const original = globalThis.fetch
  const user = new UserPreferenceController()
  const auth = { store: createStore(() => ({ status: 'anonymous', currentUser: null })) }
  const pending = []
  globalThis.fetch = (_url, init) => new Promise((resolve) => pending.push({ resolve, signal: init.signal }))
  const stop = user.start(auth)
  try {
    assert.equal(pending.length, 0)
    auth.store.setState({ status: 'authenticated', currentUser: { userId: '1', organizationId: '2' } })
    assert.equal(pending.length, 1)
    pending[0].resolve(Response.json({ data: { userPreferences: { node: userPreferenceDefaults } } }))
    for (let i = 0; i < 100 && user.store.getState().status !== 'ready'; i++)
      await new Promise((resolve) => setTimeout(resolve, 1))
    assert.equal(user.store.getState().status, 'ready')
    const settings = new SettingsController('Research', { navigate() {} }, { signOut() {} }, 'general', user)
    assert.equal(settings.pages.preferences.user, user)
    settings.setPage('profile')
    auth.store.setState({ currentUser: { userId: '1', organizationId: '3' } })
    assert.equal(pending.length, 1)
    assert.notEqual(user.store.getState().saved, null)
    auth.store.setState({ status: 'authenticated', currentUser: { userId: '4', organizationId: '5' } })
    assert.equal(user.store.getState().saved, null)
    assert.equal(pending.length, 2)
    auth.store.setState({ status: 'anonymous', currentUser: null })
    assert.equal(pending[1].signal.aborted, true)
    pending[1].resolve(Response.json({ data: { userPreferences: { node: userPreferenceDefaults } } }))
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(user.store.getState().saved, null)
    assert.equal(user.store.getState().status, 'idle')
  } finally {
    stop()
    globalThis.fetch = original
  }
})

test('loads authoritative user preferences into its own store and form without saving', async () => {
  const original = globalThis.fetch
  const controller = setup(),
    user = controller.pages.preferences.user
  const node = merge({}, userPreferenceDefaults, {
    navigation: { home: 'MODELS' },
    calendar: { firstDay: 'FRIDAY' },
    theme: { mode: 'DARK', palettes: { dark: 'BLUEBERRY_DARK' } },
    codeAppearance: { font: { size: 'PX16' } },
    sidebar: { entries: userPreferenceDefaults.sidebar.entries.map((entry) => ({ ...entry, visibility: 'HIDDEN' })) },
  })
  let reads = 0
  const notifications = []
  const unsubscribe = user.store.subscribe((state) => notifications.push(state.status))
  globalThis.fetch = async (_url, init) => {
    assert.ok(JSON.parse(init.body).query.startsWith('query'))
    reads++
    return Response.json({ data: { userPreferences: { node } } })
  }
  try {
    await user.load()
    assert.equal(reads, 1)
    assert.ok(notifications.includes('loading'))
    assert.equal(user.store.getState().status, 'ready')
    assert.deepEqual(user.store.getState().saved, node)
    const draft = user.store.form.values.draft
    assert.equal(draft.home, 'MODELS')
    assert.equal(draft.firstDay, 'FRIDAY')
    assert.equal(draft.darkTheme, 'BLUEBERRY_DARK')
    assert.equal(draft.codeFontSize, 'PX16')
    assert.equal(draft.sidebarMODELS, 'HIDDEN')
    assert.equal(user.store.form.dirty, false)
    assert.deepEqual(user.store.form.baseline.draft, draft)
    assert.equal(Object.hasOwn(controller.store.getState(), 'preferences'), false)
  } finally {
    unsubscribe()
    user.cancel()
    globalThis.fetch = original
  }
})

test('preference reads reject partial errors and discard obsolete and cancelled responses', async () => {
  const original = globalThis.fetch
  const user = setup().pages.preferences.user
  const pending = []
  globalThis.fetch = (_url, init) => new Promise((resolve) => pending.push({ resolve, signal: init.signal }))
  const result = (home) =>
    Response.json({
      data: {
        userPreferences: { node: merge({}, userPreferenceDefaults, { navigation: { home } }) },
      },
    })
  try {
    const first = user.load(),
      second = user.load()
    assert.equal(pending[0].signal.aborted, true)
    pending[1].resolve(result('PROVIDERS'))
    await second
    pending[0].resolve(result('MODELS'))
    await first
    assert.equal(user.store.getState().saved.navigation.home, 'PROVIDERS')
    const cancelled = user.load()
    user.cancel()
    pending[2].resolve(result('CHAT'))
    await cancelled
    assert.equal(user.store.getState().saved.navigation.home, 'PROVIDERS')
    globalThis.fetch = async () =>
      Response.json({
        data: { userPreferences: { node: userPreferenceDefaults } },
        errors: [{ message: 'Denied', extensions: { code: 'FORBIDDEN', status: 403 } }],
      })
    await user.load()
    assert.equal(user.store.getState().status, 'denied')
    assert.equal(user.store.getState().saved.navigation.home, 'PROVIDERS')
    globalThis.fetch = async () => result('MODELS')
    await user.load()
    assert.equal(user.store.getState().status, 'ready')
    assert.equal(user.store.form.values.draft.home, 'MODELS')
  } finally {
    user.cancel()
    globalThis.fetch = original
  }
})

test('initial form values come from Core defaults and round-trip through the write contract', async () => {
  const { userPreferenceDefaults } = await import('@compallm/contract/preference')
  const controller = setup()
  const { input } = parsePreferencePatch(controller.pages.preferences.user.store.form.values.draft)
  const { sidebar, typography, ...groups } = userPreferenceDefaults
  assert.deepEqual(input, {
    ...groups,
    typography: { size: typography.size },
    sidebar: {
      layout: sidebar.layout,
      badgeStyle: sidebar.badgeStyle,
      order: sidebar.entries.map(({ item }) => item),
      visibility: sidebar.entries,
    },
  })
  controller.pages.preferences.user.store.form.stop()
})

test('preference edits use grouped wire values and preserve omitted siblings', () => {
  const { input } = parsePreferencePatch({
    home: 'MODELS',
    displayNames: 'USERNAME',
    firstDay: 'FRIDAY',
    fontSize: 'LARGER',
    theme: 'SYSTEM',
    lightTheme: 'MARSHMALLOW_WHITE',
    darkTheme: 'BLUEBERRY_DARK',
    underlineLinks: true,
    reduceMotion: false,
    pointerCursors: true,
    emoticons: false,
    sendShortcut: 'MODIFIER_ENTER',
    codeTheme: 'SYSTEM',
    codeFont: 'MONOSPACE',
    codeFontSize: 'PX14',
    codeLanguage: 'rust',
    sidebarOrder: ['MODELS', 'PROVIDERS', 'PLUGINS', 'NOTEBOOKS'],
    sidebarBadgeStyle: 'COUNT',
    sidebarMODELS: 'HIDDEN',
    sidebarPLUGINS: 'WHEN_BADGED',
  })
  assert.deepEqual(input, {
    navigation: { home: 'MODELS' },
    memberDisplay: { nameFormat: 'USERNAME' },
    calendar: { firstDay: 'FRIDAY' },
    typography: { size: 'LARGER' },
    theme: { mode: 'SYSTEM', palettes: { light: 'MARSHMALLOW_WHITE', dark: 'BLUEBERRY_DARK' } },
    accessibility: { underlineLinks: true, reduceMotion: false, pointerCursors: true },
    messageComposition: { convertEmoticons: false, sendShortcut: 'MODIFIER_ENTER' },
    codeAppearance: { theme: 'SYSTEM', font: { family: 'MONOSPACE', size: 'PX14' } },
    sidebar: {
      order: ['MODELS', 'PROVIDERS', 'PLUGINS', 'NOTEBOOKS'],
      badgeStyle: 'COUNT',
      visibility: [
        { item: 'MODELS', visibility: 'HIDDEN' },
        { item: 'PLUGINS', visibility: 'WHEN_BADGED' },
      ],
    },
  })
  assert.deepEqual(parsePreferencePatch({ codeFontSize: 'PX16' }).input, { codeAppearance: { font: { size: 'PX16' } } })
  assert.deepEqual(parsePreferencePatch({ sidebarMODELS: 'ALWAYS' }).input, {
    sidebar: { visibility: [{ item: 'MODELS', visibility: 'ALWAYS' }] },
  })
})

test('loaded home preferences delegate navigation to the organization controller and preserve direct links', async () => {
  const { createStore } = await import('zustand/vanilla')
  const { OrganizationController } = await server.ssrLoadModule('/src/organization/controller.ts')
  const original = globalThis.fetch
  try {
    for (const home of ['CHAT', 'MODELS', 'PROVIDERS']) {
      const paths = []
      const url = {
        store: createStore(() => ({ pathname: '/Research%20team', search: '', hash: '' })),
        navigate: (path, options) => paths.push({ path, options }),
      }
      const organization = new OrganizationController(
        { userId: '1', id: '2', organizationId: '3', organizationName: 'Research team' },
        url,
        { signOut() {} },
      )
      const user = new UserPreferenceController()
      const disconnect = user.connectOrganization(organization, url)
      globalThis.fetch = async () =>
        Response.json({
          data: {
            userPreferences: { node: merge({}, userPreferenceDefaults, { navigation: { home } }) },
          },
        })
      await user.load()
      assert.deepEqual(paths, [{ path: `/Research%20team/${home.toLowerCase()}`, options: { replace: true } }])
      url.store.setState({ pathname: '/Research%20team/settings/general' })
      await user.load()
      assert.equal(paths.length, 1)
      url.store.setState({ pathname: '/Research%20team/providers/new/choose-provider' })
      assert.equal(paths.length, 1)
      url.store.setState({ pathname: '/Research%20team/' })
      assert.equal(paths.length, 2)
      disconnect()
      url.store.setState({ pathname: '/Research%20team' })
      assert.equal(paths.length, 2)
      user.cancel()
    }
  } finally {
    globalThis.fetch = original
  }
})

test('sidebar layout applies categories and cross-category moves after confirmed saves only', async () => {
  const { createStore } = await import('zustand/vanilla')
  const { OrganizationController } = await server.ssrLoadModule('/src/organization/controller.ts')
  const original = globalThis.fetch
  const url = { store: createStore(() => ({ pathname: '/Research/settings/general' })), navigate() {} }
  const organization = new OrganizationController(
    { userId: '1', id: '2', organizationId: '3', organizationName: 'Research' },
    url,
    { signOut() {} },
  )
  const user = new UserPreferenceController()
  const disconnect = user.connectOrganization(organization, url)
  let saved = structuredClone(userPreferenceDefaults)
  const layout = saved.sidebar.layout
  const plugin = layout[0].items.pop()
  layout[2].items.unshift(plugin)
  layout.reverse()
  saved.sidebar.entries.sort(
    (a, b) => layout.flatMap((s) => s.items).indexOf(a.item) - layout.flatMap((s) => s.items).indexOf(b.item),
  )
  let reject = false
  globalThis.fetch = async (_url, init) => {
    const { query, variables } = JSON.parse(init.body)
    if (query.startsWith('query')) return Response.json({ data: { userPreferences: { node: saved } } })
    if (reject) return Response.json({ errors: [{ message: 'Unavailable', extensions: { status: 503 } }] })
    const patch = Object.values(variables)[0]
    saved = {
      ...saved,
      sidebar: {
        ...saved.sidebar,
        layout: patch.sidebar.layout,
        entries: patch.sidebar.layout
          .flatMap((s) => s.items)
          .map((item) => saved.sidebar.entries.find((entry) => entry.item === item)),
      },
    }
    return Response.json({ data: { userPreferencesUpdate: { node: { sidebar: saved.sidebar } } } })
  }
  try {
    await user.load()
    assert.deepEqual(organization.store.getState().sidebarLayout, layout)
    const next = [...layout].reverse()
    await user.update({ sidebarLayout: next }, new AbortController().signal)
    assert.deepEqual(organization.store.getState().sidebarLayout, next)
    reject = true
    await assert.rejects(user.update({ sidebarLayout: layout }, new AbortController().signal))
    assert.deepEqual(organization.store.getState().sidebarLayout, next)
    const remounted = new OrganizationController(
      { userId: '1', id: '2', organizationId: '3', organizationName: 'Research' },
      url,
      { signOut() {} },
    )
    const stop = user.connectOrganization(remounted, url)
    assert.deepEqual(remounted.store.getState().sidebarLayout, next)
    stop()
  } finally {
    disconnect()
    user.cancel()
    globalThis.fetch = original
  }
})

test('save reconciles only submitted preferences, queues changes, and keeps preview language local', async () => {
  const original = globalThis.fetch
  const controller = setup(),
    form = controller.pages.preferences.user.store.form
  const writes = [],
    pending = [],
    selections = []
  const originalTheme = controller.pages.preferences.user.store.getState().saved.theme
  globalThis.fetch = (_url, init) => {
    const body = JSON.parse(init.body)
    writes.push(Object.values(body.variables)[0])
    selections.push(body.query)
    return new Promise((resolve) => pending.push(resolve))
  }
  try {
    form.change('draft.home', 'MODELS')
    await new Promise((resolve) => setTimeout(resolve, 0))
    form.change('draft.home', 'PROVIDERS')
    form.change('draft.underlineLinks', true)
    pending[0](response({ navigation: { home: 'MODELS' } }))
    for (let i = 0; i < 100 && pending.length < 2; i++) await new Promise((resolve) => setTimeout(resolve, 1))
    assert.equal(form.values.draft.home, 'PROVIDERS')
    assert.equal(form.values.draft.underlineLinks, true)
    pending[1](response({ navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } }))
    await settled(form)
    assert.deepEqual(writes, [
      { navigation: { home: 'MODELS' } },
      { navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } },
    ])
    assert.match(selections[0], /node\s*\{\s*navigation\s*\{\s*home\s*\}\s*\}/)
    assert.match(selections[1], /accessibility\s*\{/)
    for (const query of selections)
      assert.doesNotMatch(query, /\b(theme|sidebar|calendar|typography|codeAppearance|memberDisplay|messageComposition)\s*\{/)
    assert.equal(controller.pages.preferences.user.store.getState().saved.theme, originalTheme)
    assert.equal(form.baseline.draft.home, 'PROVIDERS')
    assert.equal(controller.pages.preferences.user.store.getState().saved.navigation.home, 'PROVIDERS')
    assert.equal(controller.pages.preferences.user.store.getState().saved.accessibility.underlineLinks, true)
    assert.equal(Object.hasOwn(controller.store.getState(), 'preferences'), false)
    form.change('draft.codeLanguage', 'rust')
    await settled(form)
    assert.equal(writes.length, 2)
    assert.equal(form.values.draft.codeLanguage, 'rust')
  } finally {
    form.stop()
    globalThis.fetch = original
  }
})

test('failed saves retain drafts, explicitly retry latest values, and ignore cancelled responses', async () => {
  const original = globalThis.fetch
  const controller = setup(),
    form = controller.pages.preferences.user.store.form
  const writes = []
  let retry, finish
  const stop = configureUserPreferenceForm(controller.pages.preferences.user, (error, retry) =>
    controller.apiError(error, retry),
  )
  controller.apiError = (_error, action) => {
    retry = action
  }
  globalThis.fetch = async (_url, init) => {
    writes.push(Object.values(JSON.parse(init.body).variables)[0])
    if (writes.length === 1) return Response.json({ errors: [{ message: 'Unavailable', extensions: { status: 503 } }] })
    if (writes.length === 2) return response({ navigation: { home: 'PROVIDERS' } })
    return new Promise((resolve) => {
      finish = resolve
    })
  }
  try {
    form.change('draft.home', 'MODELS')
    await settled(form)
    assert.equal(form.values.draft.home, 'MODELS')
    assert.equal(form.baseline.draft.home, 'CHAT')
    form.change('draft.home', 'PROVIDERS')
    assert.equal(writes.length, 1)
    assert.equal(typeof retry, 'function')
    retry()
    await settled(form)
    assert.deepEqual(writes[1], { navigation: { home: 'PROVIDERS' } })
    assert.equal(form.baseline.draft.home, 'PROVIDERS')
    form.change('draft.home', 'MODELS')
    await new Promise((resolve) => setTimeout(resolve, 0))
    stop()
    controller.pages.preferences.user.store.setState({ status: 'ready' })
    configureUserPreferenceForm(controller.pages.preferences.user, (error, retry) => controller.apiError(error, retry))
    finish(response({ navigation: { home: 'MODELS' } }))
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(form.baseline.draft.home, 'PROVIDERS')
    assert.equal(form.values.draft.home, 'MODELS')
    assert.equal(controller.pages.preferences.user.store.getState().saved.navigation.home, 'PROVIDERS')
    retry()
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(writes.length, 3)
  } finally {
    form.stop()
    globalThis.fetch = original
  }
})
