import { toast } from 'sonner'
import { Toast, type ToastProps } from '@compallm/storybook/components/feedback'
import get from 'lodash-es/get'
import isObjectLike from 'lodash-es/isObjectLike'

const fallbackMessage = 'Something went wrong. Please try again.'
let actionToastSequence = 0

export function apiErrorMessage(error: unknown): string {
  if (!isObjectLike(error)) return fallbackMessage
  const payload: unknown = get(error, 'info')
  if (!isObjectLike(payload)) return fallbackMessage
  const message: unknown = get(payload, 'message')
  return typeof message === 'string' && message.trim() ? message : fallbackMessage
}

function show(tone: NonNullable<ToastProps['tone']>, message: string, action?: ToastProps['action']) {
  return toast.custom(
    (id) => (
      <Toast
        tone={tone}
        message={message}
        action={
          action
            ? {
                ...action,
                onClick: () => {
                  toast.dismiss(id)
                  action.onClick()
                },
              }
            : undefined
        }
        onDismiss={() => toast.dismiss(id)}
      />
    ),
    {
      id: action ? `${tone}:${message}:action:${++actionToastSequence}` : `${tone}:${message}`,
      duration: 15_000,
    },
  )
}

export const notify = {
  info: (message: string) => show('info', message),
  warning: (message: string) => show('warning', message),
  error: (error: unknown) => {
    if (isObjectLike(error) && get(error, 'name') === 'AbortError') return
    return show('error', apiErrorMessage(error))
  },
  apiError: (error: unknown, retry: () => void) => {
    if (isObjectLike(error) && get(error, 'name') === 'AbortError') return
    return show('error', apiErrorMessage(error), { label: 'Retry', onClick: retry })
  },
}
