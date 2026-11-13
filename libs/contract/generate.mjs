import { readFile, writeFile, readdir, rm } from 'node:fs/promises'
import { existsSync } from 'node:fs'
import { dirname, resolve } from 'node:path'
import { fileURLToPath } from 'node:url'
import ts from 'typescript'
import { spawnSync } from 'node:child_process'
import { format } from 'prettier'
import { checkValidationCompatibility } from './validation-compatibility.mjs'
import {
  parse,
  print,
  visit as visitAst,
  buildASTSchema,
  typeFromAST,
  isNonNullType,
  isListType,
  isEnumType,
  isInputObjectType,
} from 'graphql'

async function writeTypescript(path, source) {
  await writeFile(
    path,
    await format(source, {
      parser: 'typescript',
      singleQuote: true,
      semi: false,
      printWidth: 120,
    }),
  )
}

await writeTypescript(new URL('./preference.ts', import.meta.url), await readFile(new URL('./preference.ts', import.meta.url), 'utf8'))

const banner = '// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.\n'
const document = JSON.parse(await readFile(new URL('./validation.schema.json', import.meta.url), 'utf8'))
const definitions = document.$defs
if (!definitions) throw new Error('Missing Rust input definitions')
// Juniper's enum ToInputValue uses a string scalar, which as_sdl prints as a
// quoted default. Restore enum literals using the Rust-exported field type.
// This changes encoding only; defaults and enum members still come from Rust.
const schemaUrl = new URL('./schema.graphql', import.meta.url)
const ast = parse(await readFile(schemaUrl, 'utf8'))
const exportedSchema = buildASTSchema(ast)
function normalizeDefault(value, type) {
  if (isNonNullType(type)) return normalizeDefault(value, type.ofType)
  if (isEnumType(type) && value.kind === 'StringValue') {
    if (!type.getValue(value.value)) throw new Error(`Unknown enum default for ${type.name}`)
    return { ...value, kind: 'EnumValue' }
  }
  if (isListType(type) && value.kind === 'ListValue')
    return {
      ...value,
      values: value.values.map((item) => normalizeDefault(item, type.ofType)),
    }
  if (isInputObjectType(type) && value.kind === 'ObjectValue')
    return {
      ...value,
      fields: value.fields.map((field) => ({
        ...field,
        value: normalizeDefault(field.value, type.getFields()[field.name.value].type),
      })),
    }
  return value
}
const normalizedSchema = (
  print(
    visitAst(ast, {
      InputValueDefinition(node) {
        return node.defaultValue
          ? {
              ...node,
              defaultValue: normalizeDefault(node.defaultValue, typeFromAST(exportedSchema, node.type)),
            }
          : node
      },
    }),
  ) + '\n'
).replace(/[\t ]+$/gm, '')
await writeFile(schemaUrl, normalizedSchema)
const compatibility = checkValidationCompatibility(normalizedSchema, document, {
  ContextTokens: 'integer',
})
console.log(`Validation compatibility: ${compatibility.checked.length} registered GraphQL definitions checked`)

const ordered = []
const visiting = new Set()
function visit(name) {
  if (ordered.includes(name)) return
  if (visiting.has(name)) throw new Error(`Recursive form contract requires explicit support: ${name}`)
  if (!definitions[name]) throw new Error(`Unknown input reference: ${name}`)
  visiting.add(name)
  function walk(value) {
    if (!value || typeof value !== 'object') return
    if (value.$ref) visit(value.$ref.split('/').at(-1))
    for (const child of Object.values(value)) walk(child)
  }
  walk(definitions[name])
  visiting.delete(name)
  ordered.push(name)
}
Object.keys(definitions).sort().forEach(visit)
const formats = new Set(['baseUrl', 'nonblank', 'authorization', 'emailAddress', 'resourceId'])
const objectRules = {
  header: 'headerObject',
  headerUpdate: 'headerUpdateObject',
  createProvider: 'createProviderObject',
  createProviders: 'createProvidersObject',
  providerPatch: 'providerPatchObject',
}
const arrayRules = {
  headers: '(entries) => headersObject({entries})',
  modelWrites: 'modelWritesArray',
}
const supported = new Set([
  '$ref',
  'type',
  'title',
  'description',
  'properties',
  'required',
  'additionalProperties',
  'items',
  'enum',
  'const',
  'default',
  'anyOf',
  'oneOf',
  'minimum',
  'maximum',
  'format',
  'minLength',
  'maxLength',
  'pattern',
  'minItems',
  'maxItems',
  'x-compallm-input',
  'x-compallm-non-null',
  'x-compallm-format',
  'x-compallm-trim',
  'x-compallm-cedar-compact',
  'x-compallm-ascii-lowercase',
  'x-compallm-email-policy',
  'x-compallm-input-pattern',
  'x-compallm-object-rule',
  'x-compallm-array-rule',
  'x-compallm-messages',
])
function validate(schema) {
  if (!schema || typeof schema !== 'object' || Array.isArray(schema)) throw new Error('Unsupported input schema')
  for (const key of Object.keys(schema))
    if (!supported.has(key)) throw new Error(`Unsupported JSON Schema keyword: ${key}`)
  if (schema['x-compallm-format'] && !formats.has(schema['x-compallm-format']))
    throw new Error('Unknown custom input format')
  if (schema['x-compallm-object-rule'] && !objectRules[schema['x-compallm-object-rule']])
    throw new Error('Unknown object rule')
  if (schema['x-compallm-array-rule'] && !arrayRules[schema['x-compallm-array-rule']])
    throw new Error('Unknown array rule')
  if (schema['x-compallm-messages'] !== undefined) {
    const messages = schema['x-compallm-messages']
    if (
      !messages ||
      typeof messages !== 'object' ||
      Array.isArray(messages) ||
      Object.values(messages).some((message) => typeof message !== 'string')
    )
      throw new Error('Validation messages must map constraint names to strings')
  }
  if (schema.properties) Object.values(schema.properties).forEach(validate)
  if (schema.items) validate(schema.items)
  if (schema.anyOf) schema.anyOf.forEach(validate)
  if (schema.oneOf) schema.oneOf.forEach(validate)
}
Object.values(definitions).forEach(validate)
function unwrapped(schema) {
  if (schema.$ref)
    return {
      ...unwrapped(definitions[schema.$ref.split('/').at(-1)]),
      ...schema,
    }
  if (schema.anyOf) {
    const values = schema.anyOf.filter((value) => value.type !== 'null')
    if (values.length !== 1 || values.length === schema.anyOf.length) throw new Error('Unsupported input union')
    return { ...schema, anyOf: undefined, ...values[0] }
  }
  if (Array.isArray(schema.type)) {
    const types = schema.type.filter((type) => type !== 'null')
    if (types.length !== 1) throw new Error('Unsupported input type union')
    return { ...schema, type: types[0] }
  }
  return schema
}
function constraints(original) {
  const schema = unwrapped(original)
  const output = {}
  for (const key of ['minLength', 'maxLength', 'pattern']) if (schema[key] !== undefined) output[key] = schema[key]
  for (const [source, target] of [
    ['x-compallm-messages', 'messages'],
    ['x-compallm-input-pattern', 'inputPattern'],
    ['x-compallm-format', 'format'],
    ['x-compallm-trim', 'trim'],
    ['x-compallm-cedar-compact', 'cedarCompact'],
    ['x-compallm-ascii-lowercase', 'asciiLowercase'],
    ['x-compallm-email-policy', 'emailPolicy'],
  ])
    if (schema[source] !== undefined) output[target] = schema[source]
  return output
}
function nullable(schema) {
  return (
    schema.type === 'null' || schema.type?.includes?.('null') || schema.anyOf?.some((value) => value.type === 'null')
  )
}
function render(schema) {
  const value = unwrapped(schema)
  let result
  if (value.$ref) result = value.$ref.split('/').at(-1)
  else if (value.oneOf?.every((variant) => typeof variant.const === 'string'))
    result = `z.enum(${JSON.stringify(value.oneOf.map((variant) => variant.const))})`
  else if (value.enum) result = `z.enum(${JSON.stringify(value.enum)})`
  else if (value.const !== undefined) result = `z.literal(${JSON.stringify(value.const)})`
  else if (value.type === 'string') {
    if (value.format) throw new Error(`String format requires an explicit runtime mapping: ${value.format}`)
    result = `inputString(${JSON.stringify(constraints(schema))})`
  } else if (value.type === 'boolean') result = 'z.boolean()'
  else if (value.type === 'integer' || value.type === 'number') {
    if (value.format && !['uint32', 'uint64', 'int32', 'int64', 'double', 'float'].includes(value.format))
      throw new Error(`Unsupported numeric format: ${value.format}`)
    if (['uint64', 'int64'].includes(value.format))
      throw new Error('Large integers require an explicit string contract')
    result = value.type === 'integer' ? 'z.int()' : 'z.number()'
    for (const [key, method] of [
      ['minimum', 'min'],
      ['maximum', 'max'],
    ])
      if (value[key] !== undefined)
        result += `.${method}(${JSON.stringify(value[key])}${value['x-compallm-messages']?.[key] ? `, {message: ${JSON.stringify(value['x-compallm-messages'][key])}}` : ''})`
  } else if (value.type === 'array') {
    result = `z.array(${render(value.items)})`
    for (const [key, method] of [
      ['minItems', 'min'],
      ['maxItems', 'max'],
    ])
      if (value[key] !== undefined)
        result += `.${method}(${value[key]}${value['x-compallm-messages']?.[key] ? `, {message: ${JSON.stringify(value['x-compallm-messages'][key])}}` : ''})`
  } else if (value.type === 'object') {
    if (value.additionalProperties !== false) throw new Error('Input objects must explicitly reject unknown fields')
    const fields = Object.entries(value.properties ?? {}).map(([name, property]) => {
      let field = render(property)
      if (!value.required?.includes(name) && property.default === undefined) field += '.optional()'
      return `${JSON.stringify(name)}: ${field}`
    })
    result = `z.object({${fields.join(',')}}).strict()`
  } else throw new Error(`Unsupported input shape: ${JSON.stringify(value)}`)
  if (nullable(schema) && !schema['x-compallm-non-null']) result += '.nullable()'
  if (schema['x-compallm-array-rule'])
    result += `.refine(${arrayRules[schema['x-compallm-array-rule']]}, {message: ${JSON.stringify(schema['x-compallm-messages']?.['x-compallm-array-rule'] ?? 'Invalid input')}})`
  if (schema['x-compallm-object-rule'])
    result += `.refine(${objectRules[schema['x-compallm-object-rule']]}, {message: ${JSON.stringify(schema['x-compallm-messages']?.['x-compallm-object-rule'] ?? 'Invalid input')}})`
  if (schema.default !== undefined && !(schema['x-compallm-non-null'] && schema.default === null))
    result += `.default(${JSON.stringify(schema.default)})`
  if (schema.description) result += `.describe(${JSON.stringify(schema.description)})`
  return result
}
const lines = [
  banner,
  "import { z } from 'zod'",
  "import { inputString, stringInput, integerInput, headerObject, headersObject, headerUpdateObject, modelWritesArray, createProviderObject, createProvidersObject, providerPatchObject } from './runtime.ts'",
]
for (const name of ordered) {
  const schema = definitions[name]
  const fieldConstraints = {}
  const guards = []
  for (const [key, field] of Object.entries(schema.properties ?? {})) {
    const rule = constraints(field)
    if (Object.keys(rule).length) {
      fieldConstraints[key] = rule
      guards.push(`${JSON.stringify(key)}: stringInput(${name}Constraints[${JSON.stringify(key)}])`)
    } else if (unwrapped(field).type === 'integer') {
      const numeric = Object.fromEntries(
        ['minimum', 'maximum']
          .filter((key) => unwrapped(field)[key] !== undefined)
          .map((key) => [key, unwrapped(field)[key]]),
      )
      guards.push(`${JSON.stringify(key)}: integerInput(${JSON.stringify(numeric)})`)
    }
  }
  if (schema.properties) {
    lines.push(`export const ${name}Constraints = ${JSON.stringify(fieldConstraints)} as const`)
    lines.push(`export const ${name}Input = {${guards.join(',')}}`)
  }
  if (schema.description) lines.push(`/** ${schema.description.replaceAll('*/', '* /')} */`)
  lines.push(`export const ${name} = ${render(schema)}`)
  lines.push(`export type ${name}FormInput = z.input<typeof ${name}>`)
  lines.push(`export type ${name}FormOutput = z.output<typeof ${name}>`)
}
const output = lines.join('\n') + '\n'
await writeTypescript(new URL('./validation.ts', import.meta.url), output)
const generated = spawnSync(process.platform === 'win32' ? 'npm.cmd' : 'npm', ['run', 'generate:graphql'], {
  cwd: new URL('.', import.meta.url),
  stdio: 'inherit',
})
if (generated.status !== 0) throw new Error('Genql CLI generation failed')
// Genql emits extensionless imports. Normalize generated source so TypeScript's
// rewriteRelativeImportExtensions produces valid ESM, including directory imports.
async function normalizeGeneratedImports(directory) {
  for (const entry of await readdir(directory, { withFileTypes: true })) {
    const path = resolve(directory, entry.name)
    if (entry.isDirectory()) {
      await normalizeGeneratedImports(path)
      continue
    }
    if (entry.name.endsWith('.graphql')) {
      const schema = await format(await readFile(path, 'utf8'), {
        parser: 'graphql',
      })
      await writeFile(path, schema.replace(/[\t ]+$/gm, ''))
      continue
    }
    if (!entry.name.endsWith('.ts')) continue
    let source = await readFile(path, 'utf8')
    const ast = ts.createSourceFile(path, source, ts.ScriptTarget.Latest, true)
    const replacements = []
    for (const statement of ast.statements) {
      if (!(ts.isImportDeclaration(statement) || ts.isExportDeclaration(statement))) continue
      const specifier = statement.moduleSpecifier
      if (!specifier || !ts.isStringLiteral(specifier) || !specifier.text.startsWith('.')) continue
      const name = specifier.text
      if (/\.[cm]?[jt]s$/.test(name)) continue
      const target = resolve(dirname(path), name)
      const suffix = existsSync(`${target}.ts`)
        ? '.ts'
        : existsSync(resolve(target, 'index.ts'))
          ? '/index.ts'
          : undefined
      if (!suffix) throw new Error(`Unresolved generated import: ${name} in ${path}`)
      replacements.push({
        start: specifier.getStart(ast) + 1,
        end: specifier.getEnd() - 1,
        value: name + suffix,
      })
    }
    for (const change of replacements.reverse())
      source = source.slice(0, change.start) + change.value + source.slice(change.end)
    await writeTypescript(path, source)
  }
}
await normalizeGeneratedImports(fileURLToPath(new URL('./graphql', import.meta.url)))

await writeTypescript(new URL('./graphql.ts', import.meta.url), banner + "export * from './graphql/index.ts'\n")
// Clear compiled artifacts so removed legacy modules cannot survive a generation.
await rm(new URL('./dist', import.meta.url), { recursive: true, force: true })
const build = spawnSync(process.platform === 'win32' ? 'npm.cmd' : 'npm', ['run', 'build'], {
  cwd: new URL('.', import.meta.url),
  stdio: 'inherit',
})
if (build.status !== 0) throw new Error('Generated contract TypeScript build failed')
