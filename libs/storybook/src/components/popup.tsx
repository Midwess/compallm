import { useFormField } from './form'
import { useContext, type ButtonHTMLAttributes, type HTMLAttributes, type ReactNode } from 'react'
import { Menu } from '@base-ui/react/menu'
import { CaretRightIcon } from '@phosphor-icons/react/dist/csr/CaretRight'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { animated, FloatingMotion } from './motion'
import { cx } from './shared'
import { FloatingContainerContext } from '../context'

const itemClass = `flex h-8 w-full cursor-pointer items-center rounded-md px-2 text-xs outline-none
data-highlighted:bg-menu-hover focus-visible:outline-ring data-disabled:cursor-not-allowed data-disabled:opacity-50`
const surfaceClass = `max-h-(--available-height) w-48 max-w-[calc(100vw-1rem)] overflow-y-auto overscroll-contain
rounded-control border border-border bg-menu p-1 font-sans text-foreground shadow-menu outline-none origin-(--transform-origin)`

function Root({ children }: { children: ReactNode }) {
  return <Menu.Root modal={false}>{children}</Menu.Root>
}

function Trigger({ className, ...props }: ButtonHTMLAttributes<HTMLButtonElement>) {
  return (
    <Menu.Trigger
      {...props}
      render={<animated.button type="button" />}
      className={cx(
        `flex size-7 cursor-pointer items-center justify-center rounded-xl text-muted-foreground hover:bg-hover
focus-visible:outline-ring`,
        className,
      )}
    />
  )
}

function IconTrigger(props: ButtonHTMLAttributes<HTMLButtonElement>) {
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
}: HTMLAttributes<HTMLDivElement> & Pick<Menu.Positioner.Props, 'align' | 'sideOffset'>) {
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

function Item({ className, ...props }: Omit<Menu.Item.Props, 'className'> & { className?: string }) {
  return <Menu.Item {...props} nativeButton render={<animated.button type="button" />} className={cx(itemClass, className)} />
}

function Separator() {
  // eslint-disable-next-line design/spacing -- The separator is a 1px hairline.
  return <Menu.Separator className="my-1 h-px bg-border/60" />
}

function CheckboxItem({
  className,
  children,
  name,
  selectionValue,
  ...props
}: Omit<Menu.CheckboxItem.Props, 'className'> & { className?: string; name?: string; selectionValue?: string }) {
  const field = useFormField<boolean | string[]>(name)
  const checked = field
    ? Array.isArray(field.value)
      ? field.value.includes(selectionValue ?? '')
      : Boolean(field.value)
    : props.checked

  return (
    <Menu.CheckboxItem
      {...props}
      checked={checked}
      onCheckedChange={(next, details) => {
        if (field)
          field.change(
            Array.isArray(field.value)
              ? next
                ? [...field.value, selectionValue ?? '']
                : field.value.filter((id) => id !== selectionValue)
              : next,
          )
        else props.onCheckedChange?.(next, details)
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

function RadioItem({ className, children, ...props }: Omit<Menu.RadioItem.Props, 'className'> & { className?: string }) {
  return (
    <Menu.RadioItem {...props} closeOnClick className={cx(itemClass, 'justify-between gap-4', className)}>
      {children}
      <Menu.RadioItemIndicator>
        <CheckIcon aria-hidden="true" className="size-3" />
      </Menu.RadioItemIndicator>
    </Menu.RadioItem>
  )
}

function Submenu({ label, children }: { label: ReactNode; children: ReactNode }) {
  const container = useContext(FloatingContainerContext)
  return (
    <Menu.SubmenuRoot>
      <Menu.SubmenuTrigger className={cx(itemClass, 'justify-between')}>
        {label}
        <CaretRightIcon className="size-3" aria-hidden="true" />
      </Menu.SubmenuTrigger>
      <Menu.Portal container={container}>
        <Menu.Positioner
          sideOffset={4}
          alignOffset={-4}
          positionMethod="fixed"
          collisionPadding={8}
          className="z-50 outline-none"
        >
          <Menu.Popup
            aria-label={typeof label === 'string' ? label : undefined}
            render={<FloatingMotion />}
            className={surfaceClass}
          >
            {children}
          </Menu.Popup>
        </Menu.Positioner>
      </Menu.Portal>
    </Menu.SubmenuRoot>
  )
}

export const Popup = Object.assign(Root, {
  Trigger,
  IconTrigger,
  Content,
  Item,
  CheckboxItem,
  RadioGroup: Menu.RadioGroup,
  RadioItem,
  Separator,
  Submenu,
})
