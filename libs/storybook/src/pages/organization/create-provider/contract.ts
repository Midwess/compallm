import {
  ProvidersCreateInput,
  ProviderReplaceInput,
  TextModelProtocol,
  ProviderModelCapability,
} from '@compallm/contract/validation'
import type { ProviderModelConfiguration, ProviderModelWrite } from '@compallm/contract/graphql'
import { contextTokensFromText } from '@compallm/contract/provider'
import { modelPath, modelProtocol, effectiveContextTokens, effectiveModelPrice } from './model-options'
import type { CreateProviderState, ProviderModel } from './types'

export function modelConfiguration(model: ProviderModel): ProviderModelConfiguration {
  return {
    name: model.name,
    protocol: TextModelProtocol.parse(modelProtocol(model).toUpperCase()),
    path: modelPath(model),
    maxContextTokens: contextTokensFromText(effectiveContextTokens(model)),
    contextLimitEnabled: model.contextLimitEnabled ?? true,
    ...((model.inputLimitEnabled ?? false) ? { maxInputTokens: contextTokensFromText(model.maxInputTokens ?? '') } : {}),
    ...((model.outputLimitEnabled ?? !!model.maxOutputTokens)
      ? { maxOutputTokens: contextTokensFromText(model.maxOutputTokens ?? '') }
      : {}),
    headers: (model.headers ?? []).map(({ key, value }) => ({ key, value })),
    bodyDefaults: (model.bodyDefaults ?? []).map(({ key, value }) => ({ key, value })),
    enabled: model.enabled,
    effortMapping: Object.fromEntries(
      Object.entries(model.effortMapping ?? Object.fromEntries(model.thinkingLevels.map((level) => [level, level]))).filter(
        ([, value]) => !!value,
      ),
    ),
    capabilities: (model.capabilities ?? []).map((value) => ProviderModelCapability.parse(value.toUpperCase())),
    ...(model.catalogRef ? { catalogRef: model.catalogRef } : {}),
    inputPrice: effectiveModelPrice(model, 'inputPrice'),
    outputPrice: effectiveModelPrice(model, 'outputPrice'),
    cachedPrice: effectiveModelPrice(model, 'cachedPrice'),
    inputPriceEnabled: model.inputPriceEnabled ?? true,
    outputPriceEnabled: model.outputPriceEnabled ?? true,
    cachedPriceEnabled: model.cachedPriceEnabled ?? true,
  }
}

/** Build and validate a batch payload; never sends a request or persists a credential. */
export function providerCreateRequest(state: CreateProviderState): ReturnType<typeof ProvidersCreateInput.safeParse> {
  return ProvidersCreateInput.safeParse({
    providers: [
      {
        name: state.name,
        baseUrl: state.baseUrl,
        apiKey: state.apiKey,
        headers: state.headers.map(({ key, value }) => ({ key, value })),
        models: state.models.filter((model) => model.enabled).map((model) => ({ configuration: modelConfiguration(model) })),
      },
    ],
  })
}

/** Persisted IDs are supplied only by loaded detail data, never by local UI IDs. */
export function providerReplaceRequest(state: CreateProviderState): ReturnType<typeof ProviderReplaceInput.safeParse> {
  const models: ProviderModelWrite[] = state.models.map((model) => ({
    ...(model.persistedId ? { id: model.persistedId } : {}),
    configuration: modelConfiguration(model),
  }))
  return ProviderReplaceInput.safeParse({
    name: state.name,
    baseUrl: state.baseUrl,
    ...(state.apiKey ? { apiKey: state.apiKey } : {}),
    headers: state.headers.map(({ key, value }) => ({ key, value })),
    models,
  })
}
