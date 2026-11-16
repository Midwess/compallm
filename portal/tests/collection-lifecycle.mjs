import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'

const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
})
after(() => server.close())
const { CollectionForms } = await server.ssrLoadModule('../libs/storybook/src/components/collection-forms.ts')
const { processInSlices } = await server.ssrLoadModule('../libs/storybook/src/components/scheduling.ts')

test('offscreen edits and in-flight saves survive row detachment without replaying writes', async () => {
  const registry = new CollectionForms()
  let resolve
  let writes = 0
  const form = registry.get(
    'member-1:role',
    { role: 'member' },
    {
      submit: (values) => {
        writes++
        return new Promise((done) => {
          resolve = () => done(values)
        })
      },
    },
  )
  const detach = registry.attach('member-1:role', form)
  form.change('role', 'admin')
  const saving = form.submit()
  await new Promise((done) => setImmediate(done))
  detach()
  assert.equal(form.pending, true)
  assert.equal(registry.get('member-1:role', { role: 'member' }, {}), form)
  assert.equal(form.get('role'), 'admin')
  resolve()
  await saving
  assert.equal(writes, 1)
  assert.equal(form.pending, false)
  assert.equal(registry.has('member-1:role'), false)
  registry.dispose()
})

test('collection teardown cancels pending operations and discards drafts for the previous identity', async () => {
  const registry = new CollectionForms()
  let signal
  const form = registry.get(
    'member-1:groups',
    { groups: ['a'] },
    {
      submit: (_values, _patch, requestSignal) => {
        signal = requestSignal
        return new Promise(() => {})
      },
    },
  )
  registry.attach('member-1:groups', form)
  form.change('groups', ['a', 'b'])
  void form.submit()
  await new Promise((done) => setImmediate(done))
  registry.dispose()
  assert.equal(signal.aborted, true)
  assert.equal(registry.has('member-1:groups'), false)
})

test('time-sliced work stops processing when its owning operation is cancelled', async () => {
  const abort = new AbortController()
  const processed = []
  await assert.rejects(
    processInSlices(
      [1, 2, 3],
      (value) => {
        processed.push(value)
        if (value === 2) abort.abort()
      },
      abort.signal,
    ),
    { name: 'AbortError' },
  )
  assert.deepEqual(processed, [1, 2])
})
