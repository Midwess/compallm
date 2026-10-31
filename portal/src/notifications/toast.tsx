import { toast } from 'sonner'
import { Toast, type ToastProps } from '@compallm/storybook/components/feedback'

const fallbackMessage = 'Something went wrong. Please try again.'

export function apiErrorMessage(error: unknown): string {
  if (typeof error !== 'object' || error === null || !('info' in error)) return fallbackMessage
  const payload = error.info
  if (typeof payload !== 'object' || payload === null || !('message' in payload)) return fallbackMessage
  return typeof payload.message === 'string' && payload.message.trim() ? payload.message : fallbackMessage
}

function show(tone: NonNullable<ToastProps['tone']>, message: string) {
  return toast.custom((id) => <Toast tone={tone} message={message} onDismiss={() => toast.dismiss(id)} />, {
    id: `${tone}:${message}`,
    duration: tone === 'info' ? 5000 : Infinity,
  })
}

export const notify = {
  info: (message: string) => show('info', message),
  warning: (message: string) => show('warning', message),
  error: (error: unknown) => {
    if (typeof error === 'object' && error !== null && 'name' in error && error.name === 'AbortError') return
    return show('error', apiErrorMessage(error))
  },
}
