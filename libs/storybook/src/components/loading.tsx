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

function useSkeletonVisibility(active: boolean) {
  const [state, setState] = useState({ active, visible: false })
  if (state.active !== active) setState({ active, visible: false })
  useEffect(() => {
    if (!active) return
    const timer = setTimeout(() => setState({ active, visible: true }), animation.skeleton.delayMs)
    return () => clearTimeout(timer)
  }, [active])
  return active && state.active === active && state.visible
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
  const visible = useSkeletonVisibility(active)
  return <Placeholder {...props} loading={active} visible={visible} />
}

export function Skeleton({ loading, ...props }: ComponentProps<typeof RadixSkeleton>) {
  const region = useContext(LoadingRegionContext)
  if (!region) return <StandaloneSkeleton {...props} loading={loading} />
  return <Placeholder {...props} loading={loading ?? region.initial} visible={region.visible} />
}

function Region({
  ready,
  pending = false,
  label,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement> & { ready: boolean; pending?: boolean; label: string; children: ReactNode }) {
  const initial = !ready
  const visible = useSkeletonVisibility(initial)
  return (
    <div {...props} aria-busy={pending || undefined}>
      {pending ? (
        <span role="status" className="sr-only">
          {label}
        </span>
      ) : null}
      <LoadingRegionContext value={{ initial, visible }}>
        <div className="contents" inert={!ready || undefined}>
          {children}
        </div>
      </LoadingRegionContext>
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
