import {
  createContext,
  useCallback,
  useState,
  useContext,
  useEffect,
  useId,
  useRef,
  type ReactNode,
  type HTMLAttributes,
  type DialogHTMLAttributes,
  type ButtonHTMLAttributes,
} from 'react'
import { createPortal } from 'react-dom'
import { Button, type ButtonProps } from './buttons'
import { XIcon } from '@phosphor-icons/react/dist/csr/X'
import { cx, popupMotion } from './shared'

interface DialogScope {
  titleId: string
  descriptionId: string
  onClose(): void
  completeClose(): void
  registerDiscard(request: () => void): () => void
}
const DialogContext = createContext<DialogScope | null>(null)
function useDialog() {
  const scope = useContext(DialogContext)
  if (!scope) throw new Error('Dialog parts require Dialog or AlertDialog.')
  return scope
}
function Root({ children, onClose }: { children: ReactNode; onClose(): void }) {
  const titleId = useId(),
    descriptionId = useId()
  const guard = useRef<(() => void) | null>(null)
  const registerDiscard = useCallback((request: () => void) => {
    guard.current = request
    return () => {
      if (guard.current === request) guard.current = null
    }
  }, [])
  const requestClose = () => {
    if (guard.current) guard.current()
    else onClose()
  }
  return (
    <DialogContext value={{ titleId, descriptionId, onClose: requestClose, completeClose: onClose, registerDiscard }}>
      {children}
    </DialogContext>
  )
}
function Content({ className, role = 'dialog', ...props }: DialogHTMLAttributes<HTMLDialogElement>) {
  const { titleId, onClose } = useDialog()
  const ref = useRef<HTMLDialogElement>(null)
  useEffect(() => {
    const previous = document.activeElement
    const element = ref.current
    element?.showModal()
    return () => {
      element?.close()
      if (previous instanceof HTMLElement && previous.isConnected) previous.focus()
    }
  }, [])
  return (
    <dialog
      {...props}
      ref={ref}
      role={role}
      aria-labelledby={titleId}
      onCancel={(event) => {
        props.onCancel?.(event)
        const handled = event.defaultPrevented
        event.preventDefault()
        if (!handled) onClose()
      }}
      aria-describedby={props['aria-describedby'] ?? undefined}
      className={cx(
        `${popupMotion} fixed inset-0 m-auto max-h-11/12 w-160 max-w-11/12 overflow-y-auto
      rounded-xl border border-border bg-inset p-0 text-text backdrop:bg-background/70`,
        className,
      )}
    />
  )
}
function AlertContent(props: DialogHTMLAttributes<HTMLDialogElement>) {
  const { descriptionId } = useDialog()
  return <Content {...props} role="alertdialog" aria-describedby={descriptionId} className={cx('w-96', props.className)} />
}
function Header({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return (
    <div
      {...props}
      className={cx('flex min-h-12 items-center justify-between gap-4 border-b border-divider px-4', className)}
    />
  )
}
function Title({ className, ...props }: HTMLAttributes<HTMLHeadingElement>) {
  const { titleId } = useDialog()
  return <h2 {...props} id={titleId} className={cx('text-sm font-medium', className)} />
}
function Description({ className, ...props }: HTMLAttributes<HTMLParagraphElement>) {
  const { descriptionId } = useDialog()
  return <p {...props} id={descriptionId} className={cx('text-sm text-muted', className)} />
}
function Body({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('grid gap-4 px-4 py-6', className)} />
}
function Footer({ className, ...props }: HTMLAttributes<HTMLDivElement>) {
  return <div {...props} className={cx('flex justify-end gap-2 border-t border-divider px-4 py-3.5', className)} />
}
function Close({ children, className, ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  const { onClose } = useDialog()
  return (
    <button
      type="button"
      aria-label="Close dialog"
      {...props}
      className={cx('cursor-pointer rounded text-muted hover:text-text focus-visible:outline-focus', className)}
      onClick={(event) => {
        props.onClick?.(event)
        if (!event.defaultPrevented) onClose()
      }}
    >
      {children ?? <XIcon aria-hidden="true" className="size-4" />}
    </button>
  )
}
function Cancel({ children = 'Cancel', ...props }: ButtonProps) {
  const { onClose } = useDialog()
  return (
    <Button
      {...props}
      onClick={(event) => {
        props.onClick?.(event)
        if (!event.defaultPrevented) onClose()
      }}
    >
      {children}
    </Button>
  )
}
function Discard({ children = 'Your unsaved changes will be lost.' }: { children?: ReactNode }) {
  const { completeClose, registerDiscard } = useDialog()
  const [open, setOpen] = useState(false)
  useEffect(() => registerDiscard(() => setOpen(true)), [registerDiscard])
  if (!open) return null
  return createPortal(
    <AlertDialog onClose={() => setOpen(false)}>
      <AlertDialog.Content>
        <AlertDialog.Header>
          <AlertDialog.Title>Discard changes?</AlertDialog.Title>
        </AlertDialog.Header>
        <AlertDialog.Body>
          <AlertDialog.Description>{children}</AlertDialog.Description>
        </AlertDialog.Body>
        <AlertDialog.Footer>
          <Button autoFocus className="h-8 px-3 text-xs" onClick={() => setOpen(false)}>
            Keep editing
          </Button>
          <Button variant="destructive" className="h-8 px-3 text-xs" onClick={completeClose}>
            Discard changes
          </Button>
        </AlertDialog.Footer>
      </AlertDialog.Content>
    </AlertDialog>,
    document.body,
  )
}
export const Dialog = Object.assign(Root, { Content, Header, Title, Description, Body, Footer, Close, Cancel, Discard })
function AlertRoot(props: Parameters<typeof Root>[0]) {
  return <Root {...props} />
}
export const AlertDialog = Object.assign(AlertRoot, { Content: AlertContent, Header, Title, Description, Body, Footer })
