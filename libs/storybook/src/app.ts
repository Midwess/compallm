import type { StoreApi } from 'zustand/vanilla'

export interface AppState {
  loading: boolean
  error: string | null
}
export interface AppLoadingTask {
  complete(): void
  cancel(): void
  fail(error: unknown, retry: () => Promise<unknown>): void
}
export interface AppController {
  beginLoading(key: string): AppLoadingTask
  store: StoreApi<AppState>
  retry(): Promise<void>
}
