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
const { AppController } = await server.ssrLoadModule('/src/app/controller.ts')

test('page loading scopes settle immediately and do not block startup or other regions', () => {
  const app = new AppController()
  const page = app.createLoadingScope()
  const section = app.createLoadingScope()
  const pending = page.beginLoading('detail')
  const nested = section.beginLoading('models')
  assert.equal(page.store.getState().loading, true)
  assert.equal(app.store.getState().loading, false)
  pending.complete()
  assert.equal(page.store.getState().loading, false)
  assert.equal(section.store.getState().loading, true)
  nested.cancel()
  assert.equal(section.store.getState().loading, false)
})

test('obsolete reads cannot finish their replacement and failed scopes retry only explicitly', async () => {
  const scope = new AppController().createLoadingScope()
  const stale = scope.beginLoading('detail')
  const latest = scope.beginLoading('detail')
  stale.complete()
  assert.equal(scope.store.getState().loading, true)
  let retries = 0
  latest.fail(new Error('offline'), async () => {
    retries++
  })
  assert.ok(scope.store.getState().error)
  assert.equal(retries, 0)
  await scope.retry()
  assert.equal(retries, 1)
  latest.cancel()
  assert.deepEqual(scope.store.getState(), { loading: false, error: null })
})
