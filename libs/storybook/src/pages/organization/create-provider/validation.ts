import type { CreateProviderState, ModelDraft } from './types'

export function hasValidModelPrices(model: ModelDraft): boolean {
  return [model.inputPrice, model.outputPrice, model.cachedPrice].every((price) => /^\d+(?:\.\d{1,8})?$/.test(price))
}
export function isValidModelDraft(model: ModelDraft): boolean {
  const context = model.maxContextTokens
  return (
    model.name.trim().length > 0 &&
    hasValidModelPrices(model) &&
    (context === '' || (/^[1-9]\d*$/.test(context) && Number.isSafeInteger(Number(context))))
  )
}
export function canCreateProvider(state: CreateProviderState): boolean {
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
    enabled.every((model) => isValidModelDraft(model) && model.modes.includes(state.mode)) &&
    new Set(enabled.map((model) => model.name.trim())).size === enabled.length
  )
}

export function providerSnapshot(state: CreateProviderState) {
  return {
    name: state.name.trim(),
    baseUrl: state.baseUrl.trim(),
    protocol: state.protocol,
    mode: state.mode,
    models: state.models.map((model) => ({ ...model, name: model.name.trim(), thinkingLevels: [...model.thinkingLevels] })),
  }
}
function comparableProvider(snapshot: ReturnType<typeof providerSnapshot>) {
  return JSON.stringify({
    ...snapshot,
    models: snapshot.models.map((model) => ({
      id: model.id,
      name: model.name.trim(),
      maxContextTokens: model.maxContextTokens,
      thinkingLevels: [...model.thinkingLevels].sort(),
      enabled: model.enabled,
      inputPrice: model.inputPrice,
      outputPrice: model.outputPrice,
      cachedPrice: model.cachedPrice,
    })),
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
