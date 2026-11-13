import type { ProviderModelConfiguration as ModelConfiguration } from '@compallm/contract/graphql'
import { ProviderModelConfiguration, ProviderCreateInput } from '@compallm/contract/validation'
import { ProviderFields, ProviderModelFields, ProviderInputRules } from '@compallm/contract/provider'
import { headersAreValid, acceptsHeaderInput } from '@compallm/contract/provider'
import { modelPath, modelDraftFingerprint } from './model-options'
import type { CreateProviderState, ModelDraft, ProviderSnapshot } from './types'
import { providerCreateRequest, providerReplaceRequest, modelConfiguration } from './contract'

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
  const models = state.models.filter((model) => model.enabled)
  if (!models.every(isValidModelDraft)) return false
  return state.intent === 'edit' && state.savedProvider
    ? providerReplaceRequest(state).success
    : providerCreateRequest(state).success
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

export function parseModelDraft(
  draft: ModelDraft | null,
  models: CreateProviderState['models'],
  editing: string | null,
): { data?: ModelConfiguration; issues?: { path: PropertyKey[]; message: string }[] } {
  const configuration = draft ? modelConfiguration({ ...draft, enabled: true, modes: [] }) : {}
  const result = ProviderModelConfiguration.safeParse(configuration)
  if (!result.success)
    return { issues: result.error.issues.map((issue) => ({ ...issue, path: ['modelDraft', ...issue.path] })) }
  const entries = models.filter((model) => model.id !== editing).map((model) => ({ configuration: modelConfiguration(model) }))
  entries.push({ configuration: result.data })
  const collection = ProviderCreateInput.shape.models.safeParse(entries)
  return collection.success
    ? { data: result.data }
    : { issues: collection.error.issues.map((issue) => ({ ...issue, path: ['modelDraft', 'name'] })) }
}
export function parseProvider(state: CreateProviderState): {
  values?: Partial<CreateProviderState>
  issues?: { path: PropertyKey[]; message: string }[]
} {
  const result = state.intent === 'edit' ? providerReplaceRequest(state) : providerCreateRequest(state)
  if (result.success) {
    const normalized = 'providers' in result.data ? result.data.providers[0] : result.data
    return {
      values: {
        name: normalized.name,
        baseUrl: normalized.baseUrl,
        apiKey: state.apiKey,
        models: state.models,
        headers: state.headers,
      },
    }
  }
  return {
    issues: result.error.issues.map((issue) => {
      const path = [...issue.path]
      if (path[0] === 'providers') path.splice(0, 2)
      const configuration = path.indexOf('configuration')
      if (configuration >= 0) path.splice(configuration, 1)
      return { ...issue, path }
    }),
  }
}
