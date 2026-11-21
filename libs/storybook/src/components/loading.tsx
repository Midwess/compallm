import { Skeleton as RadixSkeleton } from '@radix-ui/themes/components/skeleton'
import { useContext, useEffect, useState, type ComponentProps, type HTMLAttributes, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { LoadingRegionContext } from '../context'
import type { LoadingController } from '../app'
import { animation } from './motion'
import { cx } from './shared'

export function useInitialLoading() {
  return useContext(LoadingRegionContext)?.initial ?? false
}

export function useRefreshingSkeleton() {
  const region = useContext(LoadingRegionContext)
  return !!region?.refreshing && region.visible
}

function useSkeletonPresentation(active: boolean, initial = active) {
  const [state, setState] = useState({ active, initial, visible: false, shownAt: 0 })
  if (state.active !== active) {
    setState(
      active
        ? state.visible
          ? { ...state, active, initial }
          : { active, initial, visible: false, shownAt: 0 }
        : { ...state, active },
    )
  }
  useEffect(() => {
    if (active && !state.visible) {
      const timer = setTimeout(
        () => setState({ active, initial, visible: true, shownAt: Date.now() }),
        animation.skeleton.delayMs,
      )
      return () => clearTimeout(timer)
    }
    if (!active && state.visible) {
      const remaining = Math.max(0, animation.skeleton.minimumVisibleMs - (Date.now() - state.shownAt))
      const timer = setTimeout(() => setState((value) => ({ ...value, visible: false })), remaining)
      return () => clearTimeout(timer)
    }
  }, [active, initial, state.visible, state.shownAt])
  return { visible: state.visible, initial: active ? initial : state.visible && state.initial }
}

export function LoadingFallback({
  pending,
  fallback,
  children,
}: {
  pending: boolean
  fallback: ReactNode
  children: ReactNode
}) {
  const presentation = useSkeletonPresentation(pending)
  return pending || presentation.visible ? fallback : children
}

function Placeholder({ loading, visible, className, ...props }: ComponentProps<typeof RadixSkeleton> & { visible: boolean }) {
  return (
    <RadixSkeleton
      {...props}
      loading={loading}
      className={cx(
        `pointer-events-none select-none border-transparent! bg-foreground/10! text-transparent!
        caret-transparent shadow-none! placeholder:text-transparent! *:invisible!`,
        !visible && 'opacity-0!',
        className,
      )}
    />
  )
}

function StandaloneSkeleton(props: ComponentProps<typeof RadixSkeleton>) {
  const active = props.loading ?? false
  const presentation = useSkeletonPresentation(active)
  return <Placeholder {...props} loading={active || presentation.visible} visible={presentation.visible} />
}

export function Skeleton({ loading, ...props }: ComponentProps<typeof RadixSkeleton>) {
  const region = useContext(LoadingRegionContext)
  if (!region) return <StandaloneSkeleton {...props} loading={loading} />
  return <Placeholder {...props} loading={loading ?? region.initial} visible={region.visible} />
}

export function LoadingScope({
  ready,
  refreshing = false,
  children,
}: {
  ready: boolean
  refreshing?: boolean
  children: ReactNode
}) {
  const parent = useContext(LoadingRegionContext)
  const initial = !ready
  const presentation = useSkeletonPresentation(initial || refreshing, initial)
  const inherited = !initial && !refreshing && parent?.refreshing
  return (
    <LoadingRegionContext
      value={{
        initial: initial || presentation.initial,
        visible: inherited ? parent.visible : presentation.visible,
        refreshing: refreshing || !!inherited || (!presentation.initial && presentation.visible),
      }}
    >
      {children}
    </LoadingRegionContext>
  )
}

function Region({
  ready,
  pending = false,
  refreshing = false,
  label,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement> & {
  ready: boolean
  pending?: boolean
  refreshing?: boolean
  label: string
  children: ReactNode
}) {
  return (
    <div {...props} aria-busy={pending || undefined}>
      {pending ? (
        <span role="status" className="sr-only">
          {label}
        </span>
      ) : null}
      <LoadingScope ready={ready} refreshing={refreshing}>
        <RegionContent ready={ready}>{children}</RegionContent>
      </LoadingScope>
    </div>
  )
}

function RegionContent({ children }: { ready: boolean; children: ReactNode }) {
  const initial = useInitialLoading()
  return (
    <div className="contents" inert={initial || undefined}>
      {children}
    </div>
  )
}

function ControlledRegion({ controller, ...props }: ComponentProps<typeof Region> & { controller: LoadingController }) {
  const pending = useStore(controller.store, (state) => state.loading && !state.error)
  return <Region {...props} pending={pending} />
}

export function LoadingRegion({ controller, ...props }: ComponentProps<typeof Region> & { controller?: LoadingController }) {
  return controller ? <ControlledRegion {...props} controller={controller} /> : <Region {...props} />
}
