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
const { CedarWorkerClient } = await server.ssrLoadModule('/src/workers/cedar-client.ts')
class WorkerFixture {
  messages = []
  terminated = false
  postMessage(message) {
    this.messages.push(message)
  }
  terminate() {
    this.terminated = true
  }
  reply(message) {
    this.onmessage?.({ data: message })
  }
}
const settle = () => new Promise((resolve) => setImmediate(resolve))

test('worker requests correlate out-of-order replies and discard cancelled work', async () => {
  const worker = new WorkerFixture()
  const client = new CedarWorkerClient(() => worker)
  const first = client.request('formatPolicySource', ['first'])
  const second = client.request('formatPolicySource', ['second'])
  assert.equal(worker.messages.length, 0)
  worker.reply({ type: 'ready' })
  await settle()
  const [one, two] = worker.messages
  worker.reply({ type: 'result', id: two.id, result: 'formatted second' })
  worker.reply({ type: 'result', id: one.id, result: 'formatted first' })
  assert.equal(await first, 'formatted first')
  assert.equal(await second, 'formatted second')
  const abort = new AbortController()
  const pending = client.request('projectPolicy', ['draft', []], abort.signal)
  const rejected = assert.rejects(pending, { name: 'AbortError' })
  await settle()
  const request = worker.messages.at(-1)
  abort.abort()
  await rejected
  assert.deepEqual(worker.messages.at(-1), { type: 'cancel', id: request.id })
  worker.reply({ type: 'result', id: request.id, result: { projection: [], diagnostics: [] } })
  client.stop()
  assert.equal(worker.terminated, true)
})

test('worker failures reject pending work and later requests start a fresh worker', async () => {
  const workers = []
  const client = new CedarWorkerClient(() => {
    const worker = new WorkerFixture()
    workers.push(worker)
    return worker
  })
  const pending = client.request('formatPolicySource', ['draft'])
  const rejected = assert.rejects(pending)
  workers[0].reply({ type: 'fatal', message: 'initialization failed' })
  await rejected
  assert.equal(workers[0].terminated, true)
  const retry = client.request('formatPolicySource', ['retained draft'])
  workers[1].reply({ type: 'ready' })
  await settle()
  const request = workers[1].messages[0]
  workers[1].reply({ type: 'result', id: request.id, result: 'retained draft' })
  assert.equal(await retry, 'retained draft')
  const outstanding = client.request('formatPolicySource', ['another draft'])
  const stopped = assert.rejects(outstanding)
  workers[1].onerror({ preventDefault() {} })
  await stopped
  assert.equal(workers[1].terminated, true)
  client.stop()
})
