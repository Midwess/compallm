import { ProviderHeaderInput, ProviderHeaders, ProviderInputRulesInput } from './validation.ts'

export interface HeaderInput {
  key: string
  value: string
}

export function headersAreValid(headers: HeaderInput[]): boolean {
  return ProviderHeaders.safeParse({ entries: headers.map(({ key, value }) => ({ key, value })) }).success
}

// Drafts may contain an empty key or an incomplete Authorization value.
// Final collection validation remains required before submission.
export function acceptsHeaderInput(entry: HeaderInput): boolean {
  return (
    ProviderHeaderInput.key(entry.key) &&
    ProviderHeaderInput.value(entry.value) &&
    (entry.key.toLowerCase() !== 'authorization' || ProviderInputRulesInput.authorization(entry.value))
  )
}
