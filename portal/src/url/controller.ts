import { createStore } from '@compallm/storybook/state'
import type { NavigationRequest, UrlState } from '@compallm/storybook'
import { notify } from '../notifications/toast'
type Location = { index: number; href: string }
type NavigationState = { compallmNavigationIndex: number; navigationParents: Location[] }
function stateOf(value: unknown): NavigationState {
  if (!value || typeof value !== 'object') return { compallmNavigationIndex: 0, navigationParents: [] }
  const state = value as Partial<NavigationState>
  return {
    compallmNavigationIndex: Number.isInteger(state.compallmNavigationIndex) ? state.compallmNavigationIndex! : 0,
    navigationParents: Array.isArray(state.navigationParents)
      ? state.navigationParents.filter(
          (location) => Number.isInteger(location?.index) && location.index >= 0 && typeof location.href === 'string',
        )
      : [],
  }
}
function canonical(href: string) {
  const value = new URL(href, window.location.origin)
  value.searchParams.sort()
  return value.pathname + value.search + value.hash
}
export class UrlController {
  readonly store = createStore<UrlState>(() => ({ pathname: '/', search: '', hash: '' }))
  private router?: (href: string, replace: boolean, state?: NavigationState) => Promise<void>
  private intended?: URL
  private history = stateOf(undefined)
  private guard?: { shouldBlock(url: URL): boolean; onBlocked(request: NavigationRequest): void }
  private skipGuard = false
  connect(router: (href: string, replace: boolean, state?: NavigationState) => Promise<void>) {
    this.router = router
    return () => {
      if (this.router === router) this.router = undefined
    }
  }
  synchronize(url: URL, state?: unknown) {
    this.intended = new URL(url)
    if (state !== undefined) this.history = stateOf(state)
    this.store.setState({ pathname: url.pathname, search: url.search, hash: url.hash })
  }
  block(shouldBlock: (url: URL) => boolean, onBlocked: (request: NavigationRequest) => void) {
    const guard = { shouldBlock, onBlocked }
    this.guard = guard
    return () => {
      if (this.guard === guard) this.guard = undefined
    }
  }
  beforeNavigation(target: URL, cancel: () => void, proceed: () => void) {
    if (this.skipGuard) {
      this.skipGuard = false
      return true
    }
    if (target.pathname === '/login' || !this.guard?.shouldBlock(target)) return true
    cancel()
    this.guard.onBlocked({
      discard: () => {
        this.skipGuard = true
        proceed()
      },
      keep: () => {
        this.intended = new URL(window.location.href)
      },
    })
    return false
  }
  navigate = async (href: string, options: { replace?: boolean } = {}) => {
    if (!this.router) throw new Error('Navigation is unavailable')
    const target = new URL(href, this.intended ?? window.location.href)
    if (target.origin !== window.location.origin) throw new Error('Navigation must stay on the same origin')
    for (const key of [...target.searchParams.keys()])
      if (/^(api_?key|password|code|token|access_?token|refresh_?token)$/i.test(key)) target.searchParams.delete(key)
    const path = target.pathname + target.search + target.hash
    if (path.length > 16384) {
      notify.warning('This draft is too large to store in the URL. Reduce the number or size of models.')
      return false
    }
    if (target.href === this.intended?.href && !options.replace) return true
    if (!this.skipGuard && target.pathname !== '/login' && this.guard?.shouldBlock(target)) {
      const approved = await new Promise<boolean>((resolve) =>
        this.guard!.onBlocked({
          discard: () => resolve(true),
          keep: () => resolve(false),
        }),
      )
      if (!approved) return false
      this.skipGuard = true
    }
    const previous = this.intended
    this.intended = target
    const current = this.store.getState(),
      index = this.history.compallmNavigationIndex
    const parents = this.history.navigationParents.filter((location) => location.index < index)
    if (!options.replace) parents.push({ index, href: current.pathname + current.search + current.hash })
    const state = { compallmNavigationIndex: options.replace ? index : index + 1, navigationParents: parents.slice(-64) }
    try {
      await this.router(path, options.replace ?? false, state)
      return true
    } catch (error) {
      if (this.intended === target) this.intended = previous
      throw error
    } finally {
      this.skipGuard = false
    }
  }
  update = (patch: Record<string, string | null>, options: { pathname?: string; replace?: boolean } = {}) => {
    const target = new URL(this.intended ?? window.location.href)
    if (options.pathname) target.pathname = options.pathname
    for (const [key, value] of Object.entries(patch))
      if (value === null) target.searchParams.delete(key)
      else target.searchParams.set(key, value)
    return this.navigate(target.pathname + target.search + target.hash, { replace: options.replace ?? true })
  }
  back = (fallback: string, options: { steps?: number; matchTarget?: boolean } = {}) => {
    const index = this.history.compallmNavigationIndex
    const parent = options.matchTarget
      ? this.history.navigationParents.findLast(
          (location) => location.index < index && canonical(location.href) === canonical(fallback),
        )
      : undefined
    const steps = options.matchTarget ? (parent ? index - parent.index : 0) : Math.max(1, Math.trunc(options.steps ?? 1))
    if (steps > 0 && index >= steps) {
      this.intended = new URL(fallback, window.location.origin)
      window.history.go(-steps)
      return Promise.resolve(true)
    }
    return this.navigate(fallback, { replace: true })
  }
}
