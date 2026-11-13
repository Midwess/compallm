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
const { createForm } = await server.ssrLoadModule('../libs/storybook/src/components/form-model.ts')

test('button-only form retains edits without automatic writes', async () => {
  const writes = []
  const form = createForm(
    { name: 'Original', description: 'Existing' },
    {
      mode: 'button',
      fields: { name: { mode: 'change' } },
      submit: async (_values, patch) => {
        writes.push(patch)
        return patch
      },
    },
  )
  form.change('name', 'Changed')
  form.blur('name')
  form.enter('name')
  assert.equal(writes.length, 0)
  assert.equal(form.values.name, 'Changed')
  await form.submit()
  assert.deepEqual(writes, [{ name: 'Changed' }])
  form.stop()
})

test('returned values never commit or overwrite newer unsubmitted typing', async () => {
  let finish
  const writes = []
  const form = createForm(
    { name: 'Original', description: 'Existing' },
    {
      mode: 'blur',
      submit: (_values, patch) => {
        writes.push(patch)
        return new Promise((resolve) => {
          finish = resolve
        })
      },
    },
  )
  form.change('name', 'First')
  const pending = form.commit(['name'])
  form.change('name', 'Newer draft')
  finish({ name: 'Normalized first' })
  await pending
  assert.equal(form.values.name, 'Newer draft')
  assert.equal(form.baseline.name, 'Normalized first')
  assert.equal(writes.length, 1)
  assert.equal(form.values.description, 'Existing')
  form.stop()
})

test('rejection retains queued drafts and retry reads their latest values', async () => {
  let fail
  let attempts = 0
  const writes = []
  const form = createForm(
    { name: 'Original', description: 'Existing' },
    {
      mode: 'blur',
      submit: (_values, patch) => {
        writes.push(patch)
        if (attempts++ === 0)
          return new Promise((_resolve, reject) => {
            fail = reject
          })
        return patch
      },
    },
  )
  form.change('name', 'First')
  const pending = form.commit(['name'])
  form.change('description', 'Queued description')
  await form.commit(['description'])
  fail(new Error('Rejected'))
  await pending
  form.change('name', 'Repaired')
  assert.equal(writes.length, 1)
  await form.retry()
  assert.deepEqual(writes[1], { name: 'Repaired', description: 'Queued description' })
  assert.equal(form.error, null)
  form.stop()
})

test('dialog scope shares values but cannot commit the parent boundary', async () => {
  const parentWrites = [],
    dialogWrites = []
  const parent = createForm(
    { name: 'Provider', dialog: { name: '' } },
    {
      mode: 'change',
      submit: (_values, patch) => {
        parentWrites.push(patch)
        return patch
      },
    },
  )
  const dialog = parent.scope(['dialog'], {
    mode: 'button',
    command: true,
    submit: (_values, patch) => {
      dialogWrites.push(patch)
      return patch
    },
  })
  dialog.change('dialog.name', 'Model')
  assert.equal(parent.values.dialog.name, 'Model')
  assert.equal(dialog.has('name'), false)
  assert.equal(parentWrites.length, 0)
  await dialog.submit()
  assert.deepEqual(dialogWrites, [{ dialog: { name: 'Model' } }])
  assert.equal(parentWrites.length, 0)
  dialog.stop()
  parent.stop()
})

test('independent commit groups settle separately when a later group fails', async () => {
  const writes = []
  const form = createForm(
    { name: 'Original', source: 'Original' },
    {
      mode: 'blur',
      fields: { name: { group: 'metadata' }, source: { group: 'policy' } },
      submit: async (_values, patch) => {
        writes.push(patch)
        if (patch.source) throw new Error('Rejected')
        return patch
      },
    },
  )
  form.change('name', 'Updated')
  form.change('source', 'Policy')
  await form.submit()
  assert.deepEqual(writes, [{ name: 'Updated' }, { source: 'Policy' }])
  assert.equal(form.baseline.name, 'Updated')
  assert.equal(form.values.source, 'Policy')
  assert.equal(form.baseline.source, 'Original')
  form.stop()
})
