import { useMemo, type HTMLAttributes } from 'react'
import DOMPurify from 'dompurify'
import { cx } from './shared'

export function RichMessage({
  message,
  navigate,
  className,
  ...props
}: Omit<HTMLAttributes<HTMLSpanElement>, 'children' | 'dangerouslySetInnerHTML'> & {
  message: string
  navigate?: (href: string) => void
}) {
  const html = useMemo(
    () =>
      DOMPurify.sanitize(message, {
        ALLOWED_TAGS: ['a', 'strong', 'em', 'b', 'i', 'br', 'code'],
        ALLOWED_ATTR: ['href'],
        ALLOW_DATA_ATTR: false,
        ALLOW_ARIA_ATTR: false,
        ALLOWED_URI_REGEXP: /^(?:https?:\/\/|\/(?!\/)|#)/i,
      }),
    [message],
  )
  return (
    <span
      {...props}
      className={cx(
        `wrap-anywhere [&_a]:text-ring [&_a]:underline [&_a]:underline-offset-2
    [&_a]:focus-visible:outline-2 [&_a]:focus-visible:outline-ring`,
        className,
      )}
      onClick={(event) => {
        props.onClick?.(event)
        if (
          !navigate ||
          event.defaultPrevented ||
          event.button !== 0 ||
          event.metaKey ||
          event.ctrlKey ||
          event.shiftKey ||
          event.altKey
        )
          return
        const anchor = event.target instanceof Element ? event.target.closest('a') : null
        const href = anchor?.getAttribute('href')
        if (!href || !event.currentTarget.contains(anchor)) return
        const target = new URL(href, window.location.href)
        if (target.origin !== window.location.origin) return
        event.preventDefault()
        navigate(target.pathname + target.search + target.hash)
      }}
      dangerouslySetInnerHTML={{ __html: html }}
    />
  )
}
