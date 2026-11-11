import { spawn } from 'node:child_process'
import { readdir, readFile } from 'node:fs/promises'
import path from 'node:path'
import { fileURLToPath, pathToFileURL } from 'node:url'

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..')
const server = spawn(
  process.execPath,
  [path.join(root, 'node_modules/@tailwindcss/language-server/bin/tailwindcss-language-server'), '--stdio'],
  {
    cwd: root,
    stdio: ['pipe', 'pipe', 'inherit'],
  },
)
const settings = {
  tailwindCSS: {
    validate: true,
    lint: { cssConflict: 'warning', suggestCanonicalClasses: 'warning', recommendedVariantOrder: 'warning' },
    experimental: { configFile: { '.storybook/tailwind.css': ['src/**', 'stories/**', '.storybook/**'] } },
  },
  editor: { tabSize: 2 },
}
const documents = new Map()
let buffer = Buffer.alloc(0)
let pending
let nextId = 0
let completed = false
const timer = setTimeout(() => fail('Tailwind language server did not finish checking all files within 60 seconds.'), 60000)

function send(message) {
  const body = Buffer.from(JSON.stringify(message))
  server.stdin.write(`Content-Length: ${body.length}\r\n\r\n`)
  server.stdin.write(body)
}
function fail(message) {
  if (completed) return
  completed = true
  clearTimeout(timer)
  console.error(message)
  server.kill()
  process.exitCode = 1
}
async function filesIn(directory) {
  const entries = await readdir(directory, { withFileTypes: true })
  const paths = await Promise.all(
    entries.map(async (entry) => {
      const full = path.join(directory, entry.name)
      if (entry.isDirectory()) return filesIn(full)
      return /\.(?:tsx|jsx|ts|js|css)$/.test(entry.name) ? [full] : []
    }),
  )
  return paths.flat()
}
async function startDocuments() {
  const files = (await Promise.all(['src', 'stories', '.storybook'].map((dir) => filesIn(path.join(root, dir))))).flat()
  const content = await Promise.all(files.map(async (file) => ({ file, text: await readFile(file, 'utf8') })))
  pending = files.length
  for (const { file } of content) documents.set(pathToFileURL(file).href, null)
  for (const { file, text } of content) {
    const uri = pathToFileURL(file).href
    const languageId = file.endsWith('.tsx')
      ? 'typescriptreact'
      : file.endsWith('.jsx')
        ? 'javascriptreact'
        : file.endsWith('.css')
          ? 'css'
          : file.endsWith('.ts')
            ? 'typescript'
            : 'javascript'
    send({
      jsonrpc: '2.0',
      method: 'textDocument/didOpen',
      params: {
        textDocument: { uri, languageId, version: 1, text },
      },
    })
  }
  if (pending === 0) finish()
}
function finish() {
  if (completed) return
  completed = true
  clearTimeout(timer)
  let count = 0
  for (const [uri, diagnostics] of documents) {
    for (const diagnostic of diagnostics ?? []) {
      count++
      const { line, character } = diagnostic.range.start
      console.error(
        `${path.relative(root, fileURLToPath(uri))}:${line + 1}:${character + 1} ${diagnostic.message} (${diagnostic.code})`,
      )
    }
  }
  if (count) process.exitCode = 1
  else console.log(`Tailwind: no diagnostics in ${documents.size} files`)
  send({ jsonrpc: '2.0', id: ++nextId, method: 'shutdown', params: null })
  send({ jsonrpc: '2.0', method: 'exit', params: null })
  server.stdin.end()
}
server.on('error', (error) => fail(`Could not start Tailwind language server: ${error.message}`))
server.on('exit', (code) => {
  if (!completed) fail(`Tailwind language server exited unexpectedly (${code}).`)
})
server.stdout.on('data', (chunk) => {
  buffer = Buffer.concat([buffer, chunk])
  while (true) {
    const end = buffer.indexOf('\r\n\r\n')
    if (end < 0) break
    const match = /Content-Length: (\d+)/i.exec(buffer.subarray(0, end).toString())
    if (!match) return fail('Invalid Tailwind language server response.')
    const length = Number(match[1])
    if (buffer.length < end + 4 + length) break
    const message = JSON.parse(buffer.subarray(end + 4, end + 4 + length).toString())
    buffer = buffer.subarray(end + 4 + length)
    if (message.id === 1 && !message.method) {
      if (message.error) return fail(`Tailwind initialization failed: ${message.error.message}`)
      send({ jsonrpc: '2.0', method: 'initialized', params: {} })
      void startDocuments().catch((error) => fail(error.message))
    } else if (message.id !== undefined && message.method === 'workspace/configuration') {
      const values = message.params.items.map(({ section }) => (section ? (settings[section] ?? null) : settings))
      send({ jsonrpc: '2.0', id: message.id, result: values })
    } else if (message.id !== undefined && message.method) {
      send({ jsonrpc: '2.0', id: message.id, result: null })
    } else if (message.method === 'textDocument/publishDiagnostics' && documents.has(message.params.uri)) {
      if (documents.get(message.params.uri) === null) pending--
      documents.set(message.params.uri, message.params.diagnostics)
      if (pending === 0) finish()
    }
  }
})
send({
  jsonrpc: '2.0',
  id: ++nextId,
  method: 'initialize',
  params: {
    processId: process.pid,
    rootUri: pathToFileURL(root).href,
    capabilities: { workspace: { configuration: true }, textDocument: { publishDiagnostics: {} } },
    workspaceFolders: [{ uri: pathToFileURL(root).href, name: 'storybook' }],
  },
})
