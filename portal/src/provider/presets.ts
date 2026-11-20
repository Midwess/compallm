import type { ProviderPreset } from '@compallm/storybook/pages/organization/create-provider'
export const providers: ProviderPreset[] = [
  {
    id: 'openai',
    name: 'OpenAI',
    protocol: 'openai_compatible',
    baseUrl: 'https://api.openai.com/v1',
    logoPath: '/assets/providers/openai.svg',
  },
  {
    id: 'anthropic',
    name: 'Anthropic',
    protocol: 'anthropic',
    baseUrl: 'https://api.anthropic.com/v1',
    logoPath: '/assets/providers/anthropic.svg',
  },
  {
    id: 'gemini',
    name: 'Google AI Studio',
    protocol: 'provider_specific',
    baseUrl: 'https://generativelanguage.googleapis.com',
    logoPath: '/assets/providers/gemini.svg',
  },
  {
    id: 'deepseek',
    name: 'DeepSeek',
    protocol: 'openai_compatible',
    baseUrl: 'https://api.deepseek.com',
    logoPath: '/assets/providers/deepseek.svg',
  },
  {
    id: 'groq',
    name: 'Groq',
    protocol: 'openai_compatible',
    baseUrl: 'https://api.groq.com/openai/v1',
    logoPath: '/assets/providers/groq.svg',
  },
  {
    id: 'mistral',
    name: 'Mistral AI',
    protocol: 'openai_compatible',
    baseUrl: 'https://api.mistral.ai/v1',
    logoPath: '/assets/providers/mistral.svg',
  },
]
