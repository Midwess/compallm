import { createContext } from 'svelte'
import { RoleUpdateInput, EmailCodeInput } from '@compallm/contract/validation'
import { createForm, type CommitMode } from '../../src/components/form-model.svelte'
export class FormPracticeController {
  confirmed = $state({ name: 'Model operator', description: 'Explore models with the team.' })
  slow = $state(true)
  rejectNext = $state(false)
  request = $state<object | null>(null)
  response = $state<object | null>(null)
  commits = $state(0)
  readonly form = createForm(
    { ...this.confirmed },
    {
      mode: 'blur',
      fields: { name: { mode: 'blur' }, description: { mode: 'blur' } },
      parse: (values, names) => {
        const patch = Object.fromEntries(names.map((name) => [name, values[name as keyof typeof values]]))
        const result = RoleUpdateInput.safeParse(patch)
        return result.success
          ? {
              values: {
                ...values,
                name: result.data.name ?? values.name,
                description: result.data.description ?? values.description,
              },
            }
          : { issues: result.error.issues }
      },
      submit: async (_values, patch, signal) => {
        const fail = this.rejectNext
        this.rejectNext = false
        this.request = { roleUpdate: { id: 'practice-role', input: patch } }
        this.response = null
        this.commits++
        await new Promise<void>((resolve, reject) => {
          const abort = () => {
            clearTimeout(timer)
            reject(new DOMException('Cancelled', 'AbortError'))
          }
          const timer = setTimeout(
            () => {
              signal.removeEventListener('abort', abort)
              resolve()
            },
            this.slow ? 3000 : 100,
          )
          if (signal.aborted) abort()
          else signal.addEventListener('abort', abort, { once: true })
        })
        if (fail) throw new Error('Mock write rejected')
        const parsed = RoleUpdateInput.parse(patch)
        this.confirmed = {
          name: parsed.name ?? this.confirmed.name,
          description: parsed.description ?? this.confirmed.description,
        }
        this.response = { data: { roleUpdate: { node: this.confirmed } } }
        return this.confirmed
      },
    },
  )
  configureMode(mode: CommitMode) {
    this.form.setMode(mode)
  }
  readonly otp = createForm(
    { code: '' },
    {
      mode: 'complete',
      command: true,
      fields: {
        code: {
          mode: 'complete',
          complete: (value) => typeof value === 'string' && EmailCodeInput.safeParse({ code: value }).success,
        },
      },
      parse: (values) => {
        const result = EmailCodeInput.safeParse(values)
        return result.success ? { values: result.data } : { issues: result.error.issues }
      },
      submit: (values) => values,
    },
  )
  stop() {
    this.form.stop()
    this.otp.stop()
  }
}
export const [getPracticeController, setPracticeController] = createContext<FormPracticeController>()
