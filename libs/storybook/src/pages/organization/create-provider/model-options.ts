import { ProviderModelFields } from '@compallm/contract/provider'
import { effortLevels } from '../../../components/effort-mapping'
import type { ModelCapability, ModelDraft, ProviderModel, TextModelProtocol } from './types'

export const capabilityOptions = [
  { value: 'text', label: 'Text' },
  { value: 'vision', label: 'Vision' },
  { value: 'pdf', label: 'PDF' },
  { value: 'audio', label: 'Audio input' },
  { value: 'video', label: 'Video input' },
  { value: 'image_generation', label: 'Image generation' },
  { value: 'audio_generation', label: 'Audio generation' },
  { value: 'search', label: 'Web search' },
  { value: 'reasoning', label: 'Reasoning' },
  { value: 'tool_call', label: 'Tool calling' },
  { value: 'forced_tool_choice', label: 'Forced tool choice' },
  { value: 'structured_output', label: 'Structured output' },
] as const satisfies ReadonlyArray<{ value: ModelCapability; label: string }>

export function modelTypeLabels(model: Pick<ProviderModel, 'protocol' | 'modes'>): string[] {
  if (model.protocol) return [textModelOptions.find((option) => option.value === model.protocol)!.label]
  return model.modes.map(
    (mode) =>
      ({
        chat_completions: 'OpenAI Completion',
        responses: 'OpenAI Response',
        messages: 'Anthropic',
        video: 'Video',
        realtime: 'Realtime',
      })[mode],
  )
}

export const textModelOptions = [
  { value: 'openai_chat_completions', label: 'OpenAI Completion', path: '/chat/completions', brand: 'openai' },
  { value: 'openai_responses', label: 'OpenAI Response', path: '/responses', brand: 'openai' },
  { value: 'anthropic_messages', label: 'Anthropic', path: '/messages', brand: 'anthropic' },
] as const

export function modelProtocol(model: Pick<ModelDraft, 'protocol'> & Partial<Pick<ProviderModel, 'modes'>>): TextModelProtocol {
  return model.protocol ?? (model.modes?.includes('messages') ? 'anthropic_messages' : 'openai_chat_completions')
}
export function modelPath(model: ModelDraft): string {
  return model.path ?? textModelOptions.find((option) => option.value === modelProtocol(model))!.path
}

export function initializeModelDraft(model: ModelDraft): ModelDraft {
  const draft: ModelDraft = {
    ...model,
    providerContextTokens: model.providerContextTokens ?? model.maxContextTokens,
    inputPriceEnabled: model.inputPriceEnabled ?? true,
    outputPriceEnabled: model.outputPriceEnabled ?? true,
    cachedPriceEnabled: model.cachedPriceEnabled ?? true,
    templatePrices: model.templatePrices ?? {
      inputPrice: model.inputPrice,
      outputPrice: model.outputPrice,
      cachedPrice: model.cachedPrice,
    },
    path: modelPath(model),
    effortMapping: model.effortMapping ?? Object.fromEntries(model.thinkingLevels.map((level) => [level, level])),
    contextLimitEnabled: model.contextLimitEnabled ?? true,
    inputLimitEnabled: model.inputLimitEnabled ?? false,
    outputLimitEnabled: model.outputLimitEnabled ?? !!model.maxOutputTokens,
    maxInputTokens: model.maxInputTokens ?? '',
    maxOutputTokens: model.maxOutputTokens ?? '',
    capabilities: [...(model.capabilities ?? [])],
    headers: (model.headers ?? []).map((entry) => ({ ...entry })),
    bodyDefaults: (model.bodyDefaults ?? []).map((entry) => ({ ...entry })),
  }
  if (!draft.defaults) {
    const { defaults: omitted, ...defaults } = draft
    void omitted
    draft.defaults = structuredClone(defaults)
  }
  return draft
}
export function effectiveContextTokens(model: ModelDraft): string {
  return model.contextLimitEnabled === false ? (model.providerContextTokens ?? model.maxContextTokens) : model.maxContextTokens
}
export const priceEnabledFields = {
  inputPrice: 'inputPriceEnabled',
  outputPrice: 'outputPriceEnabled',
  cachedPrice: 'cachedPriceEnabled',
} as const
export type ModelPriceField = keyof typeof priceEnabledFields
export function effectiveModelPrice(model: ModelDraft, key: ModelPriceField): string {
  return model[priceEnabledFields[key]] === false ? (model.templatePrices?.[key] ?? '') : model[key]
}
export function joinModelEndpoint(baseUrl: string, path: string): string {
  return `${baseUrl.replace(/\/+$/, '')}/${path.replace(/^\/+/, '')}`
}
export function isEndpointPath(path: string): boolean {
  return ProviderModelFields.shape.path.safeParse(path).success
}

export function modelDraftFingerprint(model: ModelDraft): string {
  return JSON.stringify({
    name: model.name,
    protocol: modelProtocol(model),
    path: modelPath(model),
    context: model.maxContextTokens,
    contextEnabled: model.contextLimitEnabled ?? true,
    input: model.maxInputTokens ?? '',
    inputEnabled: model.inputLimitEnabled ?? false,
    output: model.maxOutputTokens ?? '',
    outputEnabled: model.outputLimitEnabled ?? !!model.maxOutputTokens,
    headers: model.headers ?? [],
    body: model.bodyDefaults ?? [],
    efforts: effortLevels.map((level) =>
      model.effortMapping ? (model.effortMapping[level] ?? '') : model.thinkingLevels.includes(level) ? level : '',
    ),
    capabilities: capabilityOptions
      .filter((option) => model.capabilities?.includes(option.value))
      .map((option) => option.value),
    prices: [model.inputPrice, model.outputPrice, model.cachedPrice],
    priceEnabled: [model.inputPriceEnabled ?? true, model.outputPriceEnabled ?? true, model.cachedPriceEnabled ?? true],
  })
}
