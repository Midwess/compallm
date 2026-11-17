import { FormModel, type FormOptions } from './form-model'

type RecordForm = { form: FormModel<object>; release(): void; stop(): void; views: number }

export class CollectionForms {
  private records = new Map<string, RecordForm>()
  has(key: string) {
    return this.records.has(key)
  }
  get<T extends object>(key: string, initial: T, options: FormOptions<T>): FormModel<T> {
    const existing = this.records.get(key)
    if (existing) return existing.form as unknown as FormModel<T>
    const form = new FormModel(initial, options)
    const entry: RecordForm = { form: form as unknown as FormModel<object>, release: form.mount(), stop: () => {}, views: 0 }
    this.records.set(key, entry)
    entry.stop = form.subscribe(() => this.collect(key))
    return form
  }
  attach<T extends object>(key: string, form: FormModel<T>) {
    if (!this.records.has(key)) {
      const entry: RecordForm = { form: form as unknown as FormModel<object>, release: form.mount(), stop: () => {}, views: 0 }
      this.records.set(key, entry)
      entry.stop = form.subscribe(() => this.collect(key))
    }
    const record = this.records.get(key)!
    record.views++
    return () => {
      record.views--
      this.collect(key)
    }
  }
  private collect(key: string) {
    const record = this.records.get(key)
    if (!record || record.views || record.form.pending || record.form.dirty) return
    this.records.delete(key)
    record.stop()
    record.release()
  }
  dispose = () => {
    const records = [...this.records.values()]
    this.records.clear()
    for (const record of records) {
      record.stop()
      record.form.stop()
      record.release()
    }
  }
}
