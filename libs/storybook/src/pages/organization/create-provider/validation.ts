import { ProviderFields, ProviderModelFields, ProviderModels, ProviderInputRules } from '@compallm/contract/validation'
import { headersAreValid, acceptsHeaderInput } from '@compallm/contract/provider'
import { modelPath, modelDraftFingerprint } from './model-options'
import type { CreateProviderState, ModelDraft, ProviderSnapshot } from './types'

export { headersAreValid, acceptsHeaderInput }
export const isValidBaseUrl = (value: string) => ProviderFields.shape.baseUrl.safeParse(value).success
export function hasValidModelPrices(model: ModelDraft): boolean {
  return [model.inputPrice, model.outputPrice, model.cachedPrice].every(
    (price) => ProviderModelFields.shape.inputPrice.safeParse(price).success,
  )
}
function modelInput(model: ModelDraft) {
  return {
    name: model.name,
    path: modelPath(model),
    maxContextTokens: Number(model.maxContextTokens),
    inputPrice: model.inputPrice,
    outputPrice: model.outputPrice,
    cachedPrice: model.cachedPrice,
  }
}
export function isValidModelDraft(model: ModelDraft): boolean {
  return (
    ProviderInputRules.shape.positiveInteger.safeParse(model.maxContextTokens).success &&
    ProviderModelFields.safeParse(modelInput(model)).success &&
    Object.values(model.effortMapping ?? {}).every(
      (value) => !value || ProviderInputRules.shape.effortValue.safeParse(value).success,
    )
  )
}
export function canCreateProvider(state: CreateProviderState): boolean {
  if (!headersAreValid(state.headers)) return false
  if (!ProviderFields.shape.name.safeParse(state.name).success) return false
  if (!ProviderFields.shape.baseUrl.safeParse(state.baseUrl).success) return false
  if (state.apiKey !== '' || !(state.intent === 'edit' && state.savedProvider)) {
    if (!ProviderFields.shape.apiKey.safeParse(state.apiKey).success) return false
  }
  const enabled = state.models.filter((model) => model.enabled)
  return enabled.every(isValidModelDraft) && ProviderModels.safeParse({ entries: enabled.map(modelInput) }).success
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
