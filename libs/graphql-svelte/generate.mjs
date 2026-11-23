import { fileURLToPath } from 'node:url'
import { readFile, copyFile, writeFile, readdir } from 'node:fs/promises'
import { spawnSync } from 'node:child_process'
import { buildSchema, isScalarType, isSpecifiedScalarType } from 'graphql'

const schema = buildSchema(await readFile(new URL('../contract/schema.graphql', import.meta.url), 'utf8'))
for (const type of Object.values(schema.getTypeMap())) {
  if (isScalarType(type) && !isSpecifiedScalarType(type) && !type.name.startsWith('__') && type.name !== 'ContextTokens') {
    throw new Error(`Unmapped Rust scalar: ${type.name}`)
  }
}
const result = spawnSync(process.execPath, [fileURLToPath(new URL('./run-houdini.mjs', import.meta.url))], {
  cwd: new URL('../../', import.meta.url),
  stdio: 'inherit',
})
if (result.error) throw result.error
if (result.status !== 0) throw new Error('Houdini generation failed; install libs/graphql-svelte dependencies first')
await copyFile(new URL('../contract/schema.graphql', import.meta.url), new URL('./.houdini/schema.graphql', import.meta.url))
const coverage = spawnSync(process.execPath, [fileURLToPath(new URL('./check-coverage.mjs', import.meta.url))], {
  cwd: new URL('../../', import.meta.url),
  stdio: 'inherit',
})
if (coverage.error) throw coverage.error
if (coverage.status !== 0) throw new Error('Houdini schema or Portal operation coverage is incomplete')

async function normalizeGeneratedFiles(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = new URL(entry.name, directory)
    if (entry.isDirectory()) await normalizeGeneratedFiles(new URL(`${entry.name}/`, directory))
    else if (/\.(?:ts|graphql|gql|json)$/.test(entry.name)) {
      const source = await readFile(path, 'utf8')
      await writeFile(path, source ? source.replace(/\s*$/, '') + '\n' : '')
    }
  }
}
await normalizeGeneratedFiles(new URL('./.houdini/', import.meta.url))
