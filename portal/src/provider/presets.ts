import type { ProviderRegistryListResponse } from '@compallm/contract/api'
export const providers: ProviderRegistryListResponse['providers'] = [
  { id: 'openai', name: 'OpenAI', protocol: 'openai_compatible', baseUrl: 'https://api.openai.com/v1' },
  { id: 'anthropic', name: 'Anthropic', protocol: 'anthropic', baseUrl: 'https://api.anthropic.com/v1' },
  {
    id: 'gemini',
    name: 'Google AI Studio',
    protocol: 'provider_specific',
    baseUrl: 'https://generativelanguage.googleapis.com',
  },
  { id: 'deepseek', name: 'DeepSeek', protocol: 'openai_compatible', baseUrl: 'https://api.deepseek.com' },
  { id: 'groq', name: 'Groq', protocol: 'openai_compatible', baseUrl: 'https://api.groq.com/openai/v1' },
  { id: 'mistral', name: 'Mistral AI', protocol: 'openai_compatible', baseUrl: 'https://api.mistral.ai/v1' },
]
