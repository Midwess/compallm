import { FormError } from './form'
import { animated } from './motion'
import { createContext, useContext, useId, type ReactNode, type ComponentPropsWithRef } from 'react'
import { TextField, type TextFieldProps } from './fields'
import { cx } from './shared'
const FieldContext = createContext<string | null>(null)
function useFieldId() {
  const id = useContext(FieldContext)
  if (!id) throw new Error('FormField parts require FormField.')
  return id
}
function Root({ children, className, ...props }: ComponentPropsWithRef<'div'> & { children: ReactNode }) {
  const id = useId()
  return (
    <FieldContext value={id}>
      <div {...props} className={cx('grid min-w-0 content-start gap-1', className)}>
        {children}
      </div>
    </FieldContext>
  )
}
function Label(props: ComponentPropsWithRef<'label'>) {
  const id = useFieldId()
  return <animated.label {...props} htmlFor={id} className={cx('text-sm font-medium', props.className)} />
}
function Input(props: TextFieldProps) {
  const id = useFieldId()
  return (
    <TextField
      {...props}
      id={id}
      className={cx('h-8 rounded-lg px-1.5 py-1 text-sm leading-5 focus:outline-input', props.className)}
    />
  )
}
function Hint(props: ComponentPropsWithRef<'p'>) {
  return <p {...props} className={cx('text-xs text-muted-foreground', props.className)} />
}
export const FormField = { Root, Label, Input, Hint, Error: FormError }
