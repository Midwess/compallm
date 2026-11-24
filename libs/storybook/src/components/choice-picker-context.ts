import { createContext } from 'svelte'
export interface ChoiceOption {
  id: string
  label: string
  disabled?: boolean
  color?: string
  description?: string
}
export const [getChoice, setChoice] = createContext<{
  readonly options: ChoiceOption[]
  readonly selected: string[]
  readonly name?: string
  readonly invalid: boolean
  readonly query: string
  setQuery(query: string): void
  readonly open: boolean
  readonly trigger: HTMLElement | null
  setTrigger(node: HTMLElement | null): void
  toggle(id: string): void
}>()
