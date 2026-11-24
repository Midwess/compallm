import type { StateStore } from '../state.svelte'
import type { InvitationContext } from './api-types'

export interface InvitationController {
  store: StateStore<{
    context: Pick<InvitationContext, 'organizationName' | 'email' | 'expiresAt' | 'signup'> | null
    loading: boolean
    step: 'continue' | 'code' | 'confirmed'
    error: string | null
    busy: boolean
  }>
  continue(): Promise<boolean>
  confirm(code: string): Promise<boolean>
  reload?(): void
}
export { default as InvitationPage } from './invitation-InvitationPage.svelte'
