import { ImageUploadInput } from '@compallm/contract/validation'

import { ProviderModelFieldsInput } from '@compallm/contract/provider'

export const modelDraftRules = {
  'modelDraft.path': { accepts: (value: unknown) => ProviderModelFieldsInput.path('/' + String(value).replace(/^\/+/, '')) },
  'modelDraft.name': { accepts: (value: unknown) => ProviderModelFieldsInput.name(String(value)) },
  'modelDraft.maxContextTokens': { accepts: (value: unknown) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
  'modelDraft.maxInputTokens': { accepts: (value: unknown) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
  'modelDraft.maxOutputTokens': { accepts: (value: unknown) => ProviderModelFieldsInput.maxContextTokens(String(value)) },
  'modelDraft.inputPrice': { accepts: (value: unknown) => ProviderModelFieldsInput.inputPrice(String(value)) },
  'modelDraft.outputPrice': { accepts: (value: unknown) => ProviderModelFieldsInput.inputPrice(String(value)) },
  'modelDraft.cachedPrice': { accepts: (value: unknown) => ProviderModelFieldsInput.inputPrice(String(value)) },
}
export function avatarInput(file: File) {
  return ImageUploadInput.safeParse({
    size: file.size,
    mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
  })
}
export { useProviderForm } from './configure-form.svelte'
export { default as ConnectionFields } from './configuration-ConnectionFields.svelte'
export { default as TestConnection } from './configuration-TestConnection.svelte'
export { default as ProviderConfiguration } from './configuration-ProviderConfiguration.svelte'
