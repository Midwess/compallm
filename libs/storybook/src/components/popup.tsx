import { useFormField } from './form'
import { useContext, type ReactNode, type ComponentPropsWithRef } from 'react'
import { Menu } from '@base-ui/react/menu'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { animated, FloatingMotion } from './motion'
import { cx } from './shared'
import { Slot, type CompositionProps, type OpenStateProps } from './composition'
import { FloatingContainerContext } from '../context'

const itemClass = `flex h-8 w-full cursor-pointer items-center rounded-md px-2 text-xs outline-none
data-highlighted:bg-menu-hover focus-visible:outline-ring data-disabled:cursor-not-allowed data-disabled:opacity-50`
const surfaceClass = `max-h-(--available-height) w-48 max-w-[calc(100vw-1rem)] overflow-y-auto overscroll-none
rounded-control border border-border bg-menu p-1 font-sans text-foreground shadow-menu outline-none origin-(--transform-origin)`

export interface PopupRootProps extends OpenStateProps {
  children: ReactNode
}

function Root({ children, onOpenChange, ...props }: PopupRootProps) {
  return (
    <Menu.Root {...props} onOpenChange={(open) => onOpenChange?.(open)} modal={false}>
      {children}
    </Menu.Root>
  )
}

export interface PopupTriggerProps extends ComponentPropsWithRef<'button'>, CompositionProps {}

function Trigger({ className, asChild = false, children, ...props }: PopupTriggerProps) {
  return (
    <Menu.Trigger
      {...props}
      render={(partProps, state) =>
        asChild ? (
          <Slot {...partProps} data-state={state.open ? 'open' : 'closed'}>
            {children}
          </Slot>
        ) : (
          <animated.button type="button" {...partProps} data-state={state.open ? 'open' : 'closed'} />
        )
      }
      children={asChild ? undefined : children}
      className={
        asChild
          ? className
          : cx(
              `flex size-7 cursor-pointer items-center justify-center rounded-xl text-muted-foreground hover:bg-hover
focus-visible:outline-ring`,
              className,
            )
      }
    />
  )
}

function IconTrigger(props: PopupTriggerProps) {
  return (
    <Trigger
      {...props}
      className={cx(
        'size-7 rounded-full border border-border/60 bg-inset text-muted-foreground hover:border-input hover:text-foreground',
        props.className,
      )}
    />
  )
}

function Content({
  className,
  align = 'end',
  sideOffset = 4,
  ...props
}: ComponentPropsWithRef<'div'> & { align?: 'start' | 'center' | 'end'; sideOffset?: number }) {
  const container = useContext(FloatingContainerContext)
  return (
    <Menu.Portal container={container}>
      <Menu.Positioner
        sideOffset={sideOffset}
        align={align}
        positionMethod="fixed"
        collisionPadding={8}
        className="z-50 outline-none"
      >
        <Menu.Popup {...props} render={<FloatingMotion />} className={cx(surfaceClass, className)} />
      </Menu.Positioner>
    </Menu.Portal>
  )
}

export interface PopupItemProps extends Omit<ComponentPropsWithRef<'button'>, 'onSelect'>, CompositionProps {
  disabled?: boolean
  onSelect?(event: React.MouseEvent<HTMLButtonElement>): void
}
function Item({ className, asChild = false, children, onSelect, onClick, ...props }: PopupItemProps) {
  const itemProps = props as unknown as Menu.Item.Props
  return (
    <Menu.Item
      {...itemProps}
      nativeButton={!asChild}
      render={asChild ? <Slot>{children}</Slot> : <animated.button type="button" />}
      onClick={(event) => {
        onClick?.(event as unknown as React.MouseEvent<HTMLButtonElement>)
        if (!event.defaultPrevented) onSelect?.(event as unknown as React.MouseEvent<HTMLButtonElement>)
      }}
      className={asChild ? className : cx(itemClass, className)}
    >
      {asChild ? undefined : children}
    </Menu.Item>
  )
}

function Separator() {
  // eslint-disable-next-line design/spacing -- The separator is a 1px hairline.
  return <Menu.Separator className="my-1 h-px bg-border/60" />
}

export interface PopupCheckboxItemProps extends Omit<ComponentPropsWithRef<'div'>, 'defaultChecked'> {
  checked?: boolean
  defaultChecked?: boolean
  disabled?: boolean
  onCheckedChange?(checked: boolean): void
  name?: string
  selectionValue?: string
}

function CheckboxItem({ className, children, name, selectionValue, ...props }: PopupCheckboxItemProps) {
  const field = useFormField<boolean | string[]>(name)
  const checked = field
    ? Array.isArray(field.value)
      ? field.value.includes(selectionValue ?? '')
      : Boolean(field.value)
    : props.checked

  return (
    <Menu.CheckboxItem
      {...props}
      render={(partProps, state) => <div {...partProps} data-state={state.checked ? 'checked' : 'unchecked'} />}
      checked={checked}
      onCheckedChange={(next) => {
        if (field)
          field.change(
            Array.isArray(field.value)
              ? next
                ? [...field.value, selectionValue ?? '']
                : field.value.filter((id) => id !== selectionValue)
              : next,
          )
        else props.onCheckedChange?.(next)
      }}
      className={cx(itemClass, 'min-h-9 gap-3 text-xsm', className)}
    >
      <span aria-hidden="true" className="flex size-4 shrink-0 items-center justify-center rounded-sm border border-input">
        <Menu.CheckboxItemIndicator>
          <CheckIcon className="size-3" />
        </Menu.CheckboxItemIndicator>
      </span>
      {children}
    </Menu.CheckboxItem>
  )
}

function RadioItem({ className, children, ...props }: ComponentPropsWithRef<'div'> & { value: string; disabled?: boolean }) {
  return (
    <Menu.RadioItem
      {...props}
      render={(partProps, state) => <div {...partProps} data-state={state.checked ? 'checked' : 'unchecked'} />}
      closeOnClick
      className={cx(itemClass, 'justify-between gap-4', className)}
    >
      {children}
      <Menu.RadioItemIndicator>
        <CheckIcon aria-hidden="true" className="size-3" />
      </Menu.RadioItemIndicator>
    </Menu.RadioItem>
  )
}

function Sub({ children, onOpenChange, ...props }: PopupRootProps) {
  return (
    <Menu.SubmenuRoot {...props} onOpenChange={(open) => onOpenChange?.(open)}>
      {children}
    </Menu.SubmenuRoot>
  )
}
function SubTrigger({ children, className, asChild = false, ...props }: ComponentPropsWithRef<'div'> & CompositionProps) {
  return (
    <Menu.SubmenuTrigger
      {...props}
      render={asChild ? <Slot>{children}</Slot> : undefined}
      className={asChild ? className : cx(itemClass, 'justify-between', className)}
    >
      {asChild ? undefined : (
        <>
          {children}
          <CaretRightIcon className="size-3" aria-hidden="true" />
        </>
      )}
    </Menu.SubmenuTrigger>
  )
}
function SubContent(props: ComponentPropsWithRef<'div'>) {
  const container = useContext(FloatingContainerContext)
  return (
    <Menu.Portal container={container}>
      <Menu.Positioner
        sideOffset={4}
        alignOffset={-4}
        positionMethod="fixed"
        collisionPadding={8}
        className="z-50 outline-none"
      >
        <Menu.Popup {...props} render={<FloatingMotion />} className={cx(surfaceClass, props.className)} />
      </Menu.Positioner>
    </Menu.Portal>
  )
}

function RadioGroup({
  onValueChange,
  ...props
}: ComponentPropsWithRef<'div'> & { value?: string; defaultValue?: string; onValueChange?(value: string): void }) {
  return <Menu.RadioGroup {...props} onValueChange={(value) => onValueChange?.(String(value))} />
}

export const Popup = {
  Root,
  Trigger,
  IconTrigger,
  Content,
  Item,
  CheckboxItem,
  RadioGroup,
  RadioItem,
  Separator,
  Sub,
  SubTrigger,
  SubContent,
}
