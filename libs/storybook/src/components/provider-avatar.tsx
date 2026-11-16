import { useFormField } from './form'
import { animated } from './motion'
import { useRef } from 'react'
import { ProviderIcon } from './provider-icons'
import { Popup } from './popup'
import { cx } from './shared'

export function ProviderAvatar({
  name,
  provider,
  src,
  onFileChange,
  disabled = false,
  className,
}: {
  name?: string
  provider?: string | null
  src?: string | null
  onFileChange?: (file: File | null) => void
  disabled?: boolean
  className?: string
}) {
  const field = useFormField<File | null>(name)
  const change = field ? field.change : onFileChange
  const input = useRef<HTMLInputElement>(null)
  return (
    <>
      <animated.input
        ref={input}
        type="file"
        accept="image/png,image/jpeg,image/webp,image/svg+xml"
        className="sr-only"
        tabIndex={-1}
        aria-label="Provider avatar image"
        disabled={disabled || !change}
        onChange={(event) => {
          const file = event.target.files?.[0]
          if (file) change?.(file)
          event.target.value = ''
        }}
      />
      <Popup.Root>
        <Popup.Trigger
          aria-label="Change provider avatar"
          title="Change provider avatar"
          disabled={disabled || !change}
          className={cx(
            `size-8 shrink-0 overflow-hidden rounded-lg bg-elevated text-muted-foreground
disabled:cursor-not-allowed disabled:opacity-50`,
            className,
          )}
        >
          {src ? (
            <img src={src} alt="Provider avatar" className="size-full object-cover" />
          ) : (
            <ProviderIcon provider={provider} className="size-5" />
          )}
        </Popup.Trigger>
        <Popup.Content aria-label="Change provider avatar">
          <Popup.Item onSelect={() => input.current?.click()}>Upload image</Popup.Item>
          <Popup.Item onSelect={() => change?.(null)}>Use provider logo</Popup.Item>
          {src ? <Popup.Item onSelect={() => change?.(null)}>Remove image</Popup.Item> : null}
        </Popup.Content>
      </Popup.Root>
    </>
  )
}
