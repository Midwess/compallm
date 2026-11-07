import type { HTMLAttributes } from 'react'
import { cx } from './shared'

export function PageHeader({ className, ...props }: HTMLAttributes<HTMLElement>) {
  return (
    <header
      {...props}
      data-ui="page-header"
      className={cx(
        'sticky top-0 z-10 flex h-10 min-w-0 shrink-0 items-center gap-4',
        'border-b-[0.5px] border-divider bg-surface px-4',
        className,
      )}
    />
  )
}

export function Title({ className, ...props }: HTMLAttributes<HTMLHeadingElement>) {
  return <h1 {...props} className={cx('m-0 min-w-0 flex-1 truncate text-xs leading-5 font-normal text-text', className)} />
}

export function Functions({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('ml-auto flex min-w-0 shrink-0 items-center gap-2', className)} />
}
