import { createStore } from 'zustand/vanilla'
import type { AppController as Contract, AppState, AppLoadingTask } from '@compallm/storybook'
import { apiErrorMessage } from '../notifications/toast'

type Operation = { error?: string; retry?: () => Promise<unknown> }

export class AppController implements Contract {
  readonly store = createStore<AppState>(() => ({ loading: false, error: null }))
  private operations = new Map<string, Operation>()
  private shownAt = 0
  private timer?: ReturnType<typeof setTimeout>

  beginLoading = (key: string): AppLoadingTask => {
    if (!this.store.getState().loading) this.shownAt = Date.now()
    clearTimeout(this.timer)
    const operation: Operation = {}
    this.operations.set(key, operation)
    this.synchronize()
    const finish = () => {
      if (this.operations.get(key) !== operation) return
      this.operations.delete(key)
      this.synchronize()
    }
    return {
      complete: finish,
      cancel: finish,
      fail: (error, retry) => {
        if (this.operations.get(key) !== operation) return
        operation.error = apiErrorMessage(error)
        operation.retry = retry
        this.synchronize()
      },
    }
  }

  private synchronize = () => {
    clearTimeout(this.timer)
    const error = [...this.operations.values()].find((operation) => operation.error)?.error ?? null
    if (this.operations.size) {
      this.store.setState({ loading: true, error })
      return
    }
    const remaining = Math.max(0, 1000 - (Date.now() - this.shownAt))
    if (remaining) {
      this.store.setState({ loading: true, error: null })
      this.timer = setTimeout(this.synchronize, remaining)
    } else this.store.setState({ loading: false, error: null })
  }

  retry = async () => {
    const failed = [...this.operations.values()].filter((operation) => operation.retry)
    await Promise.allSettled(failed.map((operation) => operation.retry!()))
  }
}

export const appController = new AppController()
