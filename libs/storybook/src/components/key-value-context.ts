import { createContext } from 'svelte'
export interface KeyValueEntry {
  id: string
  key: string
  value: string
}
export const [getKeyValue, setKeyValue] = createContext<{
  readonly entries: KeyValueEntry[]
  readonly disabled: boolean
  keyMaxLength?: number
  acceptEntry?: (entry: KeyValueEntry) => boolean
  change(entries: KeyValueEntry[]): void
}>()
