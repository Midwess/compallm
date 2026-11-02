import { createStore } from 'zustand/vanilla'
import type { createBrowserRouter } from 'react-router'
import type { UrlController as Contract, UrlState, NavigationRequest } from '@compallm/storybook'
import { notify } from '../notifications/toast'

type Router = ReturnType<typeof createBrowserRouter>
const MAX_URL_LENGTH = 16 * 1024
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
    void this.router
      .navigate(path, { replace: options.replace, preventScrollReset: true })
      .catch((error: unknown) => notify.error(error))
    return true
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
