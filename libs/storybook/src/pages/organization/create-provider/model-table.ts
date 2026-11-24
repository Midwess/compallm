export function price(value: string) {
  return value === '' ? 'Review rates' : `$${value}`
}
export { default as ProviderModels } from './model-table-ProviderModels.svelte'
