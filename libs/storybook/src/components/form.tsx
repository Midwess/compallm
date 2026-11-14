import { FieldErrorIndicator } from './feedback'
import {
  useContext,
  useEffect,
  useState,
  useSyncExternalStore,
  useCallback,
  useRef,
  useLayoutEffect,
  type ComponentProps,
  type ReactNode,
} from 'react'
import { useController } from 'react-hook-form'
import { useSyncExternalStoreWithSelector } from 'use-sync-external-store/shim/with-selector'
import { cloneFormValue as cloneDeep } from './form-model'
import isEqual from 'lodash-es/isEqual.js'
import { FormContext } from '../context'
import { Button } from './buttons'
import type { FormModel, FormScope, FormOptions, FormStore } from './form-model'
export { createForm, createFormStore, FormModel } from './form-model'
export type { CommitMode, FormOptions, FormRule, FormScope, FormStore } from './form-model'

const emptySubscribe = () => () => {}
export function useFormField<T = unknown>(name?: string) {
  const scope = useContext(FormContext)
  const form = name && scope?.has(name) ? scope : null
  const [standalone] = useState(() => new Model({ value: undefined }))
  const { field, fieldState } = useController({ name: form ? name! : 'value', control: (form ?? standalone).api.control })
  return form
    ? {
        value: field.value as T,
        invalid: fieldState.invalid,
        change: (next: T) => form.change(name!, next),
        blur: () => {
          field.onBlur()
          form.blur(name!)
        },
        enter: () => form.enter(name!),
        escape: () => form.escape(name!),
      }
    : null
}
export function useFormValue<T>(form: FormScope, name: string): T {
  return useSyncExternalStoreWithSelector(
    useCallback((notify) => form.subscribe(notify, name), [form, name]),
    () => form.editVersion,
    () => form.editVersion,
    () => cloneDeep(form.get(name)) as T,
    isEqual,
  )
}

export function useNativeFormField(name?: string) {
  const scope = useContext(FormContext)
  const form = name && scope?.has(name) ? scope : null
  return form
    ? {
        registration: form.native(name!),
        defaultValue: String(form.get(name!) ?? ''),
        enter: () => form.enter(name!),
        escape: () => form.escape(name!),
      }
    : null
}
export function useFormInvalid(name?: string) {
  const form = useContext(FormContext)
  return useSyncExternalStore(
    useCallback((notify) => (form && name ? form.subscribe(notify, name) : emptySubscribe()), [form, name]),
    () => (name ? (form?.invalid(name) ?? false) : false),
    () => (name ? (form?.invalid(name) ?? false) : false),
  )
}
export function FormError({ name, id }: { name: string; id?: string }) {
  const form = useContext(FormContext)
  const message = useSyncExternalStore(
    useCallback((notify) => (form ? form.subscribe(notify, name) : emptySubscribe()), [form, name]),
    () => form?.message(name),
    () => form?.message(name),
  )
  const validationVersion = useSyncExternalStore(
    useCallback((notify) => (form ? form.subscribe(notify, name) : emptySubscribe()), [form, name]),
    () => form?.validationVersion(name) ?? 0,
    () => form?.validationVersion(name) ?? 0,
  )
  const marker = useRef<HTMLSpanElement>(null)
  const [anchor, setAnchor] = useState<HTMLElement | null>(null)
  useLayoutEffect(() => {
    if (!message || !marker.current) return
    const root = marker.current.closest('form') ?? marker.current.parentElement
    const escaped = CSS.escape(name)
    const named = root?.querySelector<HTMLElement>(`[data-form-field="${escaped}"], [name="${escaped}"]`)
    const dropdown = named?.closest<HTMLElement>('[data-ui="dropdown"]')
    const target = dropdown?.querySelector<HTMLElement>('button') ?? named
    setAnchor(
      target?.matches('input[type=file]')
        ? (target.closest<HTMLElement>('label') ?? target)
        : (target ?? marker.current.parentElement),
    )
  }, [message, name])
  return (
    <>
      <span ref={marker} hidden />
      {message && anchor ? (
        <FieldErrorIndicator
          anchor={anchor}
          message={message}
          validationVersion={validationVersion}
          id={id ?? `${name}-error`}
        />
      ) : null}
    </>
  )
}
export function useFormStore<T extends object, V>(
  store: FormStore<T>,
  selector: (state: T) => V,
  names: readonly string[] = [],
): V {
  const key = names.join('|')
  const subscribe = useCallback(
    (notify: () => void) => {
      const stopWorkflow = store.subscribe(notify)
      const stops = key ? key.split('|').map((name) => store.form.subscribe(notify, name)) : []
      return () => {
        stopWorkflow()
        stops.forEach((stop) => stop())
      }
    },
    [store, key],
  )
  return useSyncExternalStoreWithSelector(
    subscribe,
    store.getState,
    store.getInitialState,
    (state) => cloneDeep(selector(state)),
    isEqual,
  )
}

export function contractResult<T>(
  result: { success: boolean; data?: unknown; error?: { issues: { path: PropertyKey[]; message: string }[] } },
  values: T,
  prefix = '',
) {
  return result.success
    ? { values }
    : {
        issues: result.error!.issues.map((issue) => ({
          ...issue,
          path: [...(prefix ? prefix.split('.') : []), ...issue.path],
        })),
      }
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
  ...props
}: Omit<ComponentProps<'form'>, 'onChange'> & { model: FormModel<T>; children?: ReactNode }) {
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
            model.buttonOnly() &&
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
export const Form = { Root, Submit, Reset, Status, Error: FormError }
