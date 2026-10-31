import type { ReactNode } from 'react'
import { cx } from '../../src/components/shared'

export function Preview({ children }: { children: ReactNode }) {
  return (
    <main data-ui="component-preview" className="flex flex-wrap items-start gap-4 p-4 font-sans text-[13px] text-text">
      {children}
    </main>
  )
}
export function Sample({ label, children, wide = false }: { label: string; children: ReactNode; wide?: boolean }) {
  return (
    <section className={cx('min-w-0 max-w-full has-[[role=tooltip]]:pt-8', wide ? 'w-[720px]' : 'w-[352px]')}>
      <h3 className="mb-2 text-[12px] leading-[20px] font-normal text-muted">{label}</h3>
      <div
        className={cx(
          'flex w-full flex-col items-start gap-2 [&>div:not([data-ui=dropdown])]:w-full',
          '[&>input]:w-full [&>textarea]:w-full [&>figure]:w-full',
        )}
      >
        {children}
      </div>
    </section>
  )
}
export function Row({ children }: { children: ReactNode }) {
  return <div className="flex max-w-full flex-wrap items-center gap-2">{children}</div>
}
