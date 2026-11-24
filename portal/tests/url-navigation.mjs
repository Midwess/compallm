import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { svelte } from '@sveltejs/vite-plugin-svelte'
import { fileURLToPath } from 'node:url'
import { tmpdir } from 'node:os'
import { join } from 'node:path'
const server = await createServer({
  root: fileURLToPath(new URL('..', import.meta.url)),
  configFile: false,
  plugins: [svelte({ configFile: false, compilerOptions: { experimental: { async: true } } })],
  cacheDir: join(tmpdir(), 'compallm-svelte-url-tests'),
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  resolve: {
    alias: { $houdini: fileURLToPath(new URL('../../libs/contract/houdini/generated', import.meta.url)) },
    dedupe: ['svelte'],
  },
  ssr: { noExternal: ['@compallm/storybook', '@compallm/contract'] },
})
after(() => server.close())
const { UrlController } = await server.ssrLoadModule('/src/url/controller.ts')
function fixture(t, initial = '/Research/providers/new/choose-provider') {
  const original = globalThis.window,
    entries = [{ url: new URL(initial, 'http://localhost'), state: {} }]
  let index = 0
  const url = new UrlController()
  const sync = () => {
    globalThis.window.location = entries[index].url
    url.synchronize(entries[index].url, entries[index].state)
  }
  globalThis.window = {
    location: entries[0].url,
    history: {
      go(delta) {
        index += delta
        sync()
      },
    },
  }
  t.after(() => {
    globalThis.window = original
  })
  sync()
  url.connect(async (href, replace, state) => {
    const next = { url: new URL(href, 'http://localhost'), state }
    url.beforeNavigation(
      next.url,
      () => {
        throw new Error('unexpected duplicate guard')
      },
      () => {},
    )
    if (replace) entries[index] = next
    else {
      entries.splice(index + 1)
      entries.push(next)
      index++
    }
    sync()
  })
  return { url, entries }
}
test('provider breadcrumbs pop matching history, retain Forward, and replace direct-link fallbacks', async (t) => {
  const { url, entries } = fixture(t)
  const configure = '/Research/providers/new/configure-provider?provider=openai'
  await url.navigate(configure)
  await url.navigate(configure + '&model=first')
  await url.navigate(configure + '&model=second')
  await url.back(configure, { matchTarget: true })
  assert.equal(url.store.getState().search, '?provider=openai')
  globalThis.window.history.go(2)
  assert.equal(url.store.getState().search, '?provider=openai&model=second')
  await url.back('/Research/chat', { matchTarget: true })
  assert.equal(entries.length, 4)
  assert.equal(url.store.getState().pathname, '/Research/chat')
})
test('keep preserves the current draft URL and discard carries the intended provider history', async (t) => {
  const { url } = fixture(t)
  let request
  url.block(
    () => true,
    (value) => {
      request = value
    },
  )
  const target = '/Research/providers/new/configure-provider?provider=openai'
  const kept = url.navigate(target)
  request.keep()
  assert.equal(await kept, false)
  assert.equal(url.store.getState().pathname, '/Research/providers/new/choose-provider')
  const discarded = url.navigate(target)
  request.discard()
  assert.equal(await discarded, true)
  assert.equal(url.store.getState().search, '?provider=openai')
  await url.back('/Research/providers/new/choose-provider', { matchTarget: true })
  assert.equal(url.store.getState().pathname, '/Research/providers/new/choose-provider')
})
