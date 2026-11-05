import { readFile, writeFile } from 'node:fs/promises'

const document = JSON.parse(await readFile(new URL('../../portal/openapi.json', import.meta.url), 'utf8'))
const names = []
const visiting = new Set()
function visit(name) {
  if (names.includes(name)) return
  if (visiting.has(name)) throw new Error(`Recursive input contract requires explicit support: ${name}`)
  visiting.add(name)
  const component = document.components.schemas[name]
  function dependencies(schema) {
    if (!schema || typeof schema !== 'object') return
    if (schema.$ref) {
      const reference = schema.$ref.split('/').at(-1)
      if (document.components.schemas[reference]?.['x-compallm-input']) visit(reference)
    }
    Object.values(schema).forEach(dependencies)
  }
  dependencies(component)
  visiting.delete(name)
  names.push(name)
}
Object.entries(document.components.schemas)
  .filter(([, schema]) => schema['x-compallm-input'])
  .forEach(([name]) => visit(name))
const formats = new Set(['baseUrl', 'nonblank', 'authorization'])
const lines = [
  '// Generated from Rust/OpenAPI. Do not edit; run cargo run -p compallm-api -- generate.',
  "import * as raw from './schemas.ts'",
  "import { z } from 'zod'",
  "import { inputString, stringInput, integerInput, headerObject, headersObject, modelsObject } from './runtime.ts'",
]
for (const name of names) {
  const component = document.components.schemas[name]
  if (!component) throw new Error(`Missing contract component: ${name}`)
  const overrides = []
  const inputs = []
  const metadata = {}
  for (const [field, schema] of Object.entries(component.properties)) {
    if (schema.type === 'integer') {
      const bounds = {}
      for (const key of ['minimum', 'maximum']) if (schema[key] !== undefined) bounds[key] = schema[key]
      inputs.push(`${JSON.stringify(field)}: integerInput(${JSON.stringify(bounds)})`)
    }
    if (schema.type === 'array' && schema.items?.$ref) {
      const reference = schema.items.$ref.split('/').at(-1)
      if (names.includes(reference)) {
        if (schema.minItems !== undefined || schema.maxItems !== undefined || schema.uniqueItems) {
          throw new Error(`Array bounds need explicit contract support: ${name}.${field}`)
        }
        overrides.push(`${JSON.stringify(field)}: z.array(${reference})`)
      }
    }
    if (schema.type !== 'string') continue
    const constraints = {}
    for (const key of ['minLength', 'maxLength', 'pattern']) if (schema[key] !== undefined) constraints[key] = schema[key]
    if (schema['x-compallm-trim']) constraints.trim = true
    if (schema['x-compallm-input-pattern']) constraints.inputPattern = schema['x-compallm-input-pattern']
    if (schema['x-compallm-format']) {
      if (!formats.has(schema['x-compallm-format'])) throw new Error(`Unsupported custom rule: ${schema['x-compallm-format']}`)
      constraints.format = schema['x-compallm-format']
    }
    for (const key of Object.keys(schema)) {
      if (key.startsWith('x-compallm-') && !['x-compallm-trim', 'x-compallm-format', 'x-compallm-input-pattern'].includes(key))
        throw new Error(`Unsupported contract extension: ${key}`)
    }
    if (!Object.keys(constraints).length) continue
    const supported = new Set([
      'type',
      'title',
      'description',
      'example',
      'examples',
      'deprecated',
      'readOnly',
      'writeOnly',
      'minLength',
      'maxLength',
      'pattern',
      'x-compallm-trim',
      'x-compallm-format',
      'x-compallm-input-pattern',
    ])
    for (const key of Object.keys(schema)) {
      if (!supported.has(key)) throw new Error(`String keyword needs explicit contract support: ${name}.${field}.${key}`)
    }
    metadata[field] = constraints
    inputs.push(`${JSON.stringify(field)}: stringInput(${JSON.stringify(constraints)})`)
    overrides.push(`${JSON.stringify(field)}: inputString(${JSON.stringify(constraints)})`)
  }
  lines.push(`export const ${name}Constraints = ${JSON.stringify(metadata)} as const`)
  lines.push(`export const ${name}Input = {${inputs.join(',')}}`)
  const rule = component['x-compallm-object-rule']
  if (rule && !['header', 'headers', 'models'].includes(rule)) throw new Error(`Unsupported object rule: ${rule}`)
  for (const key of Object.keys(component)) {
    if (key.startsWith('x-compallm-') && !['x-compallm-input', 'x-compallm-object-rule'].includes(key))
      throw new Error(`Unsupported input extension: ${key}`)
  }
  const refinement =
    rule === 'header'
      ? '.refine(headerObject, { message: "Invalid Authorization header", path: ["value"] })'
      : rule === 'headers'
        ? '.refine(headersObject, { message: "Duplicate header names", path: ["entries"] })'
        : rule === 'models'
          ? '.refine(modelsObject, { message: "At least one model with unique names is required", path: ["entries"] })'
          : ''
  lines.push(
    `export const ${name} = raw.${name}.extend({${overrides.join(',')}})${component.additionalProperties === false ? '.strict()' : ''}${refinement}`,
  )
}
await writeFile(new URL('./validation.ts', import.meta.url), lines.join('\n') + '\n')
