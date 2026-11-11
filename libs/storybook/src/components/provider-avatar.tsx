import { animated } from './motion'
import { useRef } from 'react'
import { ProviderIcon } from './provider-icons'
import { Popup } from './popup'
import { cx } from './shared'

export function ProviderAvatar({
  provider,
  src,
  onFileChange,
  disabled = false,
  className,
}: {
  provider?: string | null
  src?: string | null
  onFileChange?: (file: File | null) => void
  disabled?: boolean
  className?: string
}) {
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
        disabled={disabled || !onFileChange}
        onChange={(event) => {
          const file = event.target.files?.[0]
          if (file) onFileChange?.(file)
          event.target.value = ''
        }}
      />
      <Popup>
        <Popup.Trigger
          aria-label="Change provider avatar"
          title="Change provider avatar"
          disabled={disabled || !onFileChange}
          className={cx(
            'size-8 shrink-0 overflow-hidden rounded-lg bg-elevated text-muted disabled:cursor-not-allowed disabled:opacity-50',
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
          <Popup.Item onClick={() => input.current?.click()}>Upload image</Popup.Item>
          <Popup.Item onClick={() => onFileChange?.(null)}>Use provider logo</Popup.Item>
          {src ? <Popup.Item onClick={() => onFileChange?.(null)}>Remove image</Popup.Item> : null}
        </Popup.Content>
      </Popup>
    </>
  )
}
