import type { HTMLAttributes } from 'react'
import { cx } from './shared'

export function SectionSurface({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('rounded-[10px] bg-inset p-4', className)} />
}
