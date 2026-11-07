import type { HTMLAttributes } from 'react'
import { cx } from './shared'

export function SettingsPageTitle({ className, ...props }: HTMLAttributes<HTMLHeadingElement>) {
  return <h1 {...props} className={cx('m-0 text-2xl leading-8 font-medium text-text', className)} />
}

export function SettingsSectionHeader({ className, ...props }: HTMLAttributes<HTMLElement>) {
  return <header {...props} className={cx('grid min-w-0 gap-1', className)} />
}

export function SettingsSectionTitle({ className, ...props }: HTMLAttributes<HTMLHeadingElement>) {
  return <h2 {...props} className={cx('m-0 text-[15px] leading-[23px] font-medium text-text', className)} />
}

export function SettingsSectionDescription({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) {
  return <p {...props} className={cx('m-0 text-[13px] leading-[22px] text-muted', className)} />
}

export function SettingsDivider({ className, ...props }: HTMLAttributes<HTMLHRElement>) {
  return <hr {...props} className={cx('m-0 h-[0.5px] shrink-0 border-0 bg-divider', className)} />
}
