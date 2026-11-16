import assert from 'node:assert/strict'
import { after, test } from 'node:test'
import { createServer } from 'vite'
import { createElement, useState } from 'react'
import { renderToString } from 'react-dom/server'

const server = await createServer({
  root: new URL('..', import.meta.url).pathname,
  configFile: false,
  server: { middlewareMode: true, hmr: false, ws: false, watch: null },
  appType: 'custom',
  resolve: { dedupe: ['react', 'react-dom'] },
  ssr: { noExternal: ['@compallm/storybook'] },
})
after(() => server.close())
const { useProgressiveRows } = await server.ssrLoadModule('../libs/storybook/src/components/progressive-rows.ts')

test('directory rerenders with fresh arrays retain a bounded first batch without looping', () => {
  function Directory() {
    const [selected, select] = useState(false)
    const visible = useProgressiveRows(Array.from({ length: 20 }, (_, index) => ({ id: String(index) })))
    if (!selected) select(true)
    return createElement('div', null, visible.rows.map((row) => row.id).join(','))
  }
  assert.equal(renderToString(createElement(Directory)), '<div>0,1,2</div>')
})

test('a replacement search result replaces the pending batch with current identities', () => {
  function Directory() {
    const [query, search] = useState('old')
    const visible = useProgressiveRows(Array.from({ length: 20 }, (_, index) => ({ id: `${query}-${index}` })))
    if (query === 'old') search('new')
    return createElement('div', null, visible.rows.map((row) => row.id).join(','))
  }
  assert.equal(renderToString(createElement(Directory)), '<div>new-0,new-1,new-2</div>')
})
