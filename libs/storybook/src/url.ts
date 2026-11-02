import type { StoreApi } from 'zustand/vanilla'

export interface UrlState {
  pathname: string
  search: string
  hash: string
}
export interface UrlController {
  readonly store: Pick<StoreApi<UrlState>, 'getState' | 'getInitialState' | 'subscribe'>
  navigate(href: string, options?: { replace?: boolean }): void
  update(patch: Record<string, string | null>, options?: { pathname?: string; replace?: boolean }): void
}
export interface NavigationRequest {
  discard(): void
  keep(): void
}
