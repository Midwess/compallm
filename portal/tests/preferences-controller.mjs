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
const { configureUserPreferenceForm } = await server.ssrLoadModule(
  '../libs/storybook/src/pages/settings/preferences/user/form.ts',
)
const { parsePreferencePatch } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/preferences/user/values.ts')

function setup() {
  const controller = new SettingsController('Research', { navigate() {} }, { signOut() {} }, 'general')
  controller.pages.preferences.user.store.setState({ status: 'ready' })
  configureUserPreferenceForm(controller.pages.preferences.user, (error, retry) => controller.apiError(error, retry))
  return controller
}
function response(node) {
  return Response.json({ data: { userPreferencesUpdate: { node: merge({}, userPreferenceDefaults, node) } } })
}
async function settled(form) {
  for (let i = 0; i < 100 && form.pending; i++) await new Promise((resolve) => setTimeout(resolve, 1))
  assert.equal(form.pending, false)
}

test('loads authoritative user preferences into its own store and form without saving', async () => {
  const original = globalThis.fetch
  const controller = setup(),
    user = controller.pages.preferences.user
  const node = merge({}, userPreferenceDefaults, {
    navigation: { home: 'MODELS' },
    calendar: { firstDay: 'FRIDAY' },
    theme: { mode: 'DARK', palettes: { dark: 'MAGIC_BLUE' } },
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
    assert.equal(draft.darkTheme, 'MAGIC_BLUE')
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
    sidebar: { badgeStyle: sidebar.badgeStyle, order: sidebar.entries.map(({ item }) => item), visibility: sidebar.entries },
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
    lightTheme: 'PURE_LIGHT',
    darkTheme: 'MAGIC_BLUE',
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
    theme: { mode: 'SYSTEM', palettes: { light: 'PURE_LIGHT', dark: 'MAGIC_BLUE' } },
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

test('save reconciles only submitted preferences, queues changes, and keeps preview language local', async () => {
  const original = globalThis.fetch
  const controller = setup(),
    form = controller.pages.preferences.user.store.form
  const writes = [],
    pending = []
  globalThis.fetch = (_url, init) => {
    const body = JSON.parse(init.body)
    writes.push(Object.values(body.variables)[0])
    return new Promise((resolve) => pending.push(resolve))
  }
  try {
    form.change('draft.home', 'MODELS')
    await new Promise((resolve) => setTimeout(resolve, 0))
    form.change('draft.home', 'PROVIDERS')
    form.change('draft.underlineLinks', true)
    pending[0](response({ navigation: { home: 'MODELS' }, accessibility: { underlineLinks: false } }))
    for (let i = 0; i < 100 && pending.length < 2; i++) await new Promise((resolve) => setTimeout(resolve, 1))
    assert.equal(form.values.draft.home, 'PROVIDERS')
    assert.equal(form.values.draft.underlineLinks, true)
    pending[1](response({ navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } }))
    await settled(form)
    assert.deepEqual(writes, [
      { navigation: { home: 'MODELS' } },
      { navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } },
    ])
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
