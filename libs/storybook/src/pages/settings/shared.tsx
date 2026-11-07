import type { HTMLAttributes, ReactNode } from 'react'
import { SectionSurface } from '../../components/surfaces'
import { SettingsDivider, SettingsPageTitle, SettingsSectionHeader } from '../../components/settings'
import { SettingsSectionTitle, SettingsSectionDescription } from '../../components/settings'
import { SettingsRow } from '../../components/content'
import { cx } from '../../components/shared'

export function SettingsContent({ wide = false, className, ...props }: HTMLAttributes<HTMLElement> & { wide?: boolean }) {
  return (
    <article
      {...props}
      className={cx('mx-auto w-full py-16', wide ? 'min-w-0 px-6 lg:px-14 [&>div:first-child]:px-0' : 'max-w-160', className)}
    />
  )
}
export function Heading({
  children,
  description,
  action,
  className,
}: {
  children: ReactNode
  description?: ReactNode
  action?: ReactNode
  className?: string
}) {
  return (
    <div className={cx('mb-10 px-4', className)}>
      <div className="flex flex-wrap items-center justify-between gap-4">
        <SettingsPageTitle>{children}</SettingsPageTitle>
        {action}
      </div>
      {description ? <p className="mt-2 text-[13px] leading-[22px] text-muted">{description}</p> : null}
    </div>
  )
}
export function Section({
  title,
  description,
  children,
  action,
  className,
}: {
  title: ReactNode
  description?: ReactNode
  children: ReactNode
  action?: ReactNode
  className?: string
}) {
  return (
    <section className={cx('grid min-w-0 gap-4', className)}>
      <div className="flex items-start justify-between gap-3 px-4">
        <SettingsSectionHeader className="min-w-0 flex-1">
          <SettingsSectionTitle>{title}</SettingsSectionTitle>
          {description ? <SettingsSectionDescription>{description}</SettingsSectionDescription> : null}
        </SettingsSectionHeader>
        {action}
      </div>
      {children}
    </section>
  )
}
export function Rows({ children, className }: { children: ReactNode; className?: string }) {
  return <SectionSurface className={cx('overflow-hidden p-0', className)}>{children}</SectionSurface>
}
export function Row({
  label,
  description,
  children,
  last = false,
}: {
  label: ReactNode
  description?: ReactNode
  children?: ReactNode
  last?: boolean
}) {
  return (
    <>
      <SettingsRow
        label={label}
        description={description}
        className="gap-4 bg-transparent p-4 max-sm:flex-col max-sm:items-stretch"
      >
        {children ? (
          <div
            className="max-w-full shrink-0 max-sm:flex max-sm:w-full max-sm:justify-end
max-sm:[&>input]:w-full max-sm:[&>[data-ui=dropdown]]:w-full"
          >
            {children}
          </div>
        ) : null}
      </SettingsRow>
      {!last ? <SettingsDivider className="mx-4 -mt-[0.5px]" /> : null}
    </>
  )
}
export function Empty({ title, children, action }: { title: string; children?: ReactNode; action?: ReactNode }) {
  return (
    <SectionSurface className="grid gap-3">
      <h3 className="text-[13px] leading-5 font-medium">{title}</h3>
      {children ? <p className="text-xs leading-[18px] text-muted">{children}</p> : null}
      {action ? <div>{action}</div> : null}
    </SectionSurface>
  )
}
export function Indicator({ color }: { color: string }) {
  return <span aria-hidden="true" className="inline-block size-1.5 shrink-0 rounded-full" style={{ backgroundColor: color }} />
}
