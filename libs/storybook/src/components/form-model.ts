import { createFormControl, type FieldValues, type UseFormReturn, type Resolver } from 'react-hook-form'
import getBy from 'lodash-es/get.js'
import cloneDeepWith from 'lodash-es/cloneDeepWith.js'
import isEqual from 'lodash-es/isEqual.js'
import set from 'lodash-es/set.js'

export function cloneFormValue<T>(value: T): T {
  return cloneDeepWith(value, (item) => (typeof Blob !== 'undefined' && item instanceof Blob ? item : undefined))
}

const cloneDeep = cloneFormValue

function setBy<V>(source: V, path: string, value: unknown): V {
  const keys = path.replace(/\[(\d+)\]/g, '.$1').split('.')
  const result = { ...source }
  let cursor = result as Record<string, unknown>
  let original = source as Record<string, unknown>
  for (const key of keys.slice(0, -1)) {
    const child = original?.[key]
    cursor[key] = Array.isArray(child) ? [...child] : { ...(child as object) }
    cursor = cursor[key] as Record<string, unknown>
    original = child as Record<string, unknown>
  }
  set(cursor, keys.at(-1)!, value)
  return result
}
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
  resolver?: Resolver<T>
  names?: string[]
  validationNames?(names: string[]): string[]
  fields?: Record<string, FormRule>
  validate?(values: T, names: string[]): boolean | Promise<boolean>
  parse?(values: T, names: string[]): { values?: T; issues?: { path: PropertyKey[]; message: string }[] }
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
  readonly api: Omit<UseFormReturn<FieldValues>, 'formState'>
  readonly editVersion: number
  subscribe(listener: () => void, name?: string): () => void
  message(name: string): string | undefined
  validationVersion(name: string): number
  native(name: string): ReturnType<Omit<UseFormReturn<FieldValues>, 'formState'>['register']>
  invalid(name: string): boolean
  buttonOnly(): boolean
}

export class FormModel<T extends object> implements FormScope {
  readonly api: Omit<UseFormReturn<FieldValues>, 'formState'>
  private initial: T
  private confirmed: T
  private options: FormOptions<T>
  private listeners = new Set<() => void>()
  private elements = new Map<string, HTMLElement>()
  private fieldListeners = new Map<string, Set<() => void>>()
  private versions = new Map<string, number>()
  private queued = new Map<string, Pending>()
  private active = new Map<string, Pending>()
  private timers = new Map<string, ReturnType<typeof setTimeout>>()
  private dirtyNames = new Set<string>()
  private validationRevisions = new Map<string, number>()
  private invalidFields = new Set<string>()
  private request?: AbortController
  private generation = 0
  private paused = false
  private revision = 0
  private edits = { version: 0 }
  get editVersion() {
    return this.edits.version
  }
  private failedNames: string[] = []
  private failure: unknown = null
  private shared = false
  private mounted = 0

  constructor(
    values: T,
    options: FormOptions<T> = {},
    api?: Omit<UseFormReturn<FieldValues>, 'formState'>,
    versions?: Map<string, number>,
    fieldListeners?: Map<string, Set<() => void>>,
    edits?: { version: number },
    validationRevisions?: Map<string, number>,
  ) {
    if (validationRevisions) this.validationRevisions = validationRevisions
    if (edits) this.edits = edits
    if (versions) this.versions = versions
    if (fieldListeners) this.fieldListeners = fieldListeners
    this.initial = cloneDeep(values)
    this.confirmed = cloneDeep(values)
    this.options = options
    this.shared = Boolean(api)
    this.api = api ?? createFormControl<FieldValues>({ defaultValues: values, mode: 'onBlur', reValidateMode: 'onChange' })
    if (!api) this.api.subscribe({ formState: {}, callback: () => {} })
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
      this.fieldListeners,
      this.edits,
      this.validationRevisions,
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
    return this.api.getValues() as T
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
    return this.dirtyNames.size > 0
  }
  get version(): number {
    return this.revision
  }
  submitted = (name: string) => this.active.get(name)?.value
  waiting = (name: string) => this.queued.has(name)
  message = (name: string): string | undefined => {
    const messages = new Set<string>()
    const collect = (error: unknown) => {
      if (!error || typeof error !== 'object') return
      if ('message' in error && typeof error.message === 'string') messages.add(error.message)
      else for (const [key, child] of Object.entries(error)) if (key !== 'ref') collect(child)
    }
    collect(this.api.getFieldState(name).error)
    return [...messages].join(' ') || undefined
  }
  validationVersion = (name: string) => this.validationRevisions.get(name) ?? 0
  private markValidation(name: string) {
    const path = name.split('.')
    while (path.length) {
      const key = path.join('.')
      this.validationRevisions.set(key, this.validationVersion(key) + 1)
      path.pop()
    }
  }
  invalid = (name: string) => this.invalidFields.has(name) || Boolean(this.message(name))
  native = (name: string) => {
    const registration = this.api.register(name)
    return {
      ...registration,
      ref: (element: HTMLInputElement | null) => {
        registration.ref(element)
        if (element) this.elements.set(name, element)
        else this.elements.delete(name)
      },
      onChange: async (event: { target: unknown; type?: string }) => {
        const target = event.target as HTMLInputElement
        this.change(name, target.type === 'checkbox' ? target.checked : target.value)
        if (target.type !== 'checkbox' && target.value !== String(this.get(name) ?? ''))
          target.value = String(this.get(name) ?? '')
      },
      onBlur: async () => this.blur(name),
    }
  }
  has = (name: string) =>
    (!this.options.names ||
      this.options.names.some((root) => name === root || name.startsWith(root + '.') || name.startsWith(root + '['))) &&
    Object.hasOwn(this.initial, name.split(/[.[\]]/)[0])
  get = (name: string): unknown => this.api.getValues(name)
  buttonOnly = () => (this.options.mode ?? 'button') === 'button'
  subscribe = (listener: () => void, name?: string) => {
    const listeners = name ? (this.fieldListeners.get(name) ?? new Set<() => void>()) : this.listeners
    if (name) this.fieldListeners.set(name, listeners)
    listeners.add(listener)
    return () => {
      listeners.delete(listener)
      if (name && !listeners.size) this.fieldListeners.delete(name)
    }
  }
  private notify(name: string) {
    const element = this.elements.get(name)
    if (element) element.setAttribute('aria-invalid', String(this.invalid(name)))
    const paths = name.replace(/\[(\d+)\]/g, '.$1').split('.')
    while (paths.length) {
      this.fieldListeners.get(paths.join('.'))?.forEach((listener) => listener())
      paths.pop()
    }
    if (this.get(name) && typeof this.get(name) === 'object') {
      for (const [path, element] of this.elements)
        if (path.startsWith(name + '.')) element.setAttribute('aria-invalid', String(this.invalid(path)))
      for (const [path, listeners] of this.fieldListeners)
        if (path.startsWith(name + '.') || path.startsWith(name + '[')) listeners.forEach((listener) => listener())
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
    const ancestors = name.replace(/\[(\d+)\]/g, '.$1').split('.')
    ancestors.pop()
    while (ancestors.length) {
      const key = ancestors.join('.')
      this.versions.set(key, (this.versions.get(key) ?? 0) + 1)
      ancestors.pop()
    }
    if (value && typeof value === 'object')
      for (const key of this.versions.keys())
        if (key.startsWith(name + '.') || key.startsWith(name + '[')) this.versions.set(key, (this.versions.get(key) ?? 0) + 1)
    this.api.setValue(name, value, { shouldDirty: true })
    this.edits.version++
    this.notify(name)
    const wasDirty = this.dirty
    if (isEqual(this.get(name), getBy(this.confirmed, name))) this.dirtyNames.delete(name)
    else this.dirtyNames.add(name)
    if (wasDirty !== this.dirty) this.emit()
  }
  change = (name: string, value: unknown) => {
    const rule = this.rule(name)
    if (rule.accepts && !rule.accepts(value)) return
    const different = !Object.is(this.get(name), value)
    this.set(name, value)
    if (different && rule.mode === 'complete') this.paused = false
    if (this.invalid(name)) void this.validate([name])
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
    if (this.buttonOnly() || (mode !== 'blur' && mode !== 'debounce')) void this.validate([name])
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
    this.initial = cloneDeep(values)
    this.confirmed = cloneDeep(values)
    if (this.shared && this.options.names) {
      for (const name of this.options.names) this.set(name, getBy(values, name))
    } else {
      this.api.reset(values, { keepFieldsRef: true })
      this.edits.version++
    }
    this.failure = null
    this.paused = false
    this.dirtyNames.clear()
    this.invalidFields.clear()
    this.api.clearErrors()
    for (const name of this.fieldListeners.keys()) this.notify(name)
    this.emit()
  }
  accept = (patch: Partial<T>) => {
    this.confirmed = cloneDeep({ ...this.confirmed, ...patch })
    for (const name of this.dirtyNames) if (isEqual(this.get(name), getBy(this.confirmed, name))) this.dirtyNames.delete(name)
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
      if (active && isEqual(active.value, value) && active.version === version) continue
      this.queued.set(name, { value: cloneDeep(value), version })
    }
    await this.flush(explicit)
  }
  private async validate(names: string[], values = this.values): Promise<T | false> {
    const boundary = this.options.validationNames?.(names) ?? names
    const validationGeneration = this.generation
    const versions = names.map((name) => this.versions.get(name) ?? 0)
    const current = () =>
      validationGeneration === this.generation &&
      names.every((name, index) => (this.versions.get(name) ?? 0) === versions[index])
    const resolved = this.options.resolver
      ? await this.options.resolver(values, undefined, { fields: {}, shouldUseNativeValidation: false })
      : undefined
    const parsed = this.options.parse?.(values, names)
    if (!current())
      return parsed?.issues?.length || (resolved && Object.keys(resolved.errors).length)
        ? false
        : (parsed?.values ?? (resolved?.values as T | undefined) ?? values)
    for (const name of boundary) {
      this.api.clearErrors(name)
      for (const invalid of this.invalidFields)
        if (invalid === name || invalid.startsWith(name + '.')) this.invalidFields.delete(invalid)
    }
    if (resolved && Object.keys(resolved.errors).length) {
      const apply = (errors: object, prefix = '') => {
        for (const [key, error] of Object.entries(errors)) {
          if (!error) continue
          const path = prefix ? `${prefix}.${key}` : key
          if (error.message) {
            this.api.setError(path, error)
            this.markValidation(path)
            this.notify(path)
          } else apply(error, path)
        }
      }
      apply(resolved.errors)
      return false
    }
    if (parsed?.issues?.length) {
      for (const issue of parsed.issues) {
        const name = issue.path.join('.') || names[0]
        this.api.setError(name, { type: 'contract', message: issue.message })
        this.markValidation(name)
        this.invalidFields.add(name)
        this.notify(name)
      }
      return false
    }
    if (this.options.validate && !(await this.options.validate(values, names))) {
      names.forEach((name) => {
        this.invalidFields.add(name)
        this.notify(name)
      })
      return false
    }
    boundary.forEach((name) => this.notify(name))
    return parsed?.values ?? (resolved?.values as T | undefined) ?? values
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
        ([name, entry]) => (explicit && this.options.command) || !isEqual(entry.value, getBy(this.confirmed, name)),
      ),
    )
    if (!changes.size) {
      this.emit()
      if (this.queued.size) await this.flush(explicit)
      return
    }
    let snapshot = cloneDeep(this.values)
    let patch: Record<string, unknown> = {}
    for (const [name, entry] of changes) snapshot = setBy(snapshot, name, entry.value)
    for (const [name, entry] of changes) patch = setBy(patch, name, entry.value)
    const generation = this.generation
    const request = new AbortController()
    this.request = request
    this.active = changes
    this.emit()
    try {
      const parsed = await this.validate([...changes.keys()], snapshot)
      if (!parsed) return
      snapshot = parsed
      for (const name of changes.keys()) patch = setBy(patch, name, getBy(snapshot, name))
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
      for (const name of this.dirtyNames) if (isEqual(this.get(name), getBy(this.confirmed, name))) this.dirtyNames.delete(name)
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
    this.mounted++
    return () => {
      if (--this.mounted === 0) {
        this.stop()
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
  let workflowVersion = 0
  let readVersion = -1
  let editVersion = -1
  const read = () => {
    if (readVersion !== workflowVersion || editVersion !== form.editVersion) {
      snapshot = { ...store.getState(), ...form.values } as T
      readVersion = workflowVersion
      editVersion = form.editVersion
    }
    return snapshot
  }
  const listeners = new Set<(state: T, previous: T) => void>()
  const publish = () => {
    const previous = snapshot
    snapshot = { ...store.getState(), ...form.values }
    if (Object.keys(snapshot).some((key) => !Object.is(snapshot[key as keyof T], previous[key as keyof T])))
      listeners.forEach((listener) => listener(snapshot, previous))
    else snapshot = previous
  }
  store.subscribe(() => {
    workflowVersion++
    publish()
  })
  const setState: StoreApi<T>['setState'] = (update) => {
    const patch = typeof update === 'function' ? update(read()) : update
    const rest = { ...patch }
    for (const key of editable)
      if (Object.hasOwn(patch, key)) {
        form.set(String(key), patch[key])
        delete rest[key]
      }
    if (Object.keys(rest).some((key) => !Object.is(rest[key as keyof T], store.getState()[key as keyof T])))
      store.setState(rest)
  }
  return {
    form,
    setState,
    getState: read,
    getInitialState: () => initial,
    subscribe: (listener) => {
      listeners.add(listener)
      return () => {
        listeners.delete(listener)
      }
    },
  }
}
