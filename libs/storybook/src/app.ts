import type { StateStore } from './state.svelte'
export interface AppState {
  loading: boolean
  error: string | null
}
export interface AppLoadingTask {
  complete(): void
  cancel(): void
  fail(error: unknown, retry: () => Promise<unknown>): void
}
export interface LoadingController {
  beginLoading(key: string): AppLoadingTask
  store: StateStore<AppState>
  retry(): Promise<void>
}
export interface AppController extends LoadingController {
  createLoadingScope(): LoadingController
}
