import { createContext } from 'svelte'
export const [getSelect, setSelect] = createContext<{
  readonly name?: string
  readonly invalid?: boolean
  readonly value: string
  readonly items: { value: string; label: string }[]
}>()
