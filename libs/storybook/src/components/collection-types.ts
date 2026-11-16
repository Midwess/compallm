import type { CollectionForms } from './collection-forms'

export interface CollectionRowScope {
  index: number
  measure(element: HTMLElement | null): void
  key: string
  forms: CollectionForms
  retain(reason: object): () => void
}
export interface CollectionScope {
  viewport: import('./collection-viewport').CollectionViewport
  scrollElement: HTMLElement | null
  forms: CollectionForms
}
export interface CollectionPaging {
  hasMore: boolean
  loading: boolean
  error?: boolean
  onLoadMore(): void
}
