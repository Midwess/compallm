import { createContext } from 'svelte'
export interface TableColumn {
  id: string
  class?: string
  visibilityClassName?: string
  loadingWidthClassName?: string
}
export const [getColumns, setColumns] = createContext<() => TableColumn[]>()
const columnDisplays: Record<string, string> = {
  'table-cell': 'table-column',
  'sm:table-cell': 'sm:table-column',
  'md:table-cell': 'md:table-column',
  'lg:table-cell': 'lg:table-column',
  'xl:table-cell': 'xl:table-column',
  '2xl:table-cell': '2xl:table-column',
}
export function columnDisplayClass(visibility?: string) {
  return visibility
    ?.split(/\s+/)
    .map((value) => columnDisplays[value] ?? value)
    .join(' ')
}
export function columnVisibility() {
  let columns: () => TableColumn[]
  try {
    columns = getColumns()
  } catch {
    columns = () => []
  }
  return (id?: string) => (id ? columns().find((column) => column.id === id)?.visibilityClassName : undefined)
}
export const [getVisibleColumns, setVisibleColumns] = createContext<() => number | undefined>()
