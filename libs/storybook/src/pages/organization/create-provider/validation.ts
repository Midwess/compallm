import { isEndpointPath, modelPath, modelDraftFingerprint } from './model-options'
import type { HeaderEntry, CreateProviderState, ModelDraft, ProviderSnapshot } from './types'

export function hasValidModelPrices(model: ModelDraft): boolean {
  return [model.inputPrice, model.outputPrice, model.cachedPrice].every((price) => /^\d+(?:\.\d{1,8})?$/.test(price))
}
export function isValidModelDraft(model: ModelDraft): boolean {
  const context = model.maxContextTokens
  const output = model.maxOutputTokens ?? ''
  return (
    model.name.trim().length > 0 &&
    hasValidModelPrices(model) &&
    isEndpointPath(modelPath(model)) &&
    (output === '' || (/^[1-9]\d*$/.test(output) && Number.isSafeInteger(Number(output)))) &&
    (context === '' || (/^[1-9]\d*$/.test(context) && Number.isSafeInteger(Number(context))))
  )
}
export function canCreateProvider(state: CreateProviderState): boolean {
  if (!headersAreValid(state.headers)) return false
  if (!state.name.trim() || (!state.apiKey.trim() && !(state.intent === 'edit' && state.savedProvider))) return false
  try {
    const url = new URL(state.baseUrl)
    if (!['http:', 'https:'].includes(url.protocol) || url.username || url.password || url.search || url.hash) return false
  } catch {
    return false
  }
  const enabled = state.models.filter((model) => model.enabled)
  return (
    enabled.length > 0 &&
    enabled.every((model) => isValidModelDraft(model)) &&
    new Set(enabled.map((model) => model.name.trim())).size === enabled.length
  )
}

export function providerSnapshot(state: CreateProviderState) {
  return {
    name: state.name.trim(),
    baseUrl: state.baseUrl.trim(),
    protocol: state.protocol,
    mode: state.mode,
    headers: state.headers.map((header) => ({ ...header })),
    avatarUrl: state.avatarUrl,
    models: state.models.map((model) => ({ ...model, name: model.name.trim(), thinkingLevels: [...model.thinkingLevels] })),
  }
}
function comparableProvider(snapshot: ProviderSnapshot) {
  return JSON.stringify({
    ...snapshot,
    headers: snapshot.headers ?? [],
    avatarUrl: snapshot.avatarUrl ?? null,
    models: snapshot.models.map((model) => ({ id: model.id, enabled: model.enabled, draft: modelDraftFingerprint(model) })),
  })
}
export function hasProviderChanges(state: CreateProviderState): boolean {
  return (
    state.savedProvider !== null &&
    (state.apiKey !== '' || comparableProvider(providerSnapshot(state)) !== comparableProvider(state.savedProvider))
  )
}
export function canSaveProvider(state: CreateProviderState): boolean {
  return state.intent === 'edit' && hasProviderChanges(state) && canCreateProvider(state)
}

export function headersAreValid(headers: HeaderEntry[]): boolean {
  const keys = headers.map((entry) => entry.key.trim().toLowerCase())
  return (
    new Set(keys).size === keys.length &&
    headers.every((entry) => /^[!#$%&'*+.^_`|~0-9A-Za-z-]+$/.test(entry.key.trim()) && !/[\r\n]/.test(entry.value))
  )
}
