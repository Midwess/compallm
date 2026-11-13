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
const { configureSettingsForm } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/form.ts')
const { parsePreferencePatch } = await server.ssrLoadModule('../libs/storybook/src/pages/settings/preference-values.ts')

function setup() {
  const controller = new SettingsController('Research', { navigate() {} }, { signOut() {} }, 'general')
  configureSettingsForm(controller)
  return controller
}
function response(node) {
  return Response.json({ data: { userPreferencesUpdate: { node } } })
}
async function settled(form) {
  for (let i = 0; i < 100 && form.pending; i++) await new Promise((resolve) => setTimeout(resolve, 1))
  assert.equal(form.pending, false)
}

test('initial form values come from Core defaults and round-trip through the write contract', async () => {
  const { userPreferenceDefaults } = await import('@compallm/contract/preference')
  const controller = setup()
  const { input } = parsePreferencePatch(controller.store.form.values.preferences)
  const { sidebar, typography, ...groups } = userPreferenceDefaults
  assert.deepEqual(input, {
    ...groups,
    typography: { size: typography.size },
    sidebar: { badgeStyle: sidebar.badgeStyle, order: sidebar.entries.map(({ item }) => item), visibility: sidebar.entries },
  })
  controller.store.form.stop()
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
    form = controller.store.form
  const writes = [],
    pending = []
  globalThis.fetch = (_url, init) => {
    const body = JSON.parse(init.body)
    writes.push(Object.values(body.variables)[0])
    return new Promise((resolve) => pending.push(resolve))
  }
  try {
    form.change('preferences.home', 'MODELS')
    await new Promise((resolve) => setTimeout(resolve, 0))
    form.change('preferences.home', 'PROVIDERS')
    form.change('preferences.underlineLinks', true)
    pending[0](response({ navigation: { home: 'MODELS' }, accessibility: { underlineLinks: false } }))
    for (let i = 0; i < 100 && pending.length < 2; i++) await new Promise((resolve) => setTimeout(resolve, 1))
    assert.equal(form.values.preferences.home, 'PROVIDERS')
    assert.equal(form.values.preferences.underlineLinks, true)
    pending[1](response({ navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } }))
    await settled(form)
    assert.deepEqual(writes, [
      { navigation: { home: 'MODELS' } },
      { navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } },
    ])
    assert.equal(form.baseline.preferences.home, 'PROVIDERS')
    form.change('preferences.codeLanguage', 'rust')
    await settled(form)
    assert.equal(writes.length, 2)
    assert.equal(form.values.preferences.codeLanguage, 'rust')
  } finally {
    form.stop()
    globalThis.fetch = original
  }
})

test('failed saves retain drafts, explicitly retry latest values, and ignore cancelled responses', async () => {
  const original = globalThis.fetch
  const controller = setup(),
    form = controller.store.form
  const writes = []
  let retry, finish
  const stop = configureSettingsForm(controller)
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
    form.change('preferences.home', 'MODELS')
    await settled(form)
    assert.equal(form.values.preferences.home, 'MODELS')
    assert.equal(form.baseline.preferences.home, 'CHAT')
    form.change('preferences.home', 'PROVIDERS')
    assert.equal(writes.length, 1)
    assert.equal(typeof retry, 'function')
    retry()
    await settled(form)
    assert.deepEqual(writes[1], { navigation: { home: 'PROVIDERS' } })
    assert.equal(form.baseline.preferences.home, 'PROVIDERS')
    form.change('preferences.home', 'MODELS')
    await new Promise((resolve) => setTimeout(resolve, 0))
    stop()
    configureSettingsForm(controller)
    finish(response({ navigation: { home: 'MODELS' } }))
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(form.baseline.preferences.home, 'PROVIDERS')
    assert.equal(form.values.preferences.home, 'MODELS')
    retry()
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(writes.length, 3)
  } finally {
    form.stop()
    globalThis.fetch = original
  }
})
