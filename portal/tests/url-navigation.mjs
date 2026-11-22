import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { createMemoryRouter } from 'react-router'

const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
})
after(() => server.close())
const { UrlController } = await server.ssrLoadModule('/src/url/controller.ts')
const settle = () => new Promise((resolve) => setTimeout(resolve, 10))

test('Back pops model/configuration history and preserves Forward after committed edits', async (t) => {
  const entries = [
    '/workspace/providers/new/choose-provider',
    '/workspace/providers/new/configure-provider?provider=openai',
    '/workspace/providers/new/configure-provider?provider=openai&model=gpt-4o',
  ]
  const previousWindow = globalThis.window
  t.after(() => {
    globalThis.window = previousWindow
  })
  globalThis.window = { location: new URL('http://localhost' + entries[2]), history: { state: { idx: 2 } } }
  const router = createMemoryRouter([{ path: '*', Component: () => null }], { initialEntries: entries, initialIndex: 2 })
  const url = new UrlController()
  const disconnect = url.connect(router)
  t.after(() => {
    disconnect()
    router.dispose()
  })
  url.back(entries[1])
  await settle()
  assert.equal(router.state.historyAction, 'POP')
  assert.equal(router.state.location.search, '?provider=openai')
  assert.equal(url.store.getState().search, '?provider=openai')
  await router.navigate(1)
  assert.equal(router.state.location.search, '?provider=openai&model=gpt-4o')
  url.back(entries[0], { steps: 2 })
  await settle()
  assert.equal(router.state.location.pathname, entries[0])
  await router.navigate(2)
  assert.equal(router.state.location.search, '?provider=openai&model=gpt-4o')
})

test('A directly opened model replaces its entry with the configuration fallback', async (t) => {
  const initial = '/workspace/providers/new/configure-provider?provider=openai&model=gpt-4o'
  const previousWindow = globalThis.window
  t.after(() => {
    globalThis.window = previousWindow
  })
  globalThis.window = { location: new URL('http://localhost' + initial), history: { state: { idx: 0 } } }
  const router = createMemoryRouter([{ path: '*', Component: () => null }], { initialEntries: [initial] })
  const url = new UrlController()
  const disconnect = url.connect(router)
  t.after(() => {
    disconnect()
    router.dispose()
  })
  url.back('/workspace/providers/new/configure-provider?provider=openai')
  await settle()
  assert.equal(router.state.historyAction, 'REPLACE')
  assert.equal(router.state.location.search, '?provider=openai')
})
