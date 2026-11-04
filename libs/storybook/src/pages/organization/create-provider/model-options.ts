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
export function joinModelEndpoint(baseUrl: string, path: string): string {
  return `${baseUrl.replace(/\/+$/, '')}/${path.replace(/^\/+/, '')}`
}
export function isEndpointPath(path: string): boolean {
  if (!path.startsWith('/') || path.startsWith('//') || /[?#\\\s]/.test(path)) return false
  try {
    return !decodeURIComponent(path)
      .split('/')
      .some((part) => part === '.' || part === '..' || /[?#\\\s]/.test(part))
  } catch {
    return false
  }
}

export function modelDraftFingerprint(model: ModelDraft): string {
  return JSON.stringify({
    name: model.name,
    protocol: modelProtocol(model),
    path: modelPath(model),
    context: model.maxContextTokens,
    output: model.maxOutputTokens ?? '',
    efforts: effortLevels.map((level) =>
      model.effortMapping ? (model.effortMapping[level] ?? '') : model.thinkingLevels.includes(level) ? level : '',
    ),
    capabilities: capabilityOptions
      .filter((option) => model.capabilities?.includes(option.value))
      .map((option) => option.value),
    prices: [model.inputPrice, model.outputPrice, model.cachedPrice],
  })
}
