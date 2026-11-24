import { untrack } from 'svelte'

export interface StateStore<T> {
  readonly state: T
  getState(): T
  getInitialState(): T
  setState(update: Partial<T> | ((state: T) => Partial<T>)): void
  subscribe(listener: (state: T, previous: T) => void): () => void
}
export function createStore<T>(initial: () => T): StateStore<T> {
  const first = initial()
  const state = $state(first)
  const listeners = new Set<(state: T, previous: T) => void>()
  return {
    state,
    getState: () => state,
    getInitialState: () => first,
    setState(update) {
      untrack(() => {
        const patch = typeof update === 'function' ? update(state) : update
        if (!Object.keys(patch).some((key) => !Object.is(patch[key as keyof T], state[key as keyof T]))) return
        const previous = listeners.size ? { ...state } : state
        Object.assign(state as object, patch)
        listeners.forEach((listener) => listener(state, previous))
      })
    },
    subscribe(listener) {
      listeners.add(listener)
      return () => {
        listeners.delete(listener)
      }
    },
  }
}
