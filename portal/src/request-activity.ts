import { createStore } from 'zustand/vanilla'

export const requestActivity = createStore(() => ({ pending: 0 }))

export function beginRequest() {
  requestActivity.setState(({ pending }) => ({ pending: pending + 1 }))
  let finished = false
  return () => {
    if (finished) return
    finished = true
    requestActivity.setState(({ pending }) => ({ pending: pending - 1 }))
  }
}
