import { createStore } from 'zustand/vanilla'
import type { createBrowserRouter } from 'react-router'
import type { UrlController as Contract, UrlState, NavigationRequest } from '@compallm/storybook'
import { notify } from '../notifications/toast'

type Router = ReturnType<typeof createBrowserRouter>
const MAX_URL_LENGTH = 16 * 1024
type ReturnLocation = { index: number; href: string }
function returnLocations(state: unknown): ReturnLocation[] {
  if (!state || typeof state !== 'object' || !('navigationParents' in state) || !Array.isArray(state.navigationParents))
    return []
  return state.navigationParents.filter(
    (value): value is ReturnLocation =>
      !!value &&
      typeof value === 'object' &&
      Number.isInteger(value.index) &&
      value.index >= 0 &&
      typeof value.href === 'string',
  )
}
function canonical(href: string) {
  const value = new URL(href, window.location.origin)
  value.searchParams.sort()
  return value.pathname + value.search + value.hash
}
function stateOf(url: URL): UrlState {
  return { pathname: url.pathname, search: url.search, hash: url.hash }
}
export class UrlController implements Contract {
  readonly store = createStore<UrlState>(() => ({ pathname: '/', search: '', hash: '' }))
  private router: Router | undefined
  private unsubscribe: (() => void) | undefined
  private intended: URL | undefined
  private guard: { shouldBlock(url: URL): boolean; onBlocked(request: NavigationRequest): void } | undefined
  private blockedKey: string | undefined
  connect(router: Router) {
    this.unsubscribe?.()
    this.router = router
    this.intended = new URL(
      router.state.location.pathname + router.state.location.search + router.state.location.hash,
      window.location.origin,
    )
    this.store.setState(stateOf(this.intended))
    router.getBlocker(
      'url-controller',
      ({ nextLocation }) =>
        nextLocation.pathname !== '/login' &&
        !!this.guard?.shouldBlock(new URL(nextLocation.pathname + nextLocation.search, window.location.origin)),
    )
    this.unsubscribe = router.subscribe((state) => {
      const blocked = state.blockers.get('url-controller')
      if (blocked?.state === 'blocked') {
        const key = blocked.location.key
        if (this.blockedKey !== key) {
          this.blockedKey = key
          this.guard?.onBlocked({
            discard: () => {
              this.blockedKey = undefined
              blocked.proceed()
            },
            keep: () => {
              this.blockedKey = undefined
              this.intended = new URL(window.location.href)
              blocked.reset()
            },
          })
        }
        return
      }
      if (state.navigation.state !== 'idle') return
      this.intended = new URL(state.location.pathname + state.location.search + state.location.hash, window.location.origin)
      this.store.setState(stateOf(this.intended))
    })
    // Sanitize forbidden credential parameters even on a pasted initial URL.
    const initial = this.intended
    const clean = new URL(initial)
    for (const key of [...clean.searchParams.keys()])
      if (/^(api_?key|password|code|token|access_?token|refresh_?token)$/i.test(key)) clean.searchParams.delete(key)
    if (clean.href !== initial.href) this.navigate(clean.pathname + clean.search + clean.hash, { replace: true })
    return this.unsubscribe
  }
  block(shouldBlock: (url: URL) => boolean, onBlocked: (request: NavigationRequest) => void) {
    const guard = { shouldBlock, onBlocked }
    this.guard = guard
    return () => {
      if (this.guard === guard) this.guard = undefined
    }
  }
  retryFailedRoute = async () => {
    if (this.router?.state.errors) await this.router.revalidate()
  }
  navigate = (href: string, options: { replace?: boolean } = {}) => {
    if (!this.router) throw new Error('URL controller is not connected.')
    const target = new URL(href, this.intended ?? window.location.href)
    if (target.origin !== window.location.origin) throw new Error('Application navigation must stay on the same origin.')
    for (const key of [...target.searchParams.keys()])
      if (/^(api_?key|password|code|token|access_?token|refresh_?token)$/i.test(key)) target.searchParams.delete(key)
    const path = target.pathname + target.search + target.hash
    if (path.length > MAX_URL_LENGTH) {
      notify.warning('This draft is too large to store in the URL. Reduce the number or size of models.')
      return false
    }
    if (this.intended?.href === target.href) return true
    this.intended = target
    const location = this.router.state.location
    const index = window.history?.state?.idx
    const parents = returnLocations(location.state).filter((value) => typeof index !== 'number' || value.index < index)
    if (!options.replace && typeof index === 'number')
      parents.push({ index, href: location.pathname + location.search + location.hash })
    void this.router
      .navigate(path, {
        replace: options.replace,
        preventScrollReset: true,
        state: { ...location.state, navigationParents: parents.slice(-64) },
      })
      .catch((error: unknown) => notify.error(error))
    return true
  }
  back = (fallbackHref: string, options: { steps?: number; matchTarget?: boolean } = {}) => {
    if (!this.router) throw new Error('URL controller is not connected.')
    const index = window.history?.state?.idx
    const parent = options.matchTarget
      ? returnLocations(this.router.state.location.state).findLast(
          (entry) => typeof index === 'number' && entry.index < index && canonical(entry.href) === canonical(fallbackHref),
        )
      : undefined
    const steps = options.matchTarget
      ? parent && typeof index === 'number'
        ? index - parent.index
        : 0
      : Math.max(1, Math.trunc(options.steps ?? 1))
    if (typeof index === 'number' && steps > 0 && index >= steps) {
      this.intended = new URL(fallbackHref, window.location.origin)
      void this.router.navigate(-steps).catch((error: unknown) => notify.error(error))
      return true
    }
    return this.navigate(fallbackHref, { replace: true })
  }
  update = (patch: Record<string, string | null>, options: { pathname?: string; replace?: boolean } = {}) => {
    const target = new URL(this.intended ?? window.location.href)
    if (options.pathname) target.pathname = options.pathname
    for (const [key, value] of Object.entries(patch)) {
      if (value === null) target.searchParams.delete(key)
      else target.searchParams.set(key, value)
    }
    return this.navigate(target.pathname + target.search + target.hash, { replace: options.replace ?? true })
  }
}
export const urlController = new UrlController()
