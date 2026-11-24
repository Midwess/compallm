import type { ApiProtocol } from './types'
export const protocolLabels: Record<ApiProtocol, string> = {
  openai_compatible: 'OpenAI compatible',
  anthropic: 'Anthropic',
  provider_specific: 'Provider-specific',
}
export { default as ProviderChooser } from './chooser-ProviderChooser.svelte'
