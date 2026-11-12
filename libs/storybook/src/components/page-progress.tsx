import { usePageProgressMotion } from './motion'

export function PageProgress({ active }: { active: boolean }) {
  const ref = usePageProgressMotion(active)
  return (
    <div
      data-ui="page-progress"
      className="pointer-events-none fixed inset-x-0 top-0 z-50 h-0.5 scale-y-[1.15] origin-top"
      role="progressbar"
      aria-label="Loading requests"
      aria-hidden={!active}
    >
      <div ref={ref} className="h-full w-full origin-left bg-primary opacity-0" />
    </div>
  )
}
