import { createContext, useContext, useId, type HTMLAttributes } from 'react'
import { cx } from './shared'
import { SectionSurface } from './surfaces'
const SectionContext = createContext<string | null>(null)
// eslint-disable-next-line design/spacing -- The connector is a 1px hairline.
const connectorWidth = 'w-px'
function Root({ children, className, ...props }: HTMLAttributes<HTMLElement>) {
  const id = useId()
  return (
    <SectionContext value={id}>
      <section {...props} aria-labelledby={id} className={cx('group/section relative grid min-w-0 gap-4 pl-4', className)}>
        <div
          aria-hidden="true"
          className={cx(
            'absolute top-3 -bottom-9 -left-0.5 -translate-x-1/2 bg-border group-last-of-type/section:hidden',
            connectorWidth,
          )}
        />
        {children}
      </section>
    </SectionContext>
  )
}
function Heading(props: HTMLAttributes<HTMLHeadingElement>) {
  const id = useContext(SectionContext)
  if (!id) throw new Error('FormSection.Heading requires FormSection.')
  return <h2 {...props} id={id} className={cx('flex h-6 items-center px-4 text-sm font-medium', props.className)} />
}
function Marker(props: HTMLAttributes<HTMLSpanElement>) {
  return (
    <span
      {...props}
      aria-hidden="true"
      className={cx(
        `absolute top-0.5 -left-3 flex size-5 items-center justify-center rounded-full
border border-border bg-elevated text-xs text-muted`,
        props.className,
      )}
    />
  )
}
export const FormSection = Object.assign(Root, { Heading, Marker, Content: SectionSurface })
