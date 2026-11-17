import type { CollectionForms } from './collection-forms'

export interface CollectionRowScope {
  index: number
  key: string
  forms: CollectionForms
}
export interface CollectionScope {
  scrollElement: HTMLElement | null
  forms: CollectionForms
}
export interface CollectionPaging {
  hasMore: boolean
  loading: boolean
  error?: boolean
  onLoadMore(): void
}
