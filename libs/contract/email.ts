export interface EmailPolicy {
  localMaxBytes: number
  labelMaxBytes: number
  domainMaxBytes: number
  atom: string
  quoted: string
  literal: string
  edge: string
}

const bytes = new TextEncoder()
const policies = new WeakMap<EmailPolicy, Record<'atom' | 'quoted' | 'literal' | 'edge', RegExp>>()

// Explicit adapter for the Rust email_address default parser. Its character
// classes and UTF-8 byte limits are exported from the Rust dependency itself.
export function emailAddress(value: string, policy: EmailPolicy): boolean {
  let patterns = policies.get(policy)
  if (!patterns) {
    const exact = (source: string) => new RegExp(source.replace(/\$$/, '$(?![\\s\\S])'), 'u')
    patterns = {
      atom: exact(policy.atom),
      quoted: exact(policy.quoted),
      literal: exact(policy.literal),
      edge: exact(policy.edge),
    }
    policies.set(policy, patterns)
  }
  let address = value
  const display = address.lastIndexOf(' <')
  if (display >= 0) {
    const right = address.slice(display + 2).replace(/^[\p{White_Space}]+|[\p{White_Space}]+$/gu, '')
    if (!right.endsWith('>')) return false
    address = right.slice(0, -1)
  }
  const at = address.lastIndexOf('@')
  if (at < 0) return false
  const local = address.slice(0, at)
  const domain = address.slice(at + 1)
  if (
    !local ||
    !domain ||
    bytes.encode(local).length > policy.localMaxBytes ||
    bytes.encode(domain).length > policy.domainMaxBytes
  )
    return false
  if (local.startsWith('"') && local.endsWith('"')) {
    if (local.length <= 2) return false
    const characters = [...local.slice(1, -1)]
    for (let index = 0; index < characters.length; index++) {
      if (characters[index] === '\\') {
        const escaped = characters[++index]?.codePointAt(0)
        if (escaped === undefined || escaped < 0x21 || escaped > 0x7e) return false
      } else if (!patterns.quoted.test(characters[index])) return false
    }
  } else if (!local.split('.').every((part) => patterns.atom.test(part))) return false
  if (domain.startsWith('[') && domain.endsWith(']')) {
    const literal = domain.slice(1, -1)
    return literal === '' || patterns.literal.test(literal)
  }
  return domain.split('.').every((label) => {
    const characters = [...label]
    return (
      bytes.encode(label).length <= policy.labelMaxBytes &&
      patterns.atom.test(label) &&
      patterns.edge.test(characters[0] ?? '') &&
      patterns.edge.test(characters.at(-1) ?? '')
    )
  })
}
