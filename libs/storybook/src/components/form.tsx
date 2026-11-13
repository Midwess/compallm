import { useContext, useEffect, useState, useSyncExternalStore, type ComponentProps, type ReactNode } from 'react'
import { FormContext } from '../context'
import { Button } from './buttons'
import type { FormModel, FormScope, FormOptions } from './form-model'
export { createForm, createFormStore, FormModel } from './form-model'
export type { CommitMode, FormOptions, FormRule, FormScope, FormStore } from './form-model'

const emptySubscribe = () => () => {}
export function useFormField<T = unknown>(name?: string) {
  const scope = useContext(FormContext)
  const form = name && scope?.has(name) ? scope : null
  const value = useSyncExternalStore(
    form?.subscribe ?? emptySubscribe,
    () => form?.get(name!) as T,
    () => form?.get(name!) as T,
  )
  const invalid = useSyncExternalStore(
    form?.subscribe ?? emptySubscribe,
    () => form?.invalid(name!) ?? false,
    () => form?.invalid(name!) ?? false,
  )
  return form
    ? {
        value,
        invalid,
        change: (next: T) => form.change(name!, next),
        blur: () => form.blur(name!),
        enter: () => form.enter(name!),
        escape: () => form.escape(name!),
      }
    : null
}
export function useFormValue<T>(form: FormScope, name: string): T {
  return useSyncExternalStore(
    form.subscribe,
    () => form.get(name) as T,
    () => form.get(name) as T,
  )
}
export function useManagedForm<T extends object>(initial: T, options: FormOptions<T>) {
  const [form] = useState(() => new Model(initial, options))
  useEffect(() => {
    form.configure(options)
  })
  return form
}
import { FormModel as Model } from './form-model'
function Root<T extends object>({
  model,
  children,
  onSubmit,
  onKeyDown,
  onReset,
  buttonOnly,
  ...props
}: Omit<ComponentProps<'form'>, 'onChange'> & { model: FormModel<T>; children?: ReactNode; buttonOnly?: boolean }) {
  useEffect(() => model.mount(), [model])
  return (
    <FormContext value={model}>
      <form
        {...props}
        noValidate
        onReset={(event) => {
          event.preventDefault()
          model.reset()
          onReset?.(event)
        }}
        onSubmit={(event) => {
          event.preventDefault()
          if (!onSubmit) void model.submit()
          onSubmit?.(event)
        }}
        onKeyDown={(event) => {
          onKeyDown?.(event)
          const target = event.target as HTMLElement
          if (
            !event.defaultPrevented &&
            !event.nativeEvent.isComposing &&
            event.key === 'Enter' &&
            (buttonOnly ?? model.buttonOnly()) &&
            target.tagName !== 'TEXTAREA' &&
            target.tagName !== 'BUTTON'
          )
            event.preventDefault()
        }}
      >
        {children}
      </form>
    </FormContext>
  )
}
function Submit(props: ComponentProps<typeof Button>) {
  return <Button {...props} type="submit" />
}
function Reset(props: ComponentProps<typeof Button>) {
  const scope = useContext(FormContext) as FormModel<object> | null
  return (
    <Button
      {...props}
      type="button"
      onClick={(event) => {
        scope?.reset()
        props.onClick?.(event)
      }}
    />
  )
}
function Status() {
  const scope = useContext(FormContext) as FormModel<object> | null
  useSyncExternalStore(
    scope?.subscribe ?? emptySubscribe,
    () => scope?.version ?? 0,
    () => scope?.version ?? 0,
  )
  return (
    <span role="status">
      {scope?.error ? 'Couldn’t save' : scope?.pending ? 'Saving…' : scope?.dirty ? 'Unsaved' : 'Saved'}
    </span>
  )
}
export const Form = Object.assign(Root, { Submit, Reset, Status })
