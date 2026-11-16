import { useEffect, useState } from 'react'
import { FormError, useFormValue } from '../../components/form'
import { FileField } from '../../components/fields'
import { motion } from 'motion/react'
import { useTransition } from '../../components/motion'
import { PlusIcon } from '@phosphor-icons/react/dist/csr/Plus'
import { ImageUploadInput } from '@compallm/contract/validation'
import { Avatar } from '../../components/content'
import { useSettingsController } from './controller'
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
      className={cx(
        'mx-auto min-w-0 py-16',
        wide ? 'w-full px-6 lg:px-14 [&>div:first-child]:px-0' : 'w-[calc(100%-2.75rem)] max-w-160',
        className,
      )}
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
      {description ? <p className="mt-2 text-xsm leading-5.5 text-muted-foreground">{description}</p> : null}
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
        className="gap-4 bg-transparent p-4 @max-[22rem]:flex-col @max-[22rem]:items-stretch"
      >
        {children ? (
          <div
            className="min-w-0 max-w-[65%] shrink @max-[22rem]:flex @max-[22rem]:w-full @max-[22rem]:max-w-full
@max-[22rem]:justify-end @max-[22rem]:[&>input]:w-full @max-[22rem]:*:data-[ui=dropdown]:w-full"
          >
            {children}
          </div>
        ) : null}
      </SettingsRow>
      {/* eslint-disable-next-line design/spacing -- Align the 0.5px divider with the preceding row. */}
      {!last ? <SettingsDivider className="mx-4 mt-[-0.5px]" /> : null}
    </>
  )
}
export function Empty({ title, children, action }: { title: string; children?: ReactNode; action?: ReactNode }) {
  return (
    <SectionSurface className="grid gap-3">
      <h3 className="text-xsm leading-5 font-medium">{title}</h3>
      {children ? <p className="text-xs leading-4.5 text-muted-foreground">{children}</p> : null}
      {action ? <div>{action}</div> : null}
    </SectionSurface>
  )
}
export function Indicator({ color }: { color: string }) {
  return <span aria-hidden="true" className="inline-block size-1.5 shrink-0 rounded-full" style={{ backgroundColor: color }} />
}

export function AvatarUploadPreview({
  name,
  saved,
  field,
}: {
  name: string
  saved: string | null
  field: 'profileUpload' | 'organizationUpload'
}) {
  const file = useFormValue<File | null>(useSettingsController().store.form, field)
  const [preview, setPreview] = useState<{ file: File; url: string } | null>(null)
  useEffect(() => {
    if (
      !file ||
      !ImageUploadInput.safeParse({
        size: file.size,
        mediaType: ({ 'image/jpeg': 'JPEG', 'image/png': 'PNG', 'image/webp': 'WEBP' } as Record<string, string>)[file.type],
      }).success
    )
      return
    const reader = new FileReader()
    reader.onload = () => setPreview({ file, url: String(reader.result) })
    reader.readAsDataURL(file)
    return () => {
      reader.onload = null
      reader.abort()
    }
  }, [file])
  return <Avatar name={name} src={file && preview?.file === file ? preview.url : (saved ?? undefined)} />
}

export function AvatarUploadControl({
  name,
  saved,
  field,
  label,
  disabled = false,
  disabledReason,
}: {
  name: string
  saved: string | null
  field: 'profileUpload' | 'organizationUpload'
  label: string
  disabled?: boolean
  disabledReason?: string
}) {
  const timing = useTransition('interaction')
  return (
    <motion.label
      initial="rest"
      whileHover={disabled ? 'rest' : 'active'}
      whileFocus={disabled ? 'rest' : 'active'}
      title={disabled ? disabledReason : undefined}
      className={cx(
        `group relative inline-flex size-10 rounded-full
focus-within:outline-2 focus-within:outline-ring`,
        disabled ? 'cursor-default' : 'cursor-pointer',
      )}
    >
      <AvatarUploadPreview name={name} saved={saved} field={field} />
      {!disabled ? (
        <motion.span
          variants={{ rest: { opacity: 0 }, active: { opacity: 1 } }}
          transition={timing}
          aria-hidden="true"
          className="pointer-events-none absolute inset-0 grid place-items-center rounded-full bg-overlay/50
text-on-overlay [@media(hover:none)]:opacity-100!"
        >
          <PlusIcon className="size-5" weight="bold" />
        </motion.span>
      ) : null}
      <FileField
        name={field}
        className="sr-only"
        accept="image/jpeg,image/png,image/webp"
        aria-label={label}
        disabled={disabled}
      />
      <FormError name={field} />
    </motion.label>
  )
}
