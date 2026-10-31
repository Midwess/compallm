import { createHash } from 'node:crypto'
import { copyFile, mkdir, readFile, writeFile } from 'node:fs/promises'

// Font assets remain cacheable files instead of bloating the compiled stylesheet.
const output = new URL('../dist/', import.meta.url)
await mkdir(new URL('fonts/', output), { recursive: true })
const cssPath = new URL('styles.css', output)
let css = await readFile(cssPath, 'utf8')
const fonts = [...css.matchAll(/url\(data:font\/(woff2?);base64,([A-Za-z0-9+/=]+)\)/g)]
for (const [url, format, encoded] of fonts) {
  const bytes = Buffer.from(encoded, 'base64')
  const hash = createHash('sha256').update(bytes).digest('hex').slice(0, 12)
  const name = `fonts/inter-${hash}.${format}`
  await writeFile(new URL(name, output), bytes)
  css = css.replaceAll(url, `url(./${name})`)
}
await writeFile(cssPath, css)

await copyFile(new URL('../src/styles/theme.css', import.meta.url), new URL('../dist/theme.css', import.meta.url))
await mkdir(new URL('../dist/licenses/', import.meta.url), { recursive: true })
await copyFile(
  new URL('../node_modules/@fontsource/inter/LICENSE', import.meta.url),
  new URL('../dist/licenses/Inter-OFL.txt', import.meta.url),
)
