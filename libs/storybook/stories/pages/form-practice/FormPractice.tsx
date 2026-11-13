import { useState, useSyncExternalStore } from 'react'
import { Form, createForm } from '../../../src/components/form'
import { TextField, MultilineField } from '../../../src/components/fields'
import { Button } from '../../../src/components/buttons'
import { Switch } from '../../../src/components/selection'
import { RoleUpdateInput } from '@compallm/contract/validation'
import { FormPracticeController, initial } from './controller'
import { useStore } from 'zustand'

function Session() {
  const [controller] = useState(() => new FormPracticeController())
  const [buttonOnly, setButtonOnly] = useState(false)
  const [scratch, setScratch] = useState('Local scratchpad')
  const [events, setEvents] = useState<string[]>([])
  const [form] = useState(() =>
    createForm(
      { name: initial.name, description: initial.description },
      {
        mode: 'blur',
        fields: { description: { mode: 'debounce' } },
        validate: (values, names) =>
          RoleUpdateInput.safeParse(Object.fromEntries(names.map((name) => [name, values[name as keyof typeof values]])))
            .success,
        submit: async (_values, patch, signal) => {
          setEvents((current) => [`Form → Controller: ${JSON.stringify(patch)}`, ...current].slice(0, 12))
          const result = await controller.update(patch, signal)
          setEvents((current) => ['Controller → Form: reconcile returned values', ...current].slice(0, 12))
          return result
        },
      },
    ),
  )
  const configure = (only: boolean) => {
    setButtonOnly(only)
    form.configure({
      mode: only ? 'button' : 'blur',
      fields: { description: { mode: 'debounce' } },
      changed: (name) => setEvents((current) => [`Field → Form: ${name} changed`, ...current].slice(0, 12)),
      validate: (values, names) =>
        RoleUpdateInput.safeParse(Object.fromEntries(names.map((name) => [name, values[name as keyof typeof values]]))).success,
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
      <p className="text-sm text-muted">Fields find their nearest Form. Form owns TanStack values and commit policy.</p>
      <div className="flex flex-wrap items-center gap-4 rounded-lg bg-inset p-4">
        <label className="flex items-center gap-2 text-xs">
          <Switch checked={buttonOnly} onChange={(event) => configure(event.currentTarget.checked)} />
          Submit button only
        </label>
        <label className="flex items-center gap-2 text-xs">
          <Switch checked={slow} onChange={(event) => controller.setSlow(event.currentTarget.checked)} />
          Slow responses
        </label>
        <Button onClick={controller.reject}>Reject next write</Button>
      </div>
      <div className="grid items-start gap-6 lg:grid-cols-2">
        <Form model={form} className="grid gap-4 rounded-xl border border-divider bg-surface p-5">
          <h2 className="text-sm font-medium">Edit role</h2>
          <label className="grid gap-2 text-sm">
            Role name
            <TextField name="name" />
          </label>
          <label className="grid gap-2 text-sm">
            Description
            <MultilineField name="description" />
          </label>
          <p className="text-xs text-muted">
            Name commits on blur/Enter; description after a pause or blur. Button-only suppresses every automatic trigger.
            Escape restores a field.
          </p>
          <div className="flex items-center gap-3">
            <Form.Submit variant="primary">Apply changes</Form.Submit>
            <Form.Status />
          </div>
          {form.error ? (
            <div role="alert" className="text-xs text-error">
              Couldn’t save. Your draft is retained.
              <Button type="button" onClick={() => void form.retry()}>
                Retry latest changes
              </Button>
            </div>
          ) : null}
        </Form>
        <section className="grid gap-3 rounded-xl border border-divider bg-surface p-5 text-xs">
          <h2 className="text-sm font-medium">Draft → Submitted → Confirmed</h2>
          {(['name', 'description'] as const).map((name) => (
            <div key={name} className="grid gap-1 border-b border-divider pb-3">
              <strong>{name}</strong>
              <p>Draft: {form.values[name]}</p>
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
      <section className="grid gap-3 rounded-xl border border-divider p-5">
        <h2 className="text-sm font-medium">Same field outside Form</h2>
        <TextField
          name="name"
          aria-label="Standalone field"
          value={scratch}
          onChange={(event) => setScratch(event.target.value)}
        />
        <p className="text-xs text-muted">Caller manages value and listeners: {scratch}. No mutation is sent.</p>
      </section>
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
