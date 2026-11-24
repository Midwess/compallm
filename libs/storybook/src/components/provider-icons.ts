import OpenAIIcon from './OpenAIIcon.svelte'
import AnthropicIcon from './AnthropicIcon.svelte'
import GoogleAIStudioIcon from './GoogleAIStudioIcon.svelte'
import DeepSeekIcon from './DeepSeekIcon.svelte'
import GroqIcon from './GroqIcon.svelte'
import MistralIcon from './MistralIcon.svelte'
export { OpenAIIcon, AnthropicIcon, GoogleAIStudioIcon, DeepSeekIcon, GroqIcon, MistralIcon }
export type ProviderBrand = 'openai' | 'anthropic' | 'gemini' | 'deepseek' | 'groq' | 'mistral'
export const logos = {
  openai: OpenAIIcon,
  anthropic: AnthropicIcon,
  gemini: GoogleAIStudioIcon,
  deepseek: DeepSeekIcon,
  groq: GroqIcon,
  mistral: MistralIcon,
}
export function providerBrand(value: string | null | undefined): ProviderBrand | undefined {
  const name = value?.toLowerCase() ?? ''
  return (
    (Object.keys(logos) as ProviderBrand[]).find((brand) => name.includes(brand)) ??
    (name.includes('google') ? 'gemini' : undefined)
  )
}
export { default as ProviderIcon } from './ProviderIcon.svelte'
