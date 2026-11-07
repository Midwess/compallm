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
const formats = new Set(['baseUrl', 'nonblank', 'authorization', 'emailAddress', 'resourceId'])
const lines = [
  '// Generated from Rust/OpenAPI. Do not edit; run cargo run -p compallm-api -- generate.',
  "import * as raw from './schemas.ts'",
  "import { z } from 'zod'",
  "import { inputString, stringInput, integerInput, headerObject, headersObject, modelsObject, headerUpdateObject, modelWritesArray, createProviderObject, createProvidersObject, providerPatchObject } from './runtime.ts'",
]
for (const name of names) {
  const component = document.components.schemas[name]
  if (!component) throw new Error(`Missing contract component: ${name}`)
  const inputs = []
  const metadata = {}
  for (const [field, schema] of Object.entries(component.properties)) {
    if (schema.type === 'integer') {
      const bounds = {}
      for (const key of ['minimum', 'maximum']) if (schema[key] !== undefined) bounds[key] = schema[key]
      inputs.push(`${JSON.stringify(field)}: integerInput(${JSON.stringify(bounds)})`)
    }
    if (schema.type !== 'string') continue
    const constraints = {}
    for (const key of ['minLength', 'maxLength', 'pattern']) if (schema[key] !== undefined) constraints[key] = schema[key]
    if (schema['x-compallm-trim']) constraints.trim = true
    if (schema['x-compallm-ascii-lowercase']) constraints.asciiLowercase = true
    if (schema['x-compallm-email-policy']) constraints.emailPolicy = schema['x-compallm-email-policy']
    if (schema['x-compallm-input-pattern']) constraints.inputPattern = schema['x-compallm-input-pattern']
    if (schema['x-compallm-format']) {
      if (!formats.has(schema['x-compallm-format'])) throw new Error(`Unsupported custom rule: ${schema['x-compallm-format']}`)
      constraints.format = schema['x-compallm-format']
    }
    for (const key of Object.keys(schema)) {
      if (
        key.startsWith('x-compallm-') &&
        ![
          'x-compallm-trim',
          'x-compallm-format',
          'x-compallm-input-pattern',
          'x-compallm-ascii-lowercase',
          'x-compallm-email-policy',
        ].includes(key)
      )
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
      'x-compallm-ascii-lowercase',
      'x-compallm-email-policy',
    ])
    for (const key of Object.keys(schema)) {
      if (!supported.has(key)) throw new Error(`String keyword needs explicit contract support: ${name}.${field}.${key}`)
    }
    metadata[field] = constraints
    inputs.push(`${JSON.stringify(field)}: stringInput(${name}Constraints[${JSON.stringify(field)}])`)
  }
  lines.push(`export const ${name}Constraints = ${JSON.stringify(metadata)} as const`)
  lines.push(`export const ${name}Input = {${inputs.join(',')}}`)
  const rule = component['x-compallm-object-rule']
  if (
    rule &&
    !['header', 'headers', 'models', 'headerUpdate', 'createProvider', 'createProviders', 'providerPatch'].includes(rule)
  )
    throw new Error(`Unsupported object rule: ${rule}`)
  for (const key of Object.keys(component)) {
    if (key.startsWith('x-compallm-') && !['x-compallm-input', 'x-compallm-object-rule'].includes(key))
      throw new Error(`Unsupported input extension: ${key}`)
  }
  function expression(schema) {
    if (schema.$ref) {
      const reference = schema.$ref.split('/').at(-1)
      return names.includes(reference) ? reference : `raw.${reference}`
    }
    if (schema.allOf?.length === 1) return expression(schema.allOf[0])
    if (schema.type === 'array') {
      let item = `z.array(${expression(schema.items)})`
      if (schema.minItems !== undefined) item += `.min(${schema.minItems})`
      if (schema.maxItems !== undefined) item += `.max(${schema.maxItems})`
      if (schema.uniqueItems) throw new Error('uniqueItems requires an explicit array rule')
      const rule = schema['x-compallm-array-rule']
      if (rule === 'headers') item += '.refine((entries) => headersObject({entries}), {message: "Invalid input"})'
      else if (rule === 'modelWrites') item += '.refine(modelWritesArray, {message: "Invalid input"})'
      else if (rule) throw new Error(`Unsupported array rule: ${rule}`)
      return item
    }
    if (schema.type === 'object' && schema.properties) throw new Error('Inline input objects require named schemas')
    throw new Error(`Unsupported nested input schema: ${JSON.stringify(schema)}`)
  }
  const fields = Object.entries(component.properties).map(([field, schema]) => {
    let value
    if (metadata[field]) value = `inputString(${name}Constraints[${JSON.stringify(field)}])`
    else if (schema.$ref || schema.allOf || schema.type === 'array') value = expression(schema)
    else value = `raw.${name}.shape[${JSON.stringify(field)}]`
    if (!component.required?.includes(field)) value += '.optional()'
    if (schema.default !== undefined) value += `.default(${JSON.stringify(schema.default)})`
    return `${JSON.stringify(field)}: ${value}`
  })
  const refinements = {
    header: 'headerObject',
    headers: 'headersObject',
    models: 'modelsObject',
    headerUpdate: 'headerUpdateObject',
    createProvider: 'createProviderObject',
    createProviders: 'createProvidersObject',
    providerPatch: 'providerPatchObject',
  }
  const refinement = rule ? `.refine(${refinements[rule]}, {message: "Invalid input"})` : ''
  lines.push(
    `export const ${name} = raw.${name}.extend({${fields.join(',')}})${component.additionalProperties === false ? '.strict()' : ''}${refinement}`,
  )
}
await writeFile(new URL('./validation.ts', import.meta.url), lines.join('\n') + '\n')
