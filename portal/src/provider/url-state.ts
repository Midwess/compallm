import type {
  CreateProviderState,
  ModelDraft,
  ProviderModel,
  EndpointMode,
  ApiProtocol,
  ThinkingLevel,
} from '@compallm/storybook/pages/organization/create-provider'
import type { UrlState } from '@compallm/storybook'
import { providers } from './presets'
const modes: EndpointMode[] = ['chat_completions', 'responses', 'messages', 'video', 'realtime']
const protocols: ApiProtocol[] = ['openai_compatible', 'anthropic', 'provider_specific']
const thinking: ThinkingLevel[] = ['low', 'medium', 'high', 'xhigh', 'max']
const owned = [
  'v',
  'provider',
  'q',
  'model_q',
  'name',
  'base_url',
  'protocol',
  'mode',
  'models',
  'dialog',
  'model',
  'model_name',
  'context_tokens',
  'thinking',
  'input_price',
  'output_price',
  'cached_price',
  'intent',
]
const text = (value: unknown) => (typeof value === 'string' ? value : '')
const levels = (value: unknown): ThinkingLevel[] =>
  Array.isArray(value) ? thinking.filter((level) => value.includes(level)) : []
function draft(value: Record<string, unknown>): ModelDraft {
  return {
    id: text(value.id),
    name: text(value.name),
    maxContextTokens: text(value.maxContextTokens),
    thinkingLevels: levels(value.thinkingLevels),
    inputPrice: text(value.inputPrice),
    outputPrice: text(value.outputPrice),
    cachedPrice: text(value.cachedPrice),
  }
}
function decodeModels(raw: string | null): ProviderModel[] {
  if (!raw) return []
  const data: unknown = JSON.parse(raw)
  if (!Array.isArray(data) || data.length > 256) throw new Error('Invalid model draft list')
  const ids = new Set<string>()
  return data.map((item: unknown) => {
    if (typeof item !== 'object' || item === null || Array.isArray(item)) throw new Error('Invalid model draft')
    const value = item as Record<string, unknown>,
      model = draft(value)
    if (!model.id || ids.has(model.id)) throw new Error('Invalid model draft identity')
    ids.add(model.id)
    return {
      ...model,
      enabled: value.enabled === true,
      modes: modes.filter((mode) => Array.isArray(value.modes) && value.modes.includes(mode)),
    }
  })
}
export function emptyModel(id: string): ModelDraft {
  return { id, name: '', maxContextTokens: '', thinkingLevels: [], inputPrice: '', outputPrice: '', cachedPrice: '' }
}
export function modelFingerprint(model: ModelDraft) {
  return JSON.stringify(draft(model as unknown as Record<string, unknown>))
}
export function readProviderUrl(location: UrlState): Partial<CreateProviderState> {
  const q = new URLSearchParams(location.search)
  const preset = providers.find((item) => item.id === q.get('provider'))
  const protocol = protocols.find((value) => value === q.get('protocol')) ?? preset?.protocol ?? 'openai_compatible'
  const mode = modes.find((value) => value === q.get('mode')) ?? (protocol === 'anthropic' ? 'messages' : 'chat_completions')
  let models: ProviderModel[] = [],
    error: string | null = null
  try {
    if (q.has('v') && q.get('v') !== '1') throw new Error()
    models = decodeModels(q.get('models'))
  } catch {
    error = 'The model data in this URL is invalid. Check the address before continuing.'
  }
  let modelDraft: ModelDraft | null = null,
    editingModelId: string | null = null
  if (q.get('dialog') === 'add-model' || q.get('dialog') === 'edit-model') {
    const id = q.get('model') ?? 'new-model'
    const original = models.find((item) => item.id === id)
    if (q.get('dialog') === 'add-model' || original) {
      editingModelId = q.get('dialog') === 'edit-model' ? id : null
      const base = original ?? emptyModel(id)
      modelDraft = {
        ...draft(base as unknown as Record<string, unknown>),
        name: q.get('model_name') ?? base.name,
        maxContextTokens: q.get('context_tokens') ?? base.maxContextTokens,
        thinkingLevels: q.has('thinking') ? levels(q.get('thinking')?.split(',')) : base.thinkingLevels,
        inputPrice: q.get('input_price') ?? base.inputPrice,
        outputPrice: q.get('output_price') ?? base.outputPrice,
        cachedPrice: q.get('cached_price') ?? base.cachedPrice,
      }
    }
  }
  return {
    step: location.pathname.endsWith('/configure-provider') ? 'configure' : 'choose',
    intent: q.get('intent') === 'clone' ? 'clone' : 'create',
    selectedProviderId: preset?.id ?? null,
    providerQuery: q.get('q') ?? '',
    modelQuery: q.get('model_q') ?? '',
    name: q.get('name') ?? (preset ? `${preset.name} production` : ''),
    baseUrl: q.get('base_url') ?? preset?.baseUrl ?? '',
    protocol,
    mode,
    models,
    modelDraft,
    editingModelId,
    error,
  }
}
export function writeProviderUrl(state: CreateProviderState) {
  const patch: Record<string, string | null> = Object.fromEntries(owned.map((key) => [key, null]))
  Object.assign(patch, {
    v: '1',
    provider: state.selectedProviderId ?? 'custom',
    q: state.providerQuery || null,
    model_q: state.modelQuery || null,
    name: state.name,
    base_url: state.baseUrl,
    protocol: state.protocol,
    mode: state.mode,
    models: state.models.length
      ? JSON.stringify(
          state.models.map((model) => ({
            ...draft(model as unknown as Record<string, unknown>),
            enabled: model.enabled,
            modes: model.modes,
          })),
        )
      : null,
    intent: state.intent === 'clone' ? 'clone' : null,
  })
  if (state.modelDraft)
    Object.assign(patch, {
      dialog: state.editingModelId ? 'edit-model' : 'add-model',
      model: state.modelDraft.id,
      model_name: state.modelDraft.name,
      context_tokens: state.modelDraft.maxContextTokens,
      thinking: state.modelDraft.thinkingLevels.join(','),
      input_price: state.modelDraft.inputPrice,
      output_price: state.modelDraft.outputPrice,
      cached_price: state.modelDraft.cachedPrice,
    })
  return patch
}
