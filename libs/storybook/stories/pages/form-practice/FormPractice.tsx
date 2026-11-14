import { useState, useSyncExternalStore } from 'react'
import { Form, createForm, contractResult, useFormValue, type FormModel, type CommitMode } from '../../../src/components/form'
import { TextField, MultilineField, CodeEntry } from '../../../src/components/fields'
import { Button } from '../../../src/components/buttons'
import { Switch } from '../../../src/components/selection'
import { RoleUpdateInput, EmailCodeInput } from '@compallm/contract/validation'
import { FormPracticeController, initial } from './controller'
import { Select } from '../../../src/components/dropdowns'
import { Toast } from '../../../src/components/feedback'
import { useStore } from 'zustand'

function Session() {
  const [controller] = useState(() => new FormPracticeController())
  const [buttonOnly, setButtonOnly] = useState(false)
  const [mode, setMode] = useState<CommitMode>('blur')
  const [failure, setFailure] = useState(false)
  const [scratch, setScratch] = useState('Local scratchpad')
  const [events, setEvents] = useState<string[]>([])
  const [form] = useState(() =>
    createForm(
      { name: initial.name, description: initial.description },
      {
        mode: 'blur',
        fields: { description: { mode: 'debounce' } },
        parse: (values, names) => {
          const result = RoleUpdateInput.safeParse(
            Object.fromEntries(names.map((name) => [name, values[name as keyof typeof values]])),
          )
          return contractResult(result, result.success ? ({ ...values, ...result.data } as typeof values) : values)
        },
        error: () => setFailure(true),
        submit: async (_values, patch, signal) => {
          setEvents((current) => [`Form → Controller: ${JSON.stringify(patch)}`, ...current].slice(0, 12))
          const result = await controller.update(patch, signal)
          setEvents((current) => ['Controller → Form: reconcile returned values', ...current].slice(0, 12))
          return result
        },
      },
    ),
  )
  const configure = (only: boolean, nextMode = mode) => {
    setButtonOnly(only)
    form.configure({
      mode: only ? 'button' : nextMode,
      fields: { description: { mode: 'debounce' } },
      parse: (values, names) => {
        const result = RoleUpdateInput.safeParse(
          Object.fromEntries(names.map((name) => [name, values[name as keyof typeof values]])),
        )
        return contractResult(result, result.success ? ({ ...values, ...result.data } as typeof values) : values)
      },
      error: () => setFailure(true),
      submit: async (_values, patch, signal) => {
        setEvents((current) => [`Form → Controller: ${JSON.stringify(patch)}`, ...current].slice(0, 12))
        const result = await controller.update(patch, signal)
        setEvents((current) => ['Controller → Form: reconcile returned values', ...current].slice(0, 12))
        return result
      },
    })
  }
  useSyncExternalStore(
    form.subscribe,
    () => form.version,
    () => form.version,
  )
  const slow = useStore(controller.store, (state) => state.slow)
  const confirmed = useStore(controller.store, (state) => state.confirmed)
  const request = useStore(controller.store, (state) => state.request)
  const response = useStore(controller.store, (state) => state.response)
  return (
    <main className="mx-auto grid max-w-7xl gap-6 p-4 sm:p-8">
      <h1 className="text-2xl font-semibold">Form practice</h1>
      <p className="text-sm text-muted-foreground">
        Fields find their nearest Form. Form owns React Hook Form values and commit policy.
      </p>
      <div className="flex flex-wrap items-center gap-4 rounded-lg bg-inset p-4">
        <label className="flex items-center gap-2 text-xs">
          <Switch checked={buttonOnly} onChange={(event) => configure(event.currentTarget.checked)} />
          Submit button only
        </label>
        <label className="flex items-center gap-2 text-xs">
          <Switch checked={slow} onChange={(event) => controller.setSlow(event.currentTarget.checked)} />
          Slow responses
        </label>
        <Select.Control
          aria-label="Commit behavior"
          value={mode}
          options={['blur', 'change', 'debounce', 'button'].map((value) => ({ value, label: value }))}
          onValueChange={(value) => {
            setMode(value as CommitMode)
            configure(buttonOnly, value as CommitMode)
          }}
        />
        <Button onClick={controller.reject}>Reject next write</Button>
      </div>
      <div className="grid items-start gap-6 lg:grid-cols-2">
        <Form.Root model={form} className="grid gap-4 rounded-xl border border-divider bg-surface p-5">
          <h2 className="text-sm font-medium">Edit role</h2>
          <label className="grid gap-2 text-sm">
            Role name
            <TextField name="name" />
            <Form.Error name="name" />
          </label>
          <label className="grid gap-2 text-sm">
            Description
            <MultilineField name="description" />
            <Form.Error name="description" />
          </label>
          <p className="text-xs text-muted-foreground">
            Name commits on blur/Enter; description after a pause or blur. Button-only suppresses every automatic trigger.
            Escape restores a field.
          </p>
          <div className="flex items-center gap-3">
            <Form.Submit variant="primary">Apply changes</Form.Submit>
            <Form.Reset>Reset fields</Form.Reset>
            <Form.Status />
          </div>
        </Form.Root>
        <section className="grid gap-3 rounded-xl border border-divider bg-surface p-5 text-xs">
          <h2 className="text-sm font-medium">Draft → Submitted → Confirmed</h2>
          {(['name', 'description'] as const).map((name) => (
            <div key={name} className="grid gap-1 border-b border-divider pb-3">
              <strong>{name}</strong>
              <DraftValue form={form} name={name} />
              <p>Submitted: {String(form.submitted(name) ?? '—')}</p>
              <p>Confirmed: {confirmed[name]}</p>
            </div>
          ))}
          <h3 className="text-sm">Another view: {confirmed.name}</h3>
          <p>{confirmed.description}</p>
        </section>
      </div>
      <section className="grid gap-4 rounded-xl border border-divider p-5">
        <h2 className="text-sm font-medium">Actual Genql request and mock response</h2>
        <div className="grid min-w-0 gap-4 md:grid-cols-2">
          <pre className="overflow-auto rounded-lg bg-inset p-3 text-xs">{JSON.stringify(request, null, 2)}</pre>
          <pre className="overflow-auto rounded-lg bg-inset p-3 text-xs">{JSON.stringify(response, null, 2)}</pre>
        </div>
        <ol className="grid gap-2 text-xs">
          {events.map((event, i) => (
            <li key={i}>{event}</li>
          ))}
        </ol>
      </section>
      <CompletionPractice />
      <LargeFormPractice />
      <section className="grid gap-3 rounded-xl border border-divider p-5">
        <h2 className="text-sm font-medium">Same field outside Form</h2>
        <TextField
          name="name"
          aria-label="Standalone field"
          value={scratch}
          onChange={(event) => setScratch(event.target.value)}
        />
        <p className="text-xs text-muted-foreground">Caller manages value and listeners: {scratch}. No mutation is sent.</p>
      </section>
      {failure ? (
        <div className="fixed right-6 bottom-6 z-50">
          <Toast
            tone="error"
            message="Mock write rejected. Your draft is retained."
            onDismiss={() => setFailure(false)}
            action={{
              label: 'Retry',
              onClick: () => {
                setFailure(false)
                void form.retry()
              },
            }}
          />
        </div>
      ) : null}
    </main>
  )
}
export function FormPractice() {
  const [session, setSession] = useState(0)
  return (
    <>
      <Button onClick={() => setSession((value) => value + 1)}>Reset practice</Button>
      <Session key={session} />
    </>
  )
}

function CompletionPractice() {
  const [verified, setVerified] = useState('')
  const [form] = useState(() =>
    createForm(
      { code: '' },
      {
        mode: 'complete',
        command: true,
        fields: { code: { complete: (value) => EmailCodeInput.safeParse({ code: value }).success } },
        parse: (values) => contractResult(EmailCodeInput.safeParse(values), values),
        submit: (values) => {
          setVerified(values.code)
          return values
        },
      },
    ),
  )
  return (
    <Form.Root model={form} className="grid gap-3 rounded-xl border border-divider p-5">
      <h2 className="text-sm font-medium">Completion commit</h2>
      <CodeEntry name="code" aria-label="Practice verification code" />
      <Form.Error name="code" />
      <p className="text-xs text-muted-foreground">Mock verified code: {verified || '—'}</p>
    </Form.Root>
  )
}
function LargeFormPractice() {
  const [form] = useState(() =>
    createForm(Object.fromEntries(Array.from({ length: 1000 }, (_, index) => [`field${index}`, ''])), {
      mode: 'button',
      submit: (_values, patch) => patch,
    }),
  )
  return (
    <details className="rounded-xl border border-divider p-5">
      <summary>Large form: 1,000 registered native inputs</summary>
      <Form.Root model={form} className="grid max-h-96 gap-3 overflow-auto p-3">
        {Array.from({ length: 1000 }, (_, index) => (
          <label key={index} className="grid gap-1 text-xs">
            Field {index + 1}
            <TextField name={`field${index}`} />
          </label>
        ))}
        <Form.Submit>Commit</Form.Submit>
      </Form.Root>
    </details>
  )
}

function DraftValue({ form, name }: { form: FormModel<{ name: string; description: string }>; name: string }) {
  const value = useFormValue<string>(form, name)
  return <p>Draft: {value}</p>
}
