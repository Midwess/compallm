import { readFile, mkdir, writeFile } from 'node:fs/promises'

const output = new URL('../storybook-static/', import.meta.url)
const html = await readFile(new URL('page-preview/route-preview.html', output), 'utf8')
for (const path of ['signup', 'signin']) {
  await mkdir(new URL(`${path}/`, output), { recursive: true })
  await writeFile(new URL(`${path}/index.html`, output), html.replaceAll('./assets/', '../page-preview/assets/'))
}
