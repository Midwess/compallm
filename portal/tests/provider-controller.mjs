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
const { CreateProviderController } = await server.ssrLoadModule('/src/provider/controller.ts')
const { AppController } = await server.ssrLoadModule('/src/app/controller.ts')
const { createStore } = await server.ssrLoadModule('zustand/vanilla')
const tick = () => new Promise((resolve) => setTimeout(resolve, 20))
function setup() {
  const store = createStore(() => ({ pathname: '/workspace/providers', search: '', hash: '' }))
  const history = []
  const url = {
    store,
    back(fallback, options = {}) {
      const target = history.splice(Math.max(0, history.length - (options.steps ?? 1)))[0]
      if (target) store.setState(target)
      else {
        const next = new URL(fallback, 'http://localhost')
        store.setState({ pathname: next.pathname, search: next.search, hash: next.hash })
      }
    },
    update(patch, options = {}) {
      if (options.replace === false) history.push({ ...store.getState() })
      const q = new URLSearchParams(store.getState().search)
      for (const [k, v] of Object.entries(patch)) {
        if (v === null) q.delete(k)
        else q.set(k, v)
      }
      store.setState({ pathname: options.pathname ?? store.getState().pathname, search: q.size ? '?' + q : '', hash: '' })
      return true
    },
  }
  const controller = new CreateProviderController(
    url,
    { getCurrentOrganization: () => ({ id: 'org', name: 'workspace' }) },
    new AppController(),
  )
  return { controller, url }
}
const row = (id) => ({ id, name: id, baseUrl: 'https://example.com/v1', logoUrl: '/assets/' + id + '.svg' })
const list = (ids, more = false) =>
  Response.json({
    data: {
      effectiveProviderTemplates: {
        edges: ids.map((id) => ({ node: row(id) })),
        pageInfo: { endCursor: 'cursor-' + ids.at(-1), hasNextPage: more },
      },
    },
  })
const provider = (id) => ({
  ...row(id),
  description: null,
  connection: { baseUrl: 'https://example.com/v1' },
  headers: [],
  authentication: [],
  models: {
    edges: [
      {
        node: {
          id: 'model-' + id,
          name: 'model-' + id,
          displayName: 'Model ' + id,
          description: null,
          serviceKind: 'TEXT',
          capabilities: ['TEXT', 'REASONING'],
          maxContextTokens: 128000,
          inputPrice: null,
          outputPrice: null,
          cachedPrice: null,
          pricingMode: 'TIERED',
          billingModel: 'METERED',
          cacheBilling: 'UNSUPPORTED',
          operations: [{ operation: 'OPENAI_CHAT_COMPLETIONS', path: '/chat/completions' }],
        },
      },
    ],
    pageInfo: { hasNextPage: false },
  },
})
function model(id) {
  return {
    id,
    name: id,
    providerTemplateId: 'a',
    origin: 'GLOBAL',
    customFields: [],
    displayName: id,
    description: null,
    serviceKind: 'TEXT',
    capabilities: ['TEXT'],
    maxContextTokens: 128000,
    inputPrice: null,
    outputPrice: null,
    cachedPrice: null,
    pricingMode: 'TIERED',
    cacheBilling: 'UNSUPPORTED',
    connections: [{ protocol: 'OPENAI', configuration: { baseUrl: 'https://example.com/v1', path: '/chat/completions' } }],
    headers: [],
    bodyDefaults: [{ key: 'temperature', value: '0.7' }],
    text: {
      input: {
        status: 'SUPPORTED',
        pricing: {
          kind: 'USAGE',
          usage: {
            mode: 'TIERED_TOKEN',
            tiers: [{ fromTokens: '0', toTokens: null, rate: { amount: '0.2', currency: 'USD', quantity: '1000000' } }],
          },
        },
      },
      output: {
        status: 'SUPPORTED',
        configuration: {
          thinking: {
            status: 'SUPPORTED',
            configuration: { effortMapping: [{ level: 'LOW', value: 'minimal' }], defaultEffort: 'LOW', budget: null },
          },
        },
      },
    },
    vision: { status: 'UNKNOWN' },
    pdf: { status: 'UNSUPPORTED' },
    audio: { input: { status: 'UNKNOWN' }, output: { status: 'UNSUPPORTED' } },
    video: { status: 'UNKNOWN' },
    imageGeneration: { status: 'UNSUPPORTED' },
    tools: { calling: { status: 'SUPPORTED' }, forcedChoice: { status: 'UNKNOWN' }, parallelCalls: { status: 'UNKNOWN' } },
    structuredOutput: { status: 'UNKNOWN' },
    search: { status: 'UNSUPPORTED' },
    billing: null,
    routeSelection: null,
  }
}
test('provider search and paging use the API, ignore stale results and retain rows after failures', async (t) => {
  const requests = []
  t.mock.method(
    globalThis,
    'fetch',
    (_url, init) =>
      new Promise((resolve, reject) => requests.push({ body: JSON.parse(init.body), signal: init.signal, resolve, reject })),
  )
  const { controller: c } = setup()
  t.after(c.stop)
  c.start()
  await tick()
  requests[0].resolve(list(['a'], true))
  await tick()
  assert.equal(c.store.getState().providersLoaded, true)
  assert.equal(c.loading.providers.store.getState().loading, false)
  const retained = c.store.getState().providers[0]
  c.onLoadMoreProviders()
  c.onLoadMoreProviders()
  await tick()
  assert.equal(requests.length, 2)
  requests[1].reject(new Error('offline'))
  await tick()
  assert.equal(c.store.getState().providers[0], retained)
  assert.equal(c.store.getState().providersHasMore, true)
  c.onRetryCatalog()
  await tick()
  assert.ok(JSON.stringify(requests[2].body.variables).includes('cursor-a'))
  requests[2].resolve(list(['a', 'b'], false))
  await tick()
  assert.deepEqual(
    c.store.getState().providers.map((v) => v.id),
    ['a', 'b'],
  )
  assert.equal(c.store.getState().providers[0], retained)
  const beforeTyping = requests.length
  c.onProviderQueryChanged('old')
  await tick()
  assert.equal(requests.length, beforeTyping)
  c.onProviderSearch()
  await tick()
  const obsolete = requests.at(-1)
  c.onProviderQueryChanged('latest')
  await tick()
  assert.equal(obsolete.signal.aborted, false)
  c.onProviderSearch()
  await tick()
  assert.equal(obsolete.signal.aborted, true)
  const latest = requests.at(-1)
  assert.ok(JSON.stringify(latest.body.variables).includes('latest'))
  latest.resolve(list(['latest']))
  await tick()
  obsolete.resolve(list(['old']))
  await tick()
  assert.deepEqual(
    c.store.getState().providers.map((v) => v.id),
    ['latest'],
  )
  c.onProviderQueryChanged('   ')
  await tick()
  const cleared = requests.at(-1)
  assert.ok(!JSON.stringify(cleared.body.variables).includes('latest'))
  cleared.resolve(list(['a', 'b']))
  await tick()
  assert.deepEqual(
    c.store.getState().providers.map((v) => v.id),
    ['a', 'b'],
  )
  assert.ok(requests.every((r) => !r.body.query.includes('mutation')))
})
test('provider/model navigation uses catalog detail, handles reload and cancels late model responses', async (t) => {
  const requests = []
  t.mock.method(
    globalThis,
    'fetch',
    (_url, init) => new Promise((resolve) => requests.push({ body: JSON.parse(init.body), signal: init.signal, resolve })),
  )
  const { controller: c, url } = setup()
  t.after(c.stop)
  c.start()
  await tick()
  requests[0].resolve(list(['a', 'b']))
  await tick()
  c.onSelectProvider('a')
  await tick()
  requests.at(-1).resolve(Response.json({ data: { effectiveProviderTemplate: { node: provider('a') } } }))
  await tick()
  assert.equal(c.store.getState().models[0].inputPrice, '')
  assert.equal(c.store.getState().models[0].catalogPriceLabels[0], 'Tiered')
  c.onEditModel('model-a')
  await tick()
  const pending = requests.at(-1)
  assert.ok(pending.body.query.includes('modelTemplate'))
  c.onSelectProvider('b')
  await tick()
  assert.equal(pending.signal.aborted, true)
  assert.equal(c.loading.model.store.getState().loading, false)
  pending.resolve(Response.json({ data: { effectiveModelTemplate: { node: model('model-a') } } }))
  await tick()
  assert.equal(c.store.getState().modelDraft, null)
  requests.at(-1).resolve(Response.json({ data: { effectiveProviderTemplate: { node: provider('b') } } }))
  await tick()
  c.onEditModel('model-b')
  await tick()
  for (const request of requests.filter((r) => /[mM]odelTemplate/.test(r.body.query) && !r.signal.aborted))
    request.resolve(
      Response.json({
        data: { effectiveModelTemplate: { node: model('model-b') }, modelTemplate: { node: model('model-b') } },
      }),
    )
  await tick()
  const draft = c.store.getState().modelDraft
  assert.equal(draft.name, 'model-b')
  assert.equal(draft.inputPrice, '')
  assert.equal(draft.inputPriceEnabled, false)
  assert.equal(draft.effortMapping.low, 'minimal')
  assert.equal(draft.bodyDefaults[0].value, '0.7')
  c.onModelDraftChanged({ maxContextTokens: '64000' })
  assert.equal(c.store.getState().modelDraft.maxContextTokens, '64000')
  assert.ok(requests.every((r) => !r.body.query.includes('mutation')))
  assert.ok(url.store.getState().search.includes('model=model-b'))
  assert.ok(!url.store.getState().search.includes('models='))
  assert.ok(!url.store.getState().search.includes('minimal'))
  c.onCancelModel()
  c.onModelDraftChanged({ name: 'late change' })
  assert.equal(await c.onSaveModel(draft), false)
  assert.equal(c.store.getState().modelDraft, null)
  assert.equal(c.store.getState().editingModelId, null)
  assert.ok(!url.store.getState().search.includes('model='))
  c.onEditModel('model-b')
  c.stop()
  c.start()
  await tick()
  assert.ok(requests.at(-1).body.query.includes('effectiveProviderTemplate'))
  requests.at(-1).resolve(Response.json({ data: { effectiveProviderTemplate: { node: provider('b') } } }))
  await tick()
  assert.ok(requests.at(-1).body.query.includes('modelTemplate'))
  const last = requests.at(-1)
  c.stop()
  assert.equal(last.signal.aborted, true)
})

test('a new model save finishing after Back does not reopen its configuration page', async (t) => {
  const requests = []
  t.mock.method(
    globalThis,
    'fetch',
    (_url, init) => new Promise((resolve) => requests.push({ body: JSON.parse(init.body), resolve })),
  )
  const { controller: c, url } = setup()
  t.after(c.stop)
  c.start()
  await tick()
  requests.at(-1).resolve(list(['a']))
  await tick()
  c.onSelectProvider('a')
  await tick()
  requests.at(-1).resolve(Response.json({ data: { effectiveProviderTemplate: { node: provider('a') } } }))
  await tick()
  c.onAddModel('openai_chat_completions')
  c.onModelDraftChanged({ name: 'new-text', maxContextTokens: '128000', inputPrice: '0.1', outputPrice: '0.2' })
  const saved = c.onSaveModel(c.store.getState().modelDraft)
  await tick()
  const pending = requests.at(-1)
  c.onCancelModel()
  assert.equal(new URLSearchParams(url.store.getState().search).has('model'), false)
  pending.resolve(
    Response.json({
      data: { modelTemplateCreate: { node: { ...model('custom:saved'), origin: 'CUSTOM', name: 'new-text' } } },
    }),
  )
  await saved
  assert.equal(new URLSearchParams(url.store.getState().search).has('model'), false)
  assert.equal(c.store.getState().editingModelId, null)
  assert.ok(c.store.getState().models.some((row) => row.name === 'new-text'))
})
