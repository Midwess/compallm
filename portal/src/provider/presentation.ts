import { cloneFormValue } from '@compallm/storybook'
import type {
  ModelDraft,
  ProviderModel,
  ProviderPreset,
  TextModelProtocol,
} from '@compallm/storybook/pages/organization/create-provider'
import type { ProviderDetail, ProviderPage, ModelDetail } from './catalog'

export function providerRow(value: ProviderPage['edges'][number]['node']): ProviderPreset {
  return { id: value.id, name: value.name, baseUrl: value.baseUrl, logoPath: value.logoUrl ?? undefined }
}
function priceLabel(amount: string | null, mode: string) {
  return amount !== null ? `$${amount}` : mode === 'TIERED' ? 'Tiered' : mode === 'SCHEDULED' ? 'Scheduled' : 'Not set'
}
export function modelRow(value: ProviderDetail['models']['edges'][number]['node'], provider: string): ProviderModel {
  const operation = value.operations[0]
  const protocol = operation?.operation.toLowerCase() as TextModelProtocol | undefined
  const modes = value.operations.map(
    ({ operation }) =>
      (
        ({
          OPENAI_CHAT_COMPLETIONS: 'chat_completions',
          OPENAI_RESPONSES: 'responses',
          ANTHROPIC_MESSAGES: 'messages',
        }) as const
      )[operation],
  )
  return {
    id: value.id,
    catalogRef: { provider, model: value.id },
    name: value.name,
    description: value.displayName !== value.name ? value.displayName : (value.description ?? undefined),
    enabled: false,
    modes,
    protocol,
    path: operation?.path,
    maxContextTokens: value.maxContextTokens?.toString() ?? '',
    providerContextTokens: value.maxContextTokens?.toString() ?? '',
    maxInputTokens: value.maxInputTokens?.toString() ?? '',
    maxOutputTokens: value.maxOutputTokens?.toString() ?? '',
    contextLimitEnabled: value.maxContextTokens != null,
    inputLimitEnabled: value.maxInputTokens != null,
    outputLimitEnabled: value.maxOutputTokens != null,
    inputPriceEnabled: value.inputPrice != null,
    outputPriceEnabled: value.outputPrice != null,
    cachedPriceEnabled: value.cachedPrice != null,
    thinkingLevels: [],
    capabilities: value.capabilities.map((value) => value.toLowerCase()) as ProviderModel['capabilities'],
    inputPrice: value.inputPrice ?? '',
    outputPrice: value.outputPrice ?? '',
    cachedPrice: value.cachedPrice ?? '',
    catalogPriceLabels: [
      priceLabel(value.inputPrice, value.pricingMode),
      priceLabel(value.outputPrice, value.pricingMode),
      value.cachedPrice === null && value.cacheBilling === 'UNSUPPORTED'
        ? 'Unsupported'
        : priceLabel(value.cachedPrice, value.pricingMode),
    ],
  }
}
export function modelDraft(value: ModelDetail, global?: ModelDetail | null): ModelDraft {
  const protocol = value.connections
    .map(
      ({ protocol }) =>
        (
          ({
            OPENAI: 'openai_chat_completions',
            OPENAI_RESPONSES: 'openai_responses',
            ANTHROPIC: 'anthropic_messages',
          }) as Partial<Record<string, TextModelProtocol>>
        )[protocol],
    )
    .find(Boolean)
  const path =
    value.connections.find(
      (c) =>
        (
          ({
            OPENAI: 'openai_chat_completions',
            OPENAI_RESPONSES: 'openai_responses',
            ANTHROPIC: 'anthropic_messages',
          }) as Record<string, string>
        )[c.protocol] === protocol,
    )?.configuration.path ??
    value.connections[0]?.configuration.path ??
    undefined
  const input = value.text.input.configuration?.maxTokens
  const output = value.text.output.configuration?.maxTokens
  const thinking = value.text.output.configuration?.thinking.configuration
  const efforts = Object.fromEntries((thinking?.effortMapping ?? []).map(({ level, value }) => [level.toLowerCase(), value]))
  const entries = (items: { key: string; value: string }[]) => items.map((entry, index) => ({ id: String(index), ...entry }))
  const draft: ModelDraft = {
    id: value.id,
    origin: value.origin,
    catalogRef: { provider: value.providerTemplateId, model: value.id },
    name: value.name,
    protocol,
    path: path ?? '',
    maxContextTokens: value.maxContextTokens?.toString() ?? '',
    providerContextTokens: value.maxContextTokens?.toString() ?? '',
    maxInputTokens: input?.toString() ?? '',
    maxOutputTokens: output?.toString() ?? '',
    contextLimitEnabled: value.maxContextTokens !== null,
    inputLimitEnabled: input != null,
    outputLimitEnabled: output != null,
    capabilities: value.capabilities.map((c) => c.toLowerCase()) as ModelDraft['capabilities'],
    thinkingLevels: Object.keys(efforts) as ModelDraft['thinkingLevels'],
    effortMapping: efforts,
    catalogPriceLabels: [
      priceLabel(value.inputPrice, value.pricingMode),
      priceLabel(value.outputPrice, value.pricingMode),
      value.cachedPrice === null && value.cacheBilling === 'UNSUPPORTED'
        ? 'Unsupported'
        : priceLabel(value.cachedPrice, value.pricingMode),
    ],
    inputPrice: value.inputPrice ?? '',
    outputPrice: value.outputPrice ?? '',
    cachedPrice: value.cachedPrice ?? '',
    inputPriceEnabled: value.inputPrice !== null,
    outputPriceEnabled: value.outputPrice !== null,
    cachedPriceEnabled: value.cachedPrice !== null,
    catalogPrices: {
      inputPrice: value.inputPrice ?? '',
      outputPrice: value.outputPrice ?? '',
      cachedPrice: value.cachedPrice ?? '',
    },
    headers: entries(value.headers),
    bodyDefaults: entries(value.bodyDefaults),
  }
  const defaults = global ? modelDraft(global) : undefined
  if (defaults) {
    draft.providerContextTokens = defaults.maxContextTokens
    draft.catalogPrices = {
      inputPrice: defaults.inputPrice,
      outputPrice: defaults.outputPrice,
      cachedPrice: defaults.cachedPrice,
    }
  }
  for (const field of [
    'contextLimitEnabled',
    'inputLimitEnabled',
    'outputLimitEnabled',
    'inputPriceEnabled',
    'outputPriceEnabled',
    'cachedPriceEnabled',
  ] as const)
    draft[field] = value.customFields.includes(field)
  return { ...draft, defaults: cloneFormValue(defaults ?? draft) }
}
