import { readFile, writeFile } from 'node:fs/promises'
import { spawnSync } from 'node:child_process'
import { generate } from '@graphql-codegen/cli'
import { format } from 'prettier'

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

const banner = '// Generated from Rust. Do not edit; run cargo run -p compallm-api -- generate.\n'
const document = JSON.parse(await readFile(new URL('./validation.schema.json', import.meta.url), 'utf8'))
const definitions = document.$defs
if (!definitions) throw new Error('Missing Rust input definitions')
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
const objectRules = {}
const arrayRules = {}
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
  'x-compallm-ascii-lowercase',
  'x-compallm-email-policy',
  'x-compallm-input-pattern',
  'x-compallm-object-rule',
  'x-compallm-array-rule',
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
  if (schema.properties) Object.values(schema.properties).forEach(validate)
  if (schema.items) validate(schema.items)
  if (schema.anyOf) schema.anyOf.forEach(validate)
  if (schema.oneOf) schema.oneOf.forEach(validate)
}
Object.values(definitions).forEach(validate)
function unwrapped(schema) {
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
    ['x-compallm-input-pattern', 'inputPattern'],
    ['x-compallm-format', 'format'],
    ['x-compallm-trim', 'trim'],
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
      if (value[key] !== undefined) result += `.${method}(${JSON.stringify(value[key])})`
  } else if (value.type === 'array') {
    result = `z.array(${render(value.items)})`
    for (const [key, method] of [
      ['minItems', 'min'],
      ['maxItems', 'max'],
    ])
      if (value[key] !== undefined) result += `.${method}(${value[key]})`
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
    result += `.refine(${arrayRules[schema['x-compallm-array-rule']]}, {message: 'Invalid input'})`
  if (schema['x-compallm-object-rule'])
    result += `.refine(${objectRules[schema['x-compallm-object-rule']]}, {message: 'Invalid input'})`
  if (schema.default !== undefined && !(schema['x-compallm-non-null'] && schema.default === null))
    result += `.default(${JSON.stringify(schema.default)})`
  if (schema.description) result += `.describe(${JSON.stringify(schema.description)})`
  return result
}
const lines = [
  banner,
  "export * from './offline-provider-validation.ts'",
  "import { z } from 'zod'",
  "import { inputString, stringInput, integerInput } from './runtime.ts'",
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
        ['minimum', 'maximum'].filter((key) => field[key] !== undefined).map((key) => [key, field[key]]),
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
  lines.push(`export type ${name} = z.input<typeof ${name}>`)
  lines.push(`export type ${name}Output = z.output<typeof ${name}>`)
}
// Compatibility for offline consumers: only delivery naming changes. Rules come
// exclusively from the actual exported GraphQL input definitions above.
lines.push(
  "export const TokenDelivery = z.enum(CredentialDelivery.options.map((value) => value.toLowerCase()) as ['cookie', 'token'])",
)
for (const [name, source] of [
  ['SignupEmailRequest', 'EmailInput'],
  ['SigninEmailRequest', 'EmailInput'],
  ['ConfirmSignupOtpRequest', 'EmailCodeInput'],
  ['ConfirmSigninOtpRequest', 'EmailCodeInput'],
]) {
  lines.push(`export const ${name}Constraints = ${source}Constraints`)
  lines.push(`export const ${name}Input = ${source}Input`)
  lines.push(
    `export const ${name} = ${source}.omit({delivery:true}).extend({delivery:TokenDelivery.default(${source}.shape.delivery.parse(undefined).toLowerCase() as 'cookie' | 'token')})`,
  )
  lines.push(`export type ${name} = z.input<typeof ${name}>`)
}
const output = lines.join('\n') + '\n'
await writeTypescript(new URL('./validation.ts', import.meta.url), output)
await writeTypescript(new URL('./schemas.ts', import.meta.url), banner + "export * from './validation.ts'\n")
const names = [
  ...ordered,
  'SignupEmailRequest',
  'SigninEmailRequest',
  'ConfirmSignupOtpRequest',
  'ConfirmSigninOtpRequest',
]
function typeOf(schema) {
  const value = unwrapped(schema)
  let result
  if (value.$ref) result = value.$ref.split('/').at(-1)
  else if (value.enum || value.oneOf)
    result = (value.enum ?? value.oneOf.map((variant) => variant.const)).map((item) => JSON.stringify(item)).join(' | ')
  else if (value.type === 'string') result = 'string'
  else if (['integer', 'number'].includes(value.type)) result = 'number'
  else if (value.type === 'boolean') result = 'boolean'
  else if (value.type === 'array') result = `Array<${typeOf(value.items)}>`
  else throw new Error('Unsupported form type')
  return nullable(schema) && !schema['x-compallm-non-null'] ? `${result} | null` : result
}
const declarations = [banner, "import type { z } from 'zod'", "import * as schemas from './validation.ts'"]
for (const name of ordered) {
  const schema = definitions[name]
  if (schema.description) declarations.push(`/** ${schema.description.replaceAll('*/', '* /')} */`)
  if (schema.properties) {
    declarations.push(`export interface ${name} {`)
    for (const [key, property] of Object.entries(schema.properties)) {
      if (property.description) declarations.push(`/** ${property.description.replaceAll('*/', '* /')} */`)
      declarations.push(`${JSON.stringify(key)}${schema.required?.includes(key) ? '' : '?'}: ${typeOf(property)}`)
    }
    declarations.push('}')
  } else {
    declarations.push(`export type ${name} = ${typeOf(schema)}`)
    if (schema.enum || schema.oneOf) declarations.push(`export const ${name} = schemas.${name}.enum`)
  }
}
for (const name of names.filter((name) => !ordered.includes(name)))
  declarations.push(`export type ${name} = z.input<typeof schemas.${name}>`)
await writeTypescript(new URL('./form-types.ts', import.meta.url), declarations.join('\n') + '\n')
await writeTypescript(
  new URL('./types.ts', import.meta.url),
  banner +
    "export * from './display-types.ts'\nexport * from './offline-provider-types.ts'\nexport { " +
    names.join(', ') +
    " } from './form-types.ts'\n",
)
await generate(
  {
    schema: 'schema.graphql',
    documents: 'operations.graphql',
    generates: {
      'graphql/schema.ts': { plugins: ['typescript'], config: { useTypeImports: true, scalars: { ID: 'string' } } },
      'graphql/': {
        preset: 'client',
        config: {
          useTypeImports: true,
          onlyOperationTypes: false,
          scalars: { ID: 'string' },
        },
      },
    },
  },
  true,
)
await writeTypescript(
  new URL('./graphql.ts', import.meta.url),
  banner +
    "export * from './graphql/index.ts'\nexport * from './graphql/graphql.ts'\nexport * as schema from './graphql/schema.ts'\n",
)
const build = spawnSync(process.platform === 'win32' ? 'npm.cmd' : 'npm', ['run', 'build'], {
  cwd: new URL('.', import.meta.url),
  stdio: 'inherit',
})
if (build.status !== 0) throw new Error('Generated contract TypeScript build failed')
