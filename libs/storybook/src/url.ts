import type { StateStore } from './state.svelte'

export interface UrlState {
  pathname: string
  search: string
  hash: string
}
export interface UrlController {
  readonly store: Pick<StateStore<UrlState>, 'getState' | 'getInitialState' | 'subscribe'>
  navigate(href: string, options?: { replace?: boolean }): void
  back(fallbackHref: string, options?: { steps?: number; matchTarget?: boolean }): void
  update(patch: Record<string, string | null>, options?: { pathname?: string; replace?: boolean }): void
}
export interface NavigationRequest {
  discard(): void
  keep(): void
}
