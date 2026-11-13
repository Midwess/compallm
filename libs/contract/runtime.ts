import { z } from 'zod'
import { emailAddress, type EmailPolicy } from './email.ts'
import { compactCedarSource } from './cedar.ts'

// Rust str::trim and char::is_whitespace use Unicode White_Space. JS trim/\s
// additionally include FEFF and omit NEL; keep the Rust set explicit.
const whitespace = '[\\u0009-\\u000d\\u0020\\u0085\\u00a0\\u1680\\u2000-\\u200a\\u2028\\u2029\\u202f\\u205f\\u3000]'
const leadingWhitespace = new RegExp(`^${whitespace}+`, 'u')
const trailingWhitespace = new RegExp(`${whitespace}+$`, 'u')
const hasWhitespace = new RegExp(whitespace, 'u')
const invalidUnicode = /[\ud800-\udfff]/u
export const trimInput = (value: string) => value.replace(leadingWhitespace, '').replace(trailingWhitespace, '')

function baseUrl(value: string): boolean {
  if (hasWhitespace.test(value) || /[?#\\]/.test(value) || !/^https?:\/\//i.test(value)) return false
  try {
    const url = new URL(value)
    return ['http:', 'https:'].includes(url.protocol) && !!url.hostname && !url.username && !url.password
  } catch {
    return false
  }
}

// Explicit counterparts of custom Core validators. Ordinary patterns and limits
// come from generated metadata; these algorithms cannot be inferred from Rust.
const token = "[!#$%&'*+.^_`|~0-9A-Za-z-]+"
const schemePattern = new RegExp(`^${token}$`)
const token68 = /^[A-Za-z0-9\-._~+/]+=*$/
const quoted = String.raw`"(?:[\t !#-\[\]-~\x80-\xff]|\\[\t\x20-\x7e\x80-\xff])*"`
const param = `${token}[ \\t]*=[ \\t]*(?:${token}|${quoted})`
const params = new RegExp(`^${param}(?:[ \\t]*,[ \\t]*${param})*$(?![\\s\\S])`)
function authorization(value: string): boolean {
  const match = /^([^ ]+)(?: +(.+))?$/.exec(trimInput(value))
  if (!match || !schemePattern.test(match[1])) return false
  const [, scheme, credentials] = match
  if (/^(basic|bearer)$/i.test(scheme)) return !!credentials && token68.test(credentials)
  return !credentials || token68.test(credentials) || params.test(credentials)
}

export function headerObject(value: { key: string; value: string }): boolean {
  return value.key.toLowerCase() !== 'authorization' || authorization(value.value)
}

export function headersObject(value: { entries: Array<{ key: string }> }): boolean {
  return new Set(value.entries.map((header) => header.key.toLowerCase())).size === value.entries.length
}

export function modelsObject(value: { entries: Array<{ name: string }> }): boolean {
  return value.entries.length > 0 && new Set(value.entries.map((model) => model.name)).size === value.entries.length
}

const formats: Record<string, (value: string, constraints: StringConstraints) => boolean> = {
  baseUrl,
  resourceId: (value) => /^[1-9][0-9]*$/.test(value) && BigInt(value) <= 18446744073709551615n,
  nonblank: (value) => trimInput(value) !== '',
  authorization,
  emailAddress: (value, constraints) => {
    if (!constraints.emailPolicy) throw new Error('Email policy must be exported from Rust')
    return emailAddress(value, constraints.emailPolicy)
  },
}

export interface StringConstraints {
  messages?: Record<string, string>
  minLength?: number
  maxLength?: number
  pattern?: string
  inputPattern?: string
  format?: string
  trim?: boolean
  asciiLowercase?: boolean
  cedarCompact?: boolean
  emailPolicy?: EmailPolicy
}

function contractPattern(pattern: string | undefined) {
  const source = pattern?.replaceAll('\\s', whitespace.slice(1, -1))
  return source ? new RegExp(source.replace(/\$$/, '$(?![\\s\\S])'), 'u') : undefined
}

export function authorizationInput(value: string): boolean {
  if (value === '') return true
  const separator = value.indexOf(' ')
  const scheme = separator < 0 ? value : value.slice(0, separator)
  if (!schemePattern.test(scheme)) return false
  if (separator < 0 || !/^(basic|bearer)$/i.test(scheme)) return true
  const credentials = value.slice(separator).replace(/^ +/, '')
  return credentials === '' || token68.test(credentials)
}

const inputFormats: Record<string, (value: string) => boolean> = {
  resourceId: (value) => /^[1-9][0-9]*$/.test(value) && BigInt(value) <= 18446744073709551615n,
  nonblank: () => true,
  baseUrl: (value) => !hasWhitespace.test(value) && !/[?#\\]/.test(value),
  authorization: authorizationInput,
  emailAddress: () => true,
}

// Entry guards use exported limits and character rules, allowing deletion and
// incomplete values that are necessary before a final value can be submitted.
export function stringInput(constraints: StringConstraints) {
  const pattern = contractPattern(constraints.inputPattern ?? constraints.pattern)
  const format = constraints.format ? inputFormats[constraints.format] : undefined
  if (constraints.format && !format) throw new Error(`Unsupported input format: ${constraints.format}`)
  return (value: string): boolean =>
    value === '' ||
    (!invalidUnicode.test(value) &&
      (constraints.maxLength === undefined ||
        [...(constraints.cedarCompact ? compactCedarSource(value) : value)].length <= constraints.maxLength) &&
      (!pattern || pattern.test(value)) &&
      (!format || format(value)))
}

export function integerInput(constraints: { minimum?: number; maximum?: number }) {
  const signed = constraints.minimum !== undefined && constraints.minimum < 0
  const pattern = signed ? /^-?[0-9]*$/ : /^[0-9]*$/
  return (value: string): boolean => {
    if (value === '' || (signed && value === '-')) return true
    if (!pattern.test(value)) return false
    const number = Number(value)
    return (
      Number.isSafeInteger(number) &&
      (constraints.minimum === undefined || number >= Math.min(0, constraints.minimum)) &&
      (constraints.maximum === undefined || number <= constraints.maximum)
    )
  }
}

export function inputString(constraints: StringConstraints) {
  const format = constraints.format ? formats[constraints.format] : undefined
  if (constraints.format && !format) throw new Error(`Unsupported input format: ${constraints.format}`)
  // Rust regex $ requires the absolute end; JS $ also matches before a trailing
  // newline. The extra assertion preserves the Rust behavior. Unicode mode is
  // required for scalar-value quantifiers (including API-key length).
  const pattern = contractPattern(constraints.pattern)
  return z
    .string()
    .transform((value) => {
      const normalized = constraints.cedarCompact ? compactCedarSource(value) : value
      const trimmed = constraints.trim ? trimInput(normalized) : normalized
      return constraints.asciiLowercase ? trimmed.replace(/[A-Z]/g, (letter) => letter.toLowerCase()) : trimmed
    })
    .superRefine((value, context) => {
      const length = [...value].length
      const failure = invalidUnicode.test(value)
        ? 'unicode'
        : constraints.minLength !== undefined && length < constraints.minLength
          ? 'minLength'
          : constraints.maxLength !== undefined && length > constraints.maxLength
            ? 'maxLength'
            : pattern && !pattern.test(value)
              ? 'pattern'
              : format && !format(value, constraints)
                ? 'x-compallm-format'
                : undefined
      if (failure)
        context.addIssue({
          code: 'custom',
          message: constraints.messages?.[failure] ?? 'Invalid input',
        })
    })
}

// Explicit counterparts of Rust collection rules. References use generated field
// validators; these checks only cover relationships between submitted values.
export function headerUpdateObject(value: { key: string; value?: string }): boolean {
  return value.value === undefined || headerObject({ key: value.key, value: value.value })
}
export function modelWritesArray(
  entries: Array<{
    id?: string
    configuration: { name: string; enabled?: boolean }
  }>,
): boolean {
  const ids = entries.flatMap((entry) => (entry.id === undefined ? [] : [entry.id]))
  return (
    entries.some((entry) => entry.configuration.enabled !== false) &&
    new Set(entries.map((entry) => entry.configuration.name)).size === entries.length &&
    new Set(ids).size === ids.length
  )
}
export function createProviderObject(value: { models: Array<{ id?: string }> }): boolean {
  return value.models.every((model) => model.id === undefined)
}
export function createProvidersObject(value: { providers: Array<{ name: string }> }): boolean {
  return (
    value.providers.length > 0 &&
    new Set(value.providers.map((provider) => trimInput(provider.name))).size === value.providers.length
  )
}
export function providerPatchObject(value: Record<string, unknown>): boolean {
  return Object.values(value).some((field) => field !== undefined)
}
