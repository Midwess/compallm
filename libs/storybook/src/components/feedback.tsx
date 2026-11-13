import { animated, LoadingMotion, overlay, FieldErrorMotion } from './motion'
import {
  useId,
  useEffect,
  useState,
  useRef,
  useLayoutEffect,
  type HTMLAttributes,
  type ReactElement,
  type ReactNode,
} from 'react'
import { createPortal } from 'react-dom'
import { Tooltip as BaseTooltip } from '@base-ui/react/tooltip'
import { cx } from './shared'
import { RichMessage } from './rich-message'
import { Button } from './buttons'
import { QuestionIcon } from '@phosphor-icons/react/dist/csr/Question'
import { InfoIcon } from '@phosphor-icons/react/dist/csr/Info'
import { WarningCircleIcon } from '@phosphor-icons/react/dist/csr/WarningCircle'
import { WarningIcon } from '@phosphor-icons/react/dist/csr/Warning'
import { XIcon } from '@phosphor-icons/react/dist/csr/X'
import { CheckCircleIcon } from '@phosphor-icons/react/dist/csr/CheckCircle'
import { SpinnerGapIcon } from '@phosphor-icons/react/dist/csr/SpinnerGap'

export interface ToastProps extends Omit<HTMLAttributes<HTMLDivElement>, 'children'> {
  message: string
  tone?: 'info' | 'error' | 'warning' | 'success' | 'loading'
  action?: { label: string; onClick: () => void }
  onDismiss: () => void
  dismissLabel?: string
}

const toastTones = {
  info: { Icon: InfoIcon, color: 'text-primary', label: 'Info' },
  error: { Icon: WarningCircleIcon, color: 'text-error', label: 'Error' },
  warning: { Icon: WarningIcon, color: 'text-warning', label: 'Warning' },
  success: { Icon: CheckCircleIcon, color: 'text-success', label: 'Success' },
  loading: { Icon: SpinnerGapIcon, color: 'text-primary', label: 'Loading' },
}

export function Toast({
  message,
  title,
  action,
  tone = 'info',
  onDismiss,
  dismissLabel = 'Dismiss notification',
  className,
  ...props
}: ToastProps) {
  const { Icon, color, label } = toastTones[tone]
  const icon = <Icon size={20} weight="regular" aria-hidden="true" />
  return (
    <div
      {...props}
      data-ui="toast"
      data-tone={tone}
      className={cx(
        'flex w-100 max-w-full items-start gap-2.5 rounded-xl',
        'border border-border bg-elevated p-2 font-sans text-xsm leading-5 text-text shadow-lg',
        className,
      )}
    >
      <span className={cx('mt-0.5 inline-flex shrink-0', color)}>
        {tone === 'loading' ? <LoadingMotion>{icon}</LoadingMotion> : icon}
      </span>
      <div className="min-w-0 flex-1">
        <div className="wrap-anywhere" role={tone === 'error' ? 'alert' : 'status'} aria-atomic="true">
          <span className="sr-only">{label}: </span>
          {title ? <p className="text-sm leading-5.5 font-semibold">{title}</p> : null}
          <div className={cx(title ? 'mt-0.5 text-muted' : 'text-text')}>
            <RichMessage message={message} />
          </div>
        </div>
        <div className="mt-1 flex flex-wrap items-center gap-x-3 gap-y-1">
          <animated.button
            type="button"
            onClick={onDismiss}
            className={cx(
              'min-h-6 cursor-pointer rounded text-muted hover:text-text',
              'focus-visible:outline-2 focus-visible:outline-focus',
            )}
          >
            Dismiss
          </animated.button>
          {action ? (
            <animated.button
              type="button"
              onClick={action.onClick}
              className={cx(
                'min-h-6 cursor-pointer rounded text-primary hover:text-focus',
                'focus-visible:outline-2 focus-visible:outline-focus',
              )}
            >
              {action.label}
            </animated.button>
          ) : null}
        </div>
      </div>
      <animated.button
        type="button"
        onClick={onDismiss}
        aria-label={dismissLabel}
        className={cx(
          'inline-flex size-6 shrink-0 cursor-pointer items-center justify-center rounded text-muted',
          'hover:text-text focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-focus',
        )}
      >
        <XIcon size={16} weight="regular" aria-hidden="true" />
      </animated.button>
    </div>
  )
}

export function HelperText({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) {
  return <p {...props} className={cx('text-[12px] leading-4.5 text-muted', className)} />
}
export function InlineError({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) {
  return <p role="alert" {...props} className={cx('text-xsm leading-5 text-error', className)} />
}
export function CodeError(props: HTMLAttributes<HTMLParagraphElement>) {
  return <InlineError {...props}>{props.children ?? 'Incorrect code. Try again.'}</InlineError>
}
export function EmailRegistrationError(props: HTMLAttributes<HTMLParagraphElement>) {
  return <InlineError {...props}>{props.children ?? 'Email already registered. Sign in instead.'}</InlineError>
}
export function KeyboardKey({ children }: { children: ReactNode }) {
  return (
    <kbd
      className={`
  inline-flex h-7 min-w-7 items-center justify-center rounded-md bg-elevated p-0.5
  font-sans text-[14px]
`}
    >
      {children}
    </kbd>
  )
}
function TooltipPopup({ content }: { content: ReactNode }) {
  return (
    <BaseTooltip.Portal>
      <BaseTooltip.Positioner sideOffset={6} className="z-50">
        <BaseTooltip.Popup
          className="max-w-xs rounded-full border border-border bg-elevated px-3 py-1.5 text-center text-xs leading-4
text-text shadow-lg"
        >
          <BaseTooltip.Arrow
            className="absolute flex data-[side=top]:-bottom-1 data-[side=bottom]:-top-1
 data-[side=left]:-right-1 data-[side=right]:-left-1"
          >
            <span className="block size-2 rotate-45 bg-elevated" />
          </BaseTooltip.Arrow>
          {content}
        </BaseTooltip.Popup>
      </BaseTooltip.Positioner>
    </BaseTooltip.Portal>
  )
}

export function Tooltip({ children, content }: { children: ReactElement; content: ReactNode }) {
  return (
    <BaseTooltip.Provider>
      <BaseTooltip.Root>
        <BaseTooltip.Trigger render={children} />
        <TooltipPopup content={content} />
      </BaseTooltip.Root>
    </BaseTooltip.Provider>
  )
}

export function DisabledTooltip({
  disabled,
  reason,
  allowInteraction = false,
  children,
}: {
  disabled: boolean
  reason: string
  allowInteraction?: boolean
  children: ReactNode
}) {
  if (!disabled) return children
  return (
    <BaseTooltip.Provider>
      <BaseTooltip.Root>
        <BaseTooltip.Trigger
          render={<span tabIndex={0} aria-label={reason} />}
          className={allowInteraction ? 'inline-flex w-full max-w-full' : 'inline-flex max-w-full **:pointer-events-none'}
        >
          {children}
        </BaseTooltip.Trigger>
        <TooltipPopup content={reason} />
      </BaseTooltip.Root>
    </BaseTooltip.Provider>
  )
}
export function FieldHelp({ label, children }: { label: string; children: ReactNode }) {
  return (
    <Tooltip content={children}>
      <animated.button
        type="button"
        aria-label={label}
        className={`
inline-flex size-5 shrink-0 cursor-help items-center justify-center rounded text-muted hover:text-text
focus-visible:text-text focus-visible:outline-1 focus-visible:outline-muted
`}
      >
        <QuestionIcon className="size-4" aria-hidden="true" />
      </animated.button>
    </Tooltip>
  )
}
export function EmptyState({ title, children, action }: { title: ReactNode; children?: ReactNode; action?: ReactNode }) {
  return (
    <div
      className={`
  flex min-h-32 flex-col items-center gap-1 bg-surface p-3 text-center [&>h3]:text-[16px]
  [&>h3]:font-medium
`}
    >
      <h3>{title}</h3>
      <div className="text-xsm leading-5 text-muted">{children}</div>
      {action}
    </div>
  )
}
export function Callout({
  title,
  children,
  action,
  tone = 'neutral',
}: {
  title: ReactNode
  children?: ReactNode
  action?: ReactNode
  tone?: 'neutral' | 'success' | 'warning' | 'error'
}) {
  return (
    <aside
      className={`
  flex flex-col gap-1 rounded-[10px] bg-elevated p-2 data-[tone=success]:text-success
  data-[tone=warning]:text-warning data-[tone=error]:text-error [&>strong]:text-[15px]
  [&>strong]:font-medium
`}
      data-tone={tone}
    >
      <strong>{title}</strong>
      <div className="text-[12px] leading-4.5 text-muted">{children}</div>
      {action}
    </aside>
  )
}
export function Popover({ label, children }: { label: string; children: ReactNode }) {
  const id = useId()
  return (
    <span className="inline">
      <Button popoverTarget={id}>{label}</Button>
      <overlay.div
        id={id}
        popover="auto"
        className={`
  w-12 max-w-[calc(100vw-32px)] rounded-xl border-[0.5px] border-input-border bg-menu p-2
  text-text [&:popover-open]:absolute [&:popover-open]:m-1 [&:popover-open]:[position-area:bottom_span-right]
  [&:popover-open]:[position-try-fallbacks:flip-block,flip-inline] [&>*+*]:mt-1.5
`}
      >
        {children}
      </overlay.div>
    </span>
  )
}
export function DropInsertionIndicator({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div aria-hidden="true" {...props} className={cx('h-0.5 w-full rounded-[1px] bg-focus', className)} />
}

export function FeedbackSlot({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('h-fit py-1 overflow-y-auto', className)} />
}

export function FieldErrorIndicator({
  anchor,
  message,
  id,
  validationVersion = 0,
}: {
  anchor: HTMLElement
  message: string
  id: string
  validationVersion?: number
}) {
  const triggerId = useId()
  const badge = useRef<HTMLButtonElement>(null)
  const [expiredVersion, setExpiredVersion] = useState<number>()
  const [interacting, setInteracting] = useState(false)
  const [dismissedVersion, setDismissedVersion] = useState<number>()
  const [visible, setVisible] = useState(true)
  const container = anchor.closest('dialog') ?? anchor.ownerDocument.body
  useEffect(() => {
    const timer = setTimeout(() => setExpiredVersion(validationVersion), 5000)
    return () => clearTimeout(timer)
  }, [validationVersion])
  useLayoutEffect(() => attachFieldError(anchor, id, () => badge.current, setVisible), [anchor, id])

  return createPortal(
    <>
      <span id={id} className="sr-only" role="alert">
        {message}
      </span>
      <BaseTooltip.Provider>
        <BaseTooltip.Root
          open={visible && dismissedVersion !== validationVersion && (expiredVersion !== validationVersion || interacting)}
          triggerId={triggerId}
          onOpenChange={(open, details) => {
            if (details.reason === 'escape-key' || details.reason === 'trigger-press') {
              setExpiredVersion(validationVersion)
              setInteracting(false)
              setDismissedVersion(validationVersion)
            } else {
              setInteracting(open)
              if (open) setDismissedVersion(undefined)
            }
          }}
        >
          <BaseTooltip.Trigger
            id={triggerId}
            ref={badge}
            delay={0}
            closeDelay={100}
            closeOnClick={false}
            aria-label="Show validation error"
            aria-describedby={id}
            className="fixed z-50 inline-flex size-5 cursor-help items-center justify-center rounded-full
bg-error text-on-primary focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-error"
            onPointerEnter={() => setDismissedVersion(undefined)}
            onFocus={() => setDismissedVersion(undefined)}
          >
            <WarningIcon weight="fill" aria-hidden="true" className="size-3" />
          </BaseTooltip.Trigger>
          <BaseTooltip.Portal container={container}>
            <BaseTooltip.Positioner side="right" align="center" sideOffset={-20} collisionPadding={8} className="z-40">
              <BaseTooltip.Popup
                render={<FieldErrorMotion />}
                className="min-h-5 max-w-xs origin-left rounded-full bg-error py-1 pr-1 pl-6
text-xs leading-3 text-on-primary shadow-lg data-[side=left]:origin-right data-[side=left]:pr-6 data-[side=left]:pl-1"
              >
                {message}
              </BaseTooltip.Popup>
            </BaseTooltip.Positioner>
          </BaseTooltip.Portal>
        </BaseTooltip.Root>
      </BaseTooltip.Provider>
    </>,
    container,
  )
}

function attachFieldError(
  anchor: HTMLElement,
  id: string,
  getBadge: () => HTMLButtonElement | null,
  setVisible: (visible: boolean) => void,
) {
  const originalPadding = anchor.style.paddingRight
  const originalBorder = anchor.style.borderColor
  const originalDescription = anchor.getAttribute('aria-describedby')
  anchor.setAttribute('aria-invalid', 'true')
  anchor.setAttribute('aria-describedby', [...new Set([...(originalDescription?.split(' ') ?? []), id])].join(' '))
  anchor.style.borderColor = 'var(--color-error)'
  const textControl = anchor.matches('input:not([type=file]), textarea')
  const padding = parseFloat(getComputedStyle(anchor).paddingRight)
  if (textControl) anchor.style.paddingRight = `${Math.max(padding, 32)}px`
  const update = () => {
    const node = getBadge()
    if (!node) return
    const rect = anchor.getBoundingClientRect()
    node.style.top = `${rect.top + Math.min(6, Math.max(0, (rect.height - 20) / 2))}px`
    node.style.left = `${rect.right - 26}px`
    node.style.visibility = rect.width && rect.height && rect.bottom > 0 && rect.top < window.innerHeight ? 'visible' : 'hidden'
  }
  update()
  const observer = new ResizeObserver(update)
  observer.observe(anchor)
  if (anchor.parentElement) observer.observe(anchor.parentElement)
  const intersection = new IntersectionObserver(([entry]) => {
    setVisible(entry.isIntersecting)
    const node = getBadge()
    if (node) node.style.visibility = entry.isIntersecting ? 'visible' : 'hidden'
  })
  intersection.observe(anchor)
  window.addEventListener('resize', update)
  window.addEventListener('scroll', update, true)
  return () => {
    observer.disconnect()
    intersection.disconnect()
    window.removeEventListener('resize', update)
    window.removeEventListener('scroll', update, true)
    anchor.style.paddingRight = originalPadding
    anchor.style.borderColor = originalBorder
    anchor.removeAttribute('aria-invalid')
    if (originalDescription === null) anchor.removeAttribute('aria-describedby')
    else anchor.setAttribute('aria-describedby', originalDescription)
  }
}
