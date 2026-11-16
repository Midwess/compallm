import { useContext, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { AppControllerContext } from '@compallm/storybook/context'
import { AppLoadingOverlay } from '@compallm/storybook/components/app-loading-overlay'

export function AppLoading({ children }: { children: ReactNode }) {
  const controller = useContext(AppControllerContext)
  if (!controller) throw new Error('Root AppControllerContext is required.')
  const active = useStore(controller.store, (state) => state.loading)
  const error = useStore(controller.store, (state) => state.error)
  return (
    <>
      <div inert={active} aria-busy={active}>
        {children}
      </div>
      <AppLoadingOverlay active={active} error={error ?? undefined} onRetry={() => void controller.retry()} />
    </>
  )
}
