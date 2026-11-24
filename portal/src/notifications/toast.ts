import { createStore } from '@compallm/storybook/state'
import { ApiError } from '../graphql-client'
export interface Notice {
  id: string
  tone: 'info' | 'warning' | 'error'
  message: string
  action?: { label: string; onClick(): void }
}
export const notifications = createStore(() => ({ records: [] as Notice[] }))
const timers = new Map<string, ReturnType<typeof setTimeout>>()
let sequence = 0
export function apiErrorMessage(error: unknown): string {
  if (error instanceof ApiError) return error.message
  if (
    error &&
    typeof error === 'object' &&
    'info' in error &&
    error.info &&
    typeof error.info === 'object' &&
    'message' in error.info &&
    typeof error.info.message === 'string'
  )
    return error.info.message
  return 'Something went wrong. Please try again.'
}
export function dismissNotice(id: string) {
  clearTimeout(timers.get(id))
  timers.delete(id)
  notifications.setState((state) => ({ records: state.records.filter((record) => record.id !== id) }))
}
function show(tone: Notice['tone'], message: string, action?: Notice['action']) {
  const id = action ? `${tone}:${message}:${++sequence}` : `${tone}:${message}`
  clearTimeout(timers.get(id))
  notifications.setState((state) => ({
    records: [
      ...state.records.filter((record) => record.id !== id),
      {
        id,
        tone,
        message,
        action: action && {
          ...action,
          onClick() {
            dismissNotice(id)
            action.onClick()
          },
        },
      },
    ],
  }))
  timers.set(
    id,
    setTimeout(() => dismissNotice(id), 15000),
  )
  return id
}
export const notify = {
  info: (message: string) => show('info', message),
  warning: (message: string) => show('warning', message),
  error: (error: unknown) => {
    if (!(error instanceof DOMException && error.name === 'AbortError')) return show('error', apiErrorMessage(error))
  },
  apiError: (error: unknown, retry: () => void) => {
    if (!(error instanceof DOMException && error.name === 'AbortError'))
      return show('error', apiErrorMessage(error), { label: 'Retry', onClick: retry })
  },
}
export function clearNotices() {
  for (const id of timers.keys()) dismissNotice(id)
}
