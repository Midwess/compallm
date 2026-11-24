import type { StateStore } from '../../state.svelte'
import type { PasskeySummary } from '../api-types'
export interface PasskeyState {
  loaded?: boolean
  records: Pick<PasskeySummary, 'id' | 'name' | 'registeredAt' | 'lastUsedAt'>[]
  status: 'loading' | 'ready' | 'denied' | 'error'
  dialog: { kind: 'verify' } | { kind: 'add' } | { kind: 'remove'; id: string } | null
  enrollment: 'device' | 'name'
  editingId: string | null
  currentPasskeyId: string | null
  busy: boolean
  error: string | null
  reauthentication: boolean
  emailSent: boolean
}
export interface PasskeyController {
  store: StateStore<PasskeyState>
  load(): Promise<void>
  openAdd(): void
  openRemove(id: string): void
  close(): void
  prepare(): Promise<boolean>
  edit(id: string | null): void
  rename(id: string, name: string): Promise<boolean>
  add(name: string): Promise<boolean>
  remove(id: string): Promise<boolean>
}
