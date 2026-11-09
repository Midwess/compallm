import { buildSchema, isEnumType, isInputObjectType, isListType, isNonNullType, isScalarType } from 'graphql'

// Compare structure with the authoritative API schema. Domain refinements may
// reject additional values; they must never introduce an incompatible wire shape.
export function checkValidationCompatibility(sdl, document, scalarShapes = {}) {
  const schema = buildSchema(sdl)
  const definitions = document.$defs
  if (!definitions) throw new Error('Missing Rust validation definitions')
  const checked = []
  const formOnly = []
  const visited = new Set()
  const scalarTypes = {
    ID: 'string',
    String: 'string',
    Boolean: 'boolean',
    Int: 'integer',
    Float: 'number',
    ...scalarShapes,
  }

  function fail(path, reason) {
    throw new Error(`Validation metadata differs from GraphQL at ${path}: ${reason}`)
  }

  function resolveMetadata(original, path, references = new Set()) {
    let value = { ...original }
    let nullable = false
    if (value.anyOf) {
      const nonNull = value.anyOf.filter((branch) => branch.type !== 'null')
      if (nonNull.length !== 1 || nonNull.length === value.anyOf.length) fail(path, 'unsupported validation union')
      nullable = true
      const { anyOf: _, ...rest } = value
      value = { ...nonNull[0], ...rest }
    }
    if (Array.isArray(value.type)) {
      const nonNull = value.type.filter((type) => type !== 'null')
      if (nonNull.length !== 1) fail(path, 'unsupported validation type union')
      nullable ||= value.type.includes('null')
      value.type = nonNull[0]
    }
    if (value.$ref) {
      if (!value.$ref.startsWith('#/$defs/')) fail(path, 'unsupported validation reference')
      const name = value.$ref.slice('#/$defs/'.length)
      if (!definitions[name] || references.has(name)) fail(path, 'unresolved validation reference')
      const target = resolveMetadata(definitions[name], path, new Set([...references, name]))
      const { $ref: _, ...rest } = value
      value = { ...target.value, ...rest }
      nullable ||= target.nullable
    }
    return { value, nullable: nullable && !value['x-compallm-non-null'] }
  }

  function canonical(value) {
    if (Array.isArray(value)) return value.map(canonical)
    if (value && typeof value === 'object') {
      return Object.fromEntries(
        Object.keys(value)
          .sort()
          .map((key) => [key, canonical(value[key])]),
      )
    }
    return value
  }

  function compare(original, type, path) {
    const metadata = resolveMetadata(original, path)
    if (metadata.nullable && isNonNullType(type)) fail(path, 'validator allows null for a non-null API value')
    const target = isNonNullType(type) ? type.ofType : type
    const value = metadata.value
    if (isListType(target)) {
      if (value.type !== 'array' || !value.items) fail(path, 'expected a list')
      compare(value.items, target.ofType, `${path}[]`)
      return
    }
    if (isEnumType(target)) {
      const members = value.enum ?? value.oneOf?.map((variant) => variant.const)
      const expected = target
        .getValues()
        .map((member) => member.name)
        .sort()
      if (!members || JSON.stringify([...members].sort()) !== JSON.stringify(expected))
        fail(path, 'enum members differ')
      return
    }
    if (isScalarType(target)) {
      const expected = scalarTypes[target.name]
      if (!expected) fail(path, `custom scalar ${target.name} needs an explicit structural mapping`)
      if (value.type !== expected && !(expected === 'number' && value.type === 'integer')) {
        fail(path, `expected ${expected} for ${target.name}`)
      }
      return
    }
    if (!isInputObjectType(target) || value.type !== 'object') fail(path, 'expected a GraphQL input object')
    const key = `${target.name}:${JSON.stringify(value)}`
    if (visited.has(key)) return
    visited.add(key)
    const fields = target.getFields()
    const properties = value.properties ?? {}
    if (JSON.stringify(Object.keys(fields).sort()) !== JSON.stringify(Object.keys(properties).sort())) {
      fail(path, 'input fields differ')
    }
    for (const [name, field] of Object.entries(fields)) {
      const property = properties[name]
      const required = value.required?.includes(name) && property.default === undefined
      const apiRequired = isNonNullType(field.type) && field.defaultValue === undefined
      if (Boolean(required) !== apiRequired) fail(`${path}.${name}`, 'omission/default requirement differs')
      if (JSON.stringify(canonical(property.default)) !== JSON.stringify(canonical(field.defaultValue))) {
        fail(`${path}.${name}`, 'default value differs')
      }
      compare(property, field.type, `${path}.${name}`)
    }
  }

  // Every exposed input object needs metadata registration, even when it adds no
  // constraints beyond the GraphQL shape. Missing registration must not go stale.
  for (const type of Object.values(schema.getTypeMap())) {
    if (isInputObjectType(type) && !definitions[type.name]) fail(type.name, 'missing Rust validation metadata')
  }
  for (const [name, definition] of Object.entries(definitions)) {
    const type = schema.getType(name)
    if (!type) {
      formOnly.push(name)
      continue
    }
    if (!isInputObjectType(type) && !isEnumType(type)) fail(name, 'metadata name refers to a non-input API type')
    compare(definition, type, name)
    checked.push(name)
  }
  return { checked, formOnly }
}
