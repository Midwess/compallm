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

test('preference edits use grouped wire values and preserve omitted siblings', () => {
  const { input } = parsePreferencePatch({
    home: 'Models',
    displayNames: 'Username',
    firstDay: 'Friday',
    fontSize: 'Larger',
    theme: 'System preference',
    lightTheme: 'Pure Light',
    darkTheme: 'Magic Blue',
    underlineLinks: true,
    reduceMotion: false,
    pointerCursors: true,
    emoticons: false,
    sendShortcut: '⌘ + Enter',
    codeTheme: 'System preference',
    codeFont: 'Monospace',
    codeFontSize: '14px',
    codeLanguage: 'rust',
    sidebarOrder: 'Models,Providers,Plugins,Notebooks',
    sidebarBadgeStyle: 'Count',
    sidebarModels: "Don't show",
    sidebarPlugins: 'Show when badged',
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
  assert.deepEqual(parsePreferencePatch({ codeFontSize: '16px' }).input, { codeAppearance: { font: { size: 'PX16' } } })
  assert.deepEqual(parsePreferencePatch({ sidebarModels: 'Always show' }).input, {
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
    form.change('preferences.home', 'Models')
    await new Promise((resolve) => setTimeout(resolve, 0))
    form.change('preferences.home', 'Providers')
    form.change('preferences.underlineLinks', true)
    pending[0](response({ navigation: { home: 'MODELS' }, accessibility: { underlineLinks: false } }))
    for (let i = 0; i < 100 && pending.length < 2; i++) await new Promise((resolve) => setTimeout(resolve, 1))
    assert.equal(form.values.preferences.home, 'Providers')
    assert.equal(form.values.preferences.underlineLinks, true)
    pending[1](response({ navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } }))
    await settled(form)
    assert.deepEqual(writes, [
      { navigation: { home: 'MODELS' } },
      { navigation: { home: 'PROVIDERS' }, accessibility: { underlineLinks: true } },
    ])
    assert.equal(form.baseline.preferences.home, 'Providers')
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
    form.change('preferences.home', 'Models')
    await settled(form)
    assert.equal(form.values.preferences.home, 'Models')
    assert.equal(form.baseline.preferences.home, 'Chat')
    form.change('preferences.home', 'Providers')
    assert.equal(writes.length, 1)
    assert.equal(typeof retry, 'function')
    retry()
    await settled(form)
    assert.deepEqual(writes[1], { navigation: { home: 'PROVIDERS' } })
    assert.equal(form.baseline.preferences.home, 'Providers')
    form.change('preferences.home', 'Models')
    await new Promise((resolve) => setTimeout(resolve, 0))
    stop()
    configureSettingsForm(controller)
    finish(response({ navigation: { home: 'MODELS' } }))
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(form.baseline.preferences.home, 'Providers')
    assert.equal(form.values.preferences.home, 'Models')
    retry()
    await new Promise((resolve) => setTimeout(resolve, 0))
    assert.equal(writes.length, 3)
  } finally {
    form.stop()
    globalThis.fetch = original
  }
})
