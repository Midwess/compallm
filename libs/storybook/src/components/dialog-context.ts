import { createContext } from 'svelte'
export const [getDialog, setDialog] = createContext<{
  readonly open: boolean
  setOpen(value: boolean): void
  close(): void
  titleId: string
  descriptionId: string
  registerDiscard(value: (() => void) | undefined): void
}>()
