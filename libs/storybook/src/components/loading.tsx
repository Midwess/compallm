import { Skeleton as RadixSkeleton } from '@radix-ui/themes/components/skeleton'
import { useContext, type ComponentProps, type HTMLAttributes, type ReactNode } from 'react'
import { useStore } from 'zustand'
import { LoadingRegionContext } from '../context'
import type { LoadingController } from '../app'
import { cx } from './shared'

export function useInitialLoading() {
  return useContext(LoadingRegionContext)
}

export function Skeleton({ loading, className, ...props }: ComponentProps<typeof RadixSkeleton>) {
  const initial = useInitialLoading()
  return (
    <RadixSkeleton
      {...props}
      loading={loading ?? initial}
      className={cx(
        `pointer-events-none select-none border-transparent! bg-foreground/10! text-transparent!
        caret-transparent shadow-none! placeholder:text-transparent! *:invisible!`,
        className,
      )}
    />
  )
}

function Region({
  ready,
  pending = false,
  label,
  children,
  ...props
}: HTMLAttributes<HTMLDivElement> & { ready: boolean; pending?: boolean; label: string; children: ReactNode }) {
  return (
    <div {...props} aria-busy={pending || undefined}>
      {pending ? (
        <span role="status" className="sr-only">
          {label}
        </span>
      ) : null}
      <LoadingRegionContext value={!ready}>
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
