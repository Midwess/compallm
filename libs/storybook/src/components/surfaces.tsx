import type { HTMLAttributes } from 'react'
import { cx } from './shared'

export function SectionSurface({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('rounded-xl bg-inset p-4', className)} />
}
