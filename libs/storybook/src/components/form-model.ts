import { FormApi, getBy, setBy } from '@tanstack/react-form'
import type { AnyFormApi } from '@tanstack/react-form'
import { createStore, type StoreApi } from 'zustand/vanilla'

export type CommitMode = 'change' | 'blur' | 'debounce' | 'complete' | 'button'
export interface FormRule {
  mode?: CommitMode
  group?: string
  debounceMs?: number
  accepts?(value: unknown): boolean
  complete?(value: unknown): boolean
}
export interface FormOptions<T extends object> {
  mode?: CommitMode
  command?: boolean
  names?: string[]
  fields?: Record<string, FormRule>
  validate?(values: T, names: string[]): boolean | Promise<boolean>
  submit?(
    values: T,
    patch: Partial<T>,
    signal: AbortSignal,
    names: string[],
  ): Promise<Partial<T> | false | void> | Partial<T> | false | void
  changed?(name: string, value: unknown): void
  error?(error: unknown, retry: () => void): void
}
type Pending = { value: unknown; version: number }
export interface FormScope {
  has(name: string): boolean
  get(name: string): unknown
  change(name: string, value: unknown): void
  blur(name: string): void
  enter(name: string): void
  escape(name: string): void
  subscribe(listener: () => void): () => void
  invalid(name: string): boolean
  buttonOnly(): boolean
}

export class FormModel<T extends object> implements FormScope {
  readonly api: AnyFormApi
  private initial: T
  private confirmed: T
  private options: FormOptions<T>
  private listeners = new Set<() => void>()
  private versions = new Map<string, number>()
  private queued = new Map<string, Pending>()
  private active = new Map<string, Pending>()
  private timers = new Map<string, ReturnType<typeof setTimeout>>()
  private invalidFields = new Set<string>()
  private request?: AbortController
  private generation = 0
  private paused = false
  private revision = 0
  private failedNames: string[] = []
  private failure: unknown = null
  private shared = false
  private mounted = 0
  private unmountApi?: () => void
  private unsubscribeApi?: { unsubscribe(): void }

  constructor(values: T, options: FormOptions<T> = {}, api?: AnyFormApi, versions?: Map<string, number>) {
    if (versions) this.versions = versions
    this.initial = values
    this.confirmed = values
    this.options = options
    this.shared = Boolean(api)
    this.api = api ?? new FormApi({ defaultValues: values })
    if (!this.shared) this.unsubscribeApi = this.api.store.subscribe(() => this.emit())
  }
  scope = (names: string[], options: FormOptions<T> = {}) =>
    new FormModel(
      this.values,
      {
        ...this.options,
        ...options,
        names,
        changed: (name, value) => {
          this.options.changed?.(name, value)
          options.changed?.(name, value)
        },
      },
      this.api,
      this.versions,
    )
  configure = (options: FormOptions<T>) => {
    if ((options.mode ?? 'button') === 'button' && !this.buttonOnly()) {
      this.timers.forEach(clearTimeout)
      this.timers.clear()
      this.queued.clear()
    }
    this.options = options
  }
  get values(): T {
    return this.api.state.values as T
  }
  get baseline(): T {
    return this.confirmed
  }
  get error(): unknown {
    return this.failure
  }
  get pending(): boolean {
    return this.request !== undefined
  }
  get dirty(): boolean {
    return (this.options.names ?? Object.keys(this.values)).some((key) => !Object.is(this.get(key), getBy(this.confirmed, key)))
  }
  get version(): number {
    return this.revision
  }
  submitted = (name: string) => this.active.get(name)?.value
  waiting = (name: string) => this.queued.has(name)
  invalid = (name: string) => this.invalidFields.has(name)
  has = (name: string) =>
    (!this.options.names ||
      this.options.names.some((root) => name === root || name.startsWith(root + '.') || name.startsWith(root + '['))) &&
    Object.hasOwn(this.values, name.split(/[.[\]]/)[0])
  get = (name: string): unknown => getBy(this.values, name)
  buttonOnly = () => (this.options.mode ?? 'button') === 'button'
  subscribe = (listener: () => void) => {
    this.listeners.add(listener)
    return () => {
      this.listeners.delete(listener)
    }
  }
  private emit() {
    this.revision++
    this.listeners.forEach((listener) => listener())
  }
  private rule(name: string): FormRule {
    return { mode: this.options.mode ?? 'button', ...this.options.fields?.[name] }
  }
  private clear(name: string) {
    clearTimeout(this.timers.get(name))
    this.timers.delete(name)
  }
  set = (name: string, value: unknown) => {
    if (Object.is(this.get(name), value)) return
    this.versions.set(name, (this.versions.get(name) ?? 0) + 1)
    for (const key of this.versions.keys()) {
      if (
        (key.startsWith(name + '.') && !Object.is(this.get(key), getBy(value, key.slice(name.length + 1)))) ||
        (key !== name && (name.startsWith(key + '.') || name.startsWith(key + '[')))
      )
        this.versions.set(key, (this.versions.get(key) ?? 0) + 1)
    }
    this.api.setFieldValue(name, value, { dontRunListeners: true })
  }
  change = (name: string, value: unknown) => {
    const rule = this.rule(name)
    if (rule.accepts && !rule.accepts(value)) return
    const different = !Object.is(this.get(name), value)
    this.set(name, value)
    if (different && rule.mode === 'complete') this.paused = false
    this.invalidFields.delete(name)
    this.options.changed?.(name, value)
    this.clear(name)
    if (this.buttonOnly() || this.paused) return
    if (rule.mode === 'change' || (rule.mode === 'complete' && rule.complete?.(value))) void this.commit([name])
    if (rule.mode === 'debounce')
      this.timers.set(
        name,
        setTimeout(() => {
          this.timers.delete(name)
          void this.commit([name])
        }, rule.debounceMs ?? 1000),
      )
  }
  blur = (name: string) => {
    const mode = this.rule(name).mode
    if (!this.buttonOnly() && (mode === 'blur' || mode === 'debounce')) {
      this.clear(name)
      void this.commit([name])
    }
  }
  enter = (name: string) => {
    if (!this.buttonOnly() && this.rule(name).mode !== 'button') {
      this.clear(name)
      void this.commit([name])
    }
  }
  perform = async (operation: () => void | Promise<void>, names: string[]) => {
    try {
      await operation()
      if (!this.buttonOnly()) await this.commit(names)
    } catch (error) {
      this.failure = error
      this.paused = true
      this.failedNames = names
      this.emit()
      this.options.error?.(error, () => {
        void this.retry()
      })
    }
  }
  escape = (name: string) => {
    this.clear(name)
    this.queued.delete(name)
    this.set(name, this.active.has(name) ? this.active.get(name)!.value : getBy(this.confirmed, name))
  }
  reset = (values: T = this.initial) => {
    this.stop()
    this.initial = values
    this.confirmed = values
    if (this.shared && this.options.names) {
      for (const name of this.options.names) this.set(name, getBy(values, name))
    } else this.api.reset(values)
    this.failure = null
    this.paused = false
    this.invalidFields.clear()
    this.emit()
  }
  accept = (patch: Partial<T>) => {
    this.confirmed = { ...this.confirmed, ...patch }
    this.emit()
  }
  retry = () => {
    this.paused = false
    this.failure = null
    return this.commit(this.failedNames.length ? this.failedNames : (this.options.names ?? Object.keys(this.values)), true)
  }
  submit = () => this.commit(this.options.names ?? Object.keys(this.values), true)
  commit = async (names: string[], explicit = false): Promise<void> => {
    if (this.paused && !explicit) return
    if (explicit) {
      this.paused = false
      this.failure = null
    }
    for (const name of names) {
      this.clear(name)
      const value = this.get(name),
        version = this.versions.get(name) ?? 0
      const active = this.active.get(name)
      if (active && Object.is(active.value, value) && active.version === version) continue
      this.queued.set(name, { value, version })
    }
    await this.flush(explicit)
  }
  private async flush(explicit = false): Promise<void> {
    if (this.request || this.paused || !this.options.submit) return
    const firstName = this.queued.keys().next().value as string | undefined
    if (firstName === undefined) return
    const group = this.rule(firstName).group
    const selected = [...this.queued].filter(([name]) => this.rule(name).group === group)
    for (const [name] of selected) this.queued.delete(name)
    const changes = new Map(
      selected.filter(
        ([name, entry]) => (explicit && this.options.command) || !Object.is(entry.value, getBy(this.confirmed, name)),
      ),
    )
    if (!changes.size) {
      this.emit()
      if (this.queued.size) await this.flush(explicit)
      return
    }
    let snapshot = { ...this.values }
    let patch: Record<string, unknown> = {}
    for (const [name, entry] of changes) snapshot = setBy(snapshot, name, entry.value)
    for (const [name, entry] of changes) patch = setBy(patch, name, entry.value)
    const generation = this.generation
    const request = new AbortController()
    this.request = request
    this.active = changes
    this.emit()
    try {
      if (this.options.validate && !(await this.options.validate(snapshot, [...changes.keys()]))) {
        this.invalidFields = new Set(changes.keys())
        return
      }
      if (generation !== this.generation) return
      const result = await this.options.submit(snapshot, patch as Partial<T>, request.signal, [...changes.keys()])
      if (generation !== this.generation || request.signal.aborted) return
      if (result === false) {
        this.failedNames = [...changes.keys(), ...this.queued.keys()]
        this.paused = true
        return
      }
      const accepted = result ?? (patch as Partial<T>)
      for (const [name, entry] of changes) {
        if (!Object.hasOwn(accepted, name.split(/[.[\]]/)[0])) continue
        this.confirmed = setBy(this.confirmed, name, getBy(accepted, name))
        if ((this.versions.get(name) ?? 0) === entry.version) this.set(name, getBy(this.confirmed, name))
      }
      this.failedNames = []
      this.failure = null
    } catch (error) {
      if (generation !== this.generation || request.signal.aborted) return
      this.failedNames = [...changes.keys(), ...this.queued.keys()]
      this.failure = error
      this.paused = true
      this.options.error?.(error, () => {
        void this.retry()
      })
    } finally {
      if (generation === this.generation) {
        this.request = undefined
        this.active.clear()
        this.emit()
        if (!this.paused && this.queued.size) await this.flush()
      }
    }
  }
  stop = () => {
    this.generation++
    this.timers.forEach(clearTimeout)
    this.timers.clear()
    this.queued.clear()
    this.active.clear()
    this.request?.abort()
    this.request = undefined
    this.paused = false
    this.failure = null
    this.emit()
  }
  mount = () => {
    this.unsubscribeApi ??= this.api.store.subscribe(() => this.emit())
    if (this.mounted++ === 0 && !this.shared) this.unmountApi = this.api.mount()
    return () => {
      if (--this.mounted === 0) {
        this.stop()
        this.unmountApi?.()
        if (this.shared) {
          this.unsubscribeApi?.unsubscribe()
          this.unsubscribeApi = undefined
        }
      }
    }
  }
}
export const createForm = <T extends object>(values: T, options?: FormOptions<T>) => new FormModel(values, options)

export type FormStore<T extends object> = StoreApi<T> & { form: FormModel<Partial<T>> }
export function createFormStore<T extends object>(initial: T, editable: (keyof T)[]): FormStore<T> {
  const values: Partial<T> = {},
    workflow = { ...initial }
  for (const key of editable) {
    values[key] = initial[key]
    delete workflow[key]
  }
  const store = createStore<T>(() => workflow)
  const form = createForm(values)
  let snapshot = { ...workflow, ...form.values } as T
  const listeners = new Set<(state: T, previous: T) => void>()
  const publish = () => {
    const previous = snapshot
    snapshot = { ...store.getState(), ...form.values }
    if (Object.keys(snapshot).some((key) => !Object.is(snapshot[key as keyof T], previous[key as keyof T])))
      listeners.forEach((listener) => listener(snapshot, previous))
    else snapshot = previous
  }
  store.subscribe(publish)
  form.subscribe(publish)
  const setState: StoreApi<T>['setState'] = (update) => {
    const patch = typeof update === 'function' ? update(snapshot) : update
    const rest = { ...patch }
    for (const key of editable)
      if (Object.hasOwn(patch, key)) {
        form.set(String(key), patch[key])
        delete rest[key]
      }
    store.setState(rest)
  }
  return {
    form,
    setState,
    getState: () => snapshot,
    getInitialState: () => initial,
    subscribe: (listener) => {
      listeners.add(listener)
      return () => {
        listeners.delete(listener)
      }
    },
  }
}
