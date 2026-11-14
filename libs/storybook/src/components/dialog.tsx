import { animated, overlay } from './motion'
import {
  createContext,
  useCallback,
  useState,
  useContext,
  useEffect,
  useId,
  useRef,
  type ReactNode,
  type Ref,
  type ComponentPropsWithRef,
  type DialogHTMLAttributes,
  type ButtonHTMLAttributes,
} from 'react'
import type { NavigationRequest } from '../url'
import { createPortal } from 'react-dom'
import { Button, type ButtonProps } from './buttons'
import { XIcon } from '@phosphor-icons/react/dist/csr/X'
import { cx, composeRefs } from './shared'
import { Slot, useControllableState, type OpenStateProps, type CompositionProps } from './composition'
import { FloatingContainerContext } from '../context'

interface DialogScope {
  open: boolean
  setOpen(open: boolean): void
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
export interface DialogRootProps extends OpenStateProps {
  children: ReactNode
}

function Root({ children, open: controlledOpen, defaultOpen = false, onOpenChange }: DialogRootProps) {
  const [open, setOpen] = useControllableState(controlledOpen, defaultOpen, onOpenChange)
  const onClose = () => setOpen(false)
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
    <DialogContext
      value={{ open, setOpen, titleId, descriptionId, onClose: requestClose, completeClose: onClose, registerDiscard }}
    >
      {children}
    </DialogContext>
  )
}
function MountedContent({
  className,
  ref: forwardedRef,
  role = 'dialog',
  children,
  ...props
}: DialogHTMLAttributes<HTMLDialogElement> & { ref?: Ref<HTMLDialogElement> }) {
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
    <overlay.dialog
      {...props}
      ref={composeRefs(ref, forwardedRef)}
      data-state="open"
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
        `fixed inset-0 m-auto max-h-11/12 w-160 max-w-11/12 overflow-y-auto overscroll-none
      rounded-xl border border-border bg-inset p-0 text-foreground backdrop:bg-background/70`,
        className,
      )}
    >
      <FloatingContainerContext value={ref}>{children}</FloatingContainerContext>
    </overlay.dialog>
  )
}
function Content(props: DialogHTMLAttributes<HTMLDialogElement> & { ref?: Ref<HTMLDialogElement> }) {
  const { open } = useDialog()
  return open ? <MountedContent {...props} /> : null
}
function Trigger({ asChild = false, ...props }: ButtonProps) {
  const { open, setOpen } = useDialog()
  return (
    <Button
      {...props}
      asChild={asChild}
      aria-haspopup="dialog"
      aria-expanded={open}
      data-state={open ? 'open' : 'closed'}
      onClick={(event) => {
        props.onClick?.(event)
        if (!event.defaultPrevented && !props.disabled) setOpen(true)
      }}
    />
  )
}
function AlertContent(props: ComponentPropsWithRef<'dialog'>) {
  const { descriptionId } = useDialog()
  return <Content {...props} role="alertdialog" aria-describedby={descriptionId} className={cx('w-sm', props.className)} />
}
function Header({ className, ...props }: ComponentPropsWithRef<'div'>) {
  return (
    <div
      {...props}
      className={cx('flex min-h-12 items-center justify-between gap-4 border-b border-divider px-4', className)}
    />
  )
}
function Title({ className, ...props }: ComponentPropsWithRef<'h2'>) {
  const { titleId } = useDialog()
  return <h2 {...props} id={titleId} className={cx('text-sm font-medium', className)} />
}
function Description({ className, ...props }: ComponentPropsWithRef<'p'>) {
  const { descriptionId } = useDialog()
  return <p {...props} id={descriptionId} className={cx('text-sm text-muted-foreground', className)} />
}
function Body({ className, ...props }: ComponentPropsWithRef<'div'>) {
  return <div {...props} className={cx('grid gap-4 px-4 py-6', className)} />
}
function Footer({ className, ...props }: ComponentPropsWithRef<'div'>) {
  return <div {...props} className={cx('flex justify-end gap-2 border-t border-divider px-4 py-3.5', className)} />
}
function Close({
  children,
  className,
  asChild = false,
  ...props
}: ButtonHTMLAttributes<HTMLButtonElement> & CompositionProps & { ref?: Ref<HTMLButtonElement> }) {
  const { onClose } = useDialog()
  const Component = asChild ? Slot : animated.button
  return (
    <Component
      type="button"
      aria-label={asChild ? undefined : 'Close dialog'}
      {...props}
      className={
        asChild
          ? className
          : cx('cursor-pointer rounded text-muted-foreground hover:text-foreground focus-visible:outline-ring', className)
      }
      onClick={(event) => {
        props.onClick?.(event as React.MouseEvent<HTMLButtonElement>)
        if (!event.defaultPrevented && !props.disabled) onClose()
      }}
    >
      {children ?? <XIcon aria-hidden="true" className="size-4" />}
    </Component>
  )
}
function Cancel({ children = 'Cancel', ...props }: ButtonProps) {
  const { onClose } = useDialog()
  return (
    <Button
      {...props}
      onClick={(event) => {
        props.onClick?.(event as React.MouseEvent<HTMLButtonElement>)
        if (!event.defaultPrevented && !props.disabled) onClose()
      }}
    >
      {children}
    </Button>
  )
}
function Discard({
  children = 'Your unsaved changes will be lost.',
  request,
}: {
  children?: ReactNode
  request?: NavigationRequest
}) {
  const { completeClose, registerDiscard } = useDialog()
  const [open, setOpen] = useState(false)
  useEffect(() => registerDiscard(() => setOpen(true)), [registerDiscard])
  if (!open && !request) return null
  const keep = () => {
    setOpen(false)
    request?.keep()
  }
  return createPortal(
    <AlertDialog.Root
      open
      onOpenChange={(next) => {
        if (!next) keep()
      }}
    >
      <AlertDialog.Content>
        <AlertDialog.Header>
          <AlertDialog.Title>Discard changes?</AlertDialog.Title>
        </AlertDialog.Header>
        <AlertDialog.Body>
          <AlertDialog.Description>{children}</AlertDialog.Description>
        </AlertDialog.Body>
        <AlertDialog.Footer>
          <Button autoFocus className="h-8 px-3 text-xs" onClick={keep}>
            Keep editing
          </Button>
          <Button variant="destructive" className="h-8 px-3 text-xs" onClick={request ? request.discard : completeClose}>
            Discard changes
          </Button>
        </AlertDialog.Footer>
      </AlertDialog.Content>
    </AlertDialog.Root>,
    document.body,
  )
}
export const Dialog = { Root, Trigger, Content, Header, Title, Description, Body, Footer, Close, Cancel, Discard }
function AlertRoot(props: Parameters<typeof Root>[0]) {
  return <Root {...props} />
}
export const AlertDialog = {
  Root: AlertRoot,
  Trigger,
  Content: AlertContent,
  Header,
  Title,
  Description,
  Body,
  Footer,
  Close,
  Cancel,
}
