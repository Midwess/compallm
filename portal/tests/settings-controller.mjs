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

test('role and group add actions navigate to their creation screens', () => {
  const paths = []
  const url = { navigate: (path) => paths.push(path) }
  const controller = new SettingsController('Research team', url, { signOut() {} }, 'general')
  controller.navigate('new-group')
  controller.setPage('new-group')
  assert.equal(paths.at(-1), '/Research%20team/settings/new-group')
  assert.equal(controller.store.getState().location.page, 'new-group')
  assert.deepEqual(controller.store.getState().groupDraft, {
    id: null,
    name: '',
    description: '',
    memberIds: [],
    roleIds: [],
  })
  controller.navigate('new-role')
  controller.setPage('new-role')
  assert.equal(paths.at(-1), '/Research%20team/settings/new-role')
  assert.equal(controller.store.getState().location.page, 'new-role')
  assert.equal(controller.store.getState().roleDraft?.id, null)
})

test('direct visits initialize the requested creation screen', () => {
  for (const page of ['new-group', 'new-role']) {
    const controller = new SettingsController('Research team', { navigate() {} }, { signOut() {} }, page)
    const state = controller.store.getState()
    assert.equal(state.location.page, page)
    assert.ok(page === 'new-group' ? state.groupDraft : state.roleDraft)
  }
})
