import { readFile, readdir } from 'node:fs/promises'
import ts from 'typescript'
import { buildSchema, parse, validate, isInputObjectType, isEnumType, isObjectType } from 'graphql'

const schema = buildSchema(await readFile(new URL('../schema.graphql', import.meta.url), 'utf8'))
const names = new Set()
let documents = 0
async function inspect(directory) {
  for (const item of await readdir(directory, { withFileTypes: true })) {
    const path = new URL(item.name + (item.isDirectory() ? '/' : ''), directory)
    if (item.isDirectory()) await inspect(path)
    else if (item.name.endsWith('.graphql')) {
      const document = parse(await readFile(path, 'utf8'))
      const errors = validate(schema, document)
      if (errors.length) throw new Error(`${path}: ${errors.map((error) => error.message).join('; ')}`)
      for (const operation of document.definitions.filter((definition) => definition.kind === 'OperationDefinition')) {
        const name = operation.name?.value
        if (!name || names.has(name)) throw new Error(`Missing or duplicate operation name: ${path}`)
        names.add(name)
        if (operation.operation === 'mutation' && operation.selectionSet.selections.length !== 1)
          throw new Error(`Multiple mutation roots: ${name}`)
      }
      documents++
    }
  }
}
await inspect(new URL('./operations/', import.meta.url))
console.log(`Validated ${documents} Houdini operation documents`)

const inputSource = ts.createSourceFile(
  'inputs.ts',
  await readFile(new URL('./generated/graphql/inputs.ts', import.meta.url), 'utf8'),
  ts.ScriptTarget.Latest,
  true,
)
const inputNames = new Set(inputSource.statements.filter(ts.isTypeAliasDeclaration).map((node) => node.name.text))
const enumSource = await readFile(new URL('./generated/graphql/enums.ts', import.meta.url), 'utf8')
const generated = ts.createSourceFile(
  'generated.ts',
  await readFile(new URL('./generated/runtime/generated.ts', import.meta.url), 'utf8'),
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
