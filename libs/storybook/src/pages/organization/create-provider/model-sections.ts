import type { ModelCapability } from './types'
export const modelTones = {
  context: 'text-foreground/80',
  input: 'text-foreground/75',
  output: 'text-foreground/65',
}
export const capabilityDescriptions: Record<ModelCapability, string> = {
  text: 'Understand and generate text.',
  vision: 'Understand images in a request.',
  pdf: 'Read PDF documents.',
  audio: 'Understand audio in a request.',
  video: 'Understand video in a request.',
  image_generation: 'Generate images.',
  audio_generation: 'Generate audio.',
  search: 'Search the web for current information.',
  reasoning: 'Use additional reasoning before responding.',
  tool_call: 'Call functions and external tools.',
  forced_tool_choice: 'Require a specific tool to be called.',
  structured_output: 'Return responses matching a schema.',
}
export { default as ModelSection } from './model-sections-ModelSection.svelte'
export { default as ModelContext } from './model-sections-ModelContext.svelte'
export { default as ContextExample } from './model-sections-ContextExample.svelte'
export { default as LimitRow } from './model-sections-LimitRow.svelte'
export { default as ModelPricing } from './model-sections-ModelPricing.svelte'
export { default as PriceRow } from './model-sections-PriceRow.svelte'
export { default as PriceComparison } from './model-sections-PriceComparison.svelte'
export { default as ModelCapabilities } from './model-sections-ModelCapabilities.svelte'
