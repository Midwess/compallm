export type { PasskeyController, PasskeyState } from './types'
export const useController = getPasskeyController
export function credentialDescription(registeredAt: string, lastUsedAt: string | null) {
  const added = new Date(Number(registeredAt) * 1000).toLocaleDateString()
  const used = lastUsedAt ? `Last used ${new Date(Number(lastUsedAt) * 1000).toLocaleDateString()}` : 'Not used yet'
  return `Added ${added} · ${used}`
}
export { default as SecurityEnhancementPage } from './index-SecurityEnhancementPage.svelte'
export { default as PasskeySettings } from './index-PasskeySettings.svelte'
export { default as PasskeyRow } from './index-PasskeyRow.svelte'
export { default as PasskeyDialogs } from './index-PasskeyDialogs.svelte'
export { default as PasskeyDialogContent } from './index-PasskeyDialogContent.svelte'

import { getPasskeyController } from '../../context'
