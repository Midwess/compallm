export { default as EffortMapping } from './EffortMapping.svelte'
export type EffortLevel = 'low' | 'medium' | 'high' | 'xhigh' | 'max'
export type EffortValues = Partial<Record<EffortLevel, string>>
export const effortLevels = ['low', 'medium', 'high', 'xhigh', 'max'] as const
