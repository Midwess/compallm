import type { StoreApi } from 'zustand/vanilla'
import type { PasskeySummary } from '@compallm/contract/graphql'
export interface PasskeyState {
  records: Pick<PasskeySummary, 'id' | 'name' | 'registeredAt' | 'lastUsedAt'>[]
  status: 'loading' | 'ready' | 'denied' | 'error'
  dialog: { kind: 'add' } | { kind: 'remove'; id: string } | null
  busy: boolean
  error: string | null
  reauthentication: boolean
  emailSent: boolean
}
export interface PasskeyController {
  store: StoreApi<PasskeyState>
  load(): Promise<void>
  openAdd(): void
  openRemove(id: string): void
  close(): void
  add(name: string): Promise<boolean>
  remove(id: string): Promise<boolean>
  verifyPasskey(): Promise<boolean>
  sendEmail(): Promise<boolean>
  verifyEmail(code: string): Promise<boolean>
}
