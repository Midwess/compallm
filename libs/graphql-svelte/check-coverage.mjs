import { readFile, readdir } from 'node:fs/promises'
import { fileURLToPath } from 'node:url'
import { resolve, relative } from 'node:path'
import ts from 'typescript'
import { buildSchema, parse, validate, isInputObjectType, isEnumType, isObjectType } from 'graphql'

const root = fileURLToPath(new URL('../../', import.meta.url))
const manifest = JSON.parse(await readFile(new URL('../../portal-svelte/operation-coverage.json', import.meta.url), 'utf8'))
const schema = buildSchema(await readFile(new URL('../contract/schema.graphql', import.meta.url), 'utf8'))
const expected = new Map()
async function inspect(directory) {
  for (const item of await readdir(directory, { withFileTypes: true })) {
    const path = resolve(directory, item.name)
    if (item.isDirectory()) await inspect(path)
    else if (/\.tsx?$/.test(item.name)) {
      const source = ts.createSourceFile(path, await readFile(path, 'utf8'), ts.ScriptTarget.Latest, true)
      const visit = (node) => {
        if (ts.isSatisfiesExpression(node) && /(?:Query|Mutation)GenqlSelection$/.test(node.type.getText(source))) {
          const line = source.getLineAndCharacterOfPosition(node.expression.getStart(source)).line + 1
          expected.set(`${relative(root, path)}:${line}`, 0)
        }
        ts.forEachChild(node, visit)
      }
      visit(source)
    }
  }
}
await inspect(resolve(root, 'portal/src'))
const names = new Set()
for (const entry of manifest) {
  const key = `${entry.source}:${entry.line}`
  if (!expected.has(key)) throw new Error(`Stale operation coverage: ${key}`)
  expected.set(key, expected.get(key) + 1)
  if (names.has(entry.operation)) throw new Error(`Duplicate operation: ${entry.operation}`)
  names.add(entry.operation)
  const document = parse(await readFile(resolve(root, entry.document), 'utf8'))
  const errors = validate(schema, document)
  if (errors.length) throw new Error(`${entry.operation}: ${errors.map((error) => error.message).join('; ')}`)
  const operation = document.definitions.find((definition) => definition.kind === 'OperationDefinition')
  if (operation?.name?.value !== entry.operation || operation.operation !== entry.kind)
    throw new Error(`Mismatched document: ${entry.document}`)
  if (operation.operation === 'mutation' && operation.selectionSet.selections.length !== 1)
    throw new Error(`Multiple mutation roots: ${entry.operation}`)
}
for (const [source, count] of expected) if (count !== 1) throw new Error(`Missing or repeated operation coverage: ${source}`)
console.log(`Covered ${manifest.length} Portal selections with validated Houdini documents`)

const inputSource = ts.createSourceFile(
  'inputs.ts',
  await readFile(new URL('./.houdini/graphql/inputs.ts', import.meta.url), 'utf8'),
  ts.ScriptTarget.Latest,
  true,
)
const inputNames = new Set(inputSource.statements.filter(ts.isTypeAliasDeclaration).map((node) => node.name.text))
const enumSource = await readFile(new URL('./.houdini/graphql/enums.ts', import.meta.url), 'utf8')
const generated = ts.createSourceFile(
  'generated.ts',
  await readFile(new URL('./.houdini/runtime/generated.ts', import.meta.url), 'utf8'),
  ts.ScriptTarget.Latest,
  true,
)
const cache = generated.statements.find((node) => ts.isTypeAliasDeclaration(node) && node.name.text === 'CacheTypeDef')
const types = cache.type.members.find((node) => node.name.getText(generated) === 'types').type.members
const outputs = new Map(types.map((node) => [node.name.getText(generated), node]))
let inputs = 0,
  enums = 0,
  objects = 0
for (const type of Object.values(schema.getTypeMap())) {
  if (type.name.startsWith('__')) continue
  if (isInputObjectType(type)) {
    if (!inputNames.has(type.name)) throw new Error(`Missing Houdini input type: ${type.name}`)
    inputs++
  }
  if (isEnumType(type)) {
    if (!enumSource.includes(`export const ${type.name} =`)) throw new Error(`Missing Houdini enum: ${type.name}`)
    enums++
  }
  if (isObjectType(type)) {
    const output = outputs.get(type === schema.getQueryType() ? '__ROOT__' : type.name)
    if (!output) throw new Error(`Missing Houdini cache type: ${type.name}`)
    const fields = new Set(
      output.type.members
        .find((node) => node.name.getText(generated) === 'fields')
        .type.members.map((node) => node.name.getText(generated)),
    )
    for (const name of Object.keys(type.getFields()))
      if (!fields.has(name)) throw new Error(`Missing Houdini cache field: ${type.name}.${name}`)
    objects++
  }
}
console.log(`Full schema coverage: ${inputs} input objects, ${enums} enums, ${objects} output objects with all fields`)
