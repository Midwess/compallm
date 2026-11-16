import { useContext } from 'react'
import { useStore } from 'zustand'
import { PasskeyControllerContext } from '../../context'
import { AuthLayout } from '../auth/layout'
import { Button } from '../../components/buttons'
import { TextField, CodeEntry } from '../../components/fields'
import { Form, useManagedForm, contractResult } from '../../components/form'
import { Dialog } from '../../components/dialog'
import { PasskeyRegistrationInput, EmailCodeInput } from '@compallm/contract/validation'
import { Section, Rows, Row, Empty } from '../settings/shared'
import { Popup } from '../../components/popup'
import { SettingsDivider } from '../../components/settings'
import { PencilSimpleIcon } from '@phosphor-icons/react/dist/csr/PencilSimple'
import { TrashIcon } from '@phosphor-icons/react/dist/csr/Trash'
import { DotsThreeIcon } from '@phosphor-icons/react/dist/csr/DotsThree'
export type { PasskeyController, PasskeyState } from './types'
function useController() {
  const controller = useContext(PasskeyControllerContext)
  if (!controller) throw new Error('PasskeyControllerContext is required.')
  return controller
}
export function SecurityEnhancementPage({ onSkip }: { onSkip: () => void }) {
  const controller = useController()
  const busy = useStore(controller.store, (s) => s.busy)
  return (
    <AuthLayout>
      <div className="grid gap-6">
        <h1 className="m-0 text-ui-28 leading-10 font-medium">Enhance your security.</h1>
        <p className="m-0 text-sm leading-6 text-muted-foreground">
          Add a passkey to sign in with your fingerprint, face, or security key. You can also set this up later in settings.
        </p>
        <Button variant="primary" size="authentication" onClick={controller.openAdd} loading={busy}>
          Passkey
        </Button>
        <Button variant="ghost" onClick={onSkip}>
          Skip for now
        </Button>
      </div>
    </AuthLayout>
  )
}
export function PasskeySettings() {
  const controller = useController()
  const records = useStore(controller.store, (s) => s.records)
  const status = useStore(controller.store, (s) => s.status)
  const error = useStore(controller.store, (s) => s.error)
  const currentPasskeyId = useStore(controller.store, (s) => s.currentPasskeyId)
  const busy = useStore(controller.store, (s) => s.busy)
  return (
    <Section
      title="Passkeys"
      description="Passkeys are a secure way to sign in to your Compallm account."
      action={
        status === 'ready' && records.length ? (
          <Button variant="ghost" onClick={controller.openAdd} loading={busy}>
            New passkey
          </Button>
        ) : undefined
      }
    >
      {status === 'loading' ? (
        <Rows>
          <div role="status" className="p-4 text-xsm text-muted-foreground">
            Loading passkeys…
          </div>
        </Rows>
      ) : null}
      {status === 'error' || status === 'denied' ? (
        <Empty
          title={status === 'denied' ? 'Passkey access denied' : 'Couldn’t load passkeys'}
          action={status === 'error' ? <Button onClick={() => void controller.load()}>Retry</Button> : undefined}
        >
          {error}
        </Empty>
      ) : null}
      {status === 'ready' ? (
        records.length ? (
          <Rows>
            <ul className="m-0 list-none p-0">
              {records.map((record, index) => (
                <PasskeyRow
                  key={record.id}
                  record={record}
                  current={record.id === currentPasskeyId}
                  last={index === records.length - 1}
                />
              ))}
            </ul>
          </Rows>
        ) : (
          <Empty
            title="No passkeys registered"
            action={
              <Button onClick={controller.openAdd} loading={busy}>
                New passkey
              </Button>
            }
          >
            Add a passkey for a simpler sign-in experience.
          </Empty>
        )
      ) : null}
    </Section>
  )
}
function PasskeyRow({
  record,
  current,
  last,
}: {
  record: import('./types').PasskeyState['records'][number]
  current: boolean
  last: boolean
}) {
  const controller = useController()
  const editing = useStore(controller.store, (s) => s.editingId === record.id)
  const busy = useStore(controller.store, (s) => s.busy)
  const error = useStore(controller.store, (s) => s.error)
  const model = useManagedForm(
    { name: record.name },
    {
      mode: 'button',
      command: true,
      parse: (values) => contractResult(PasskeyRegistrationInput.safeParse(values), values),
      submit: async (values): Promise<{ name: string } | false> => {
        if (!(await controller.rename(record.id, values.name))) return false
        if (controller.store.getState().editingId === record.id && model.values.name === values.name) controller.edit(null)
        return values
      },
    },
  )
  if (editing)
    return (
      <li>
        <div className="p-4">
          <Form.Root
            model={model}
            className="flex flex-wrap items-start gap-3"
            onKeyDown={(event) => {
              if (event.key === 'Escape') {
                event.preventDefault()
                controller.edit(null)
              }
              if (event.key === 'Enter' && !event.nativeEvent.isComposing) {
                event.preventDefault()
                void model.submit()
              }
            }}
          >
            <label className="min-w-0 flex-1">
              <span className="sr-only">Passkey name</span>
              <TextField name="name" autoFocus className="w-full" />
              <Form.Error name="name" />
            </label>
            <Button variant="ghost" className="px-3" onClick={() => controller.edit(null)}>
              Cancel
            </Button>
            <Button type="submit" variant="neutral" className="h-8 px-3" loading={busy}>
              Save
            </Button>
            {error ? (
              <p role="alert" className="w-full text-xs text-destructive">
                {error}
              </p>
            ) : null}
          </Form.Root>
        </div>
        {!last ? <SettingsDivider className="mx-4" /> : null}
      </li>
    )
  return (
    <li>
      <Row
        label={record.name}
        description={current ? 'Used with current session' : credentialDescription(record.registeredAt, record.lastUsedAt)}
        last={last}
      >
        <Popup.Root>
          <Popup.Trigger aria-label={`Actions for ${record.name}`} className="size-8 p-0">
            <DotsThreeIcon size={16} aria-hidden="true" />
          </Popup.Trigger>
          <Popup.Content className="w-44" aria-label={`Actions for ${record.name}`}>
            <Popup.Item
              onSelect={() => {
                model.reset({ name: record.name })
                controller.edit(record.id)
              }}
            >
              <PencilSimpleIcon size={16} aria-hidden="true" />
              Rename…
            </Popup.Item>
            <Popup.Item className="text-destructive" onSelect={() => controller.openRemove(record.id)}>
              <TrashIcon size={16} aria-hidden="true" />
              Revoke
            </Popup.Item>
          </Popup.Content>
        </Popup.Root>
      </Row>
    </li>
  )
}
export function PasskeyDialogs() {
  const controller = useController()
  const dialog = useStore(controller.store, (s) => s.dialog)
  const enrollment = useStore(controller.store, (s) => s.enrollment)
  const reauthentication = useStore(controller.store, (s) => s.reauthentication)
  const error = useStore(controller.store, (s) => s.error)
  if (dialog?.kind === 'add' && enrollment === 'device' && !reauthentication && !error) return null
  return dialog ? <PasskeyDialogContent /> : null
}
function PasskeyDialogContent() {
  const controller = useController()
  const busy = useStore(controller.store, (s) => s.busy)
  const error = useStore(controller.store, (s) => s.error)
  const reauthentication = useStore(controller.store, (s) => s.reauthentication)
  const emailSent = useStore(controller.store, (s) => s.emailSent)
  const dialog = useStore(controller.store, (s) => s.dialog)!
  const enrollment = useStore(controller.store, (s) => s.enrollment)
  const passkeyName =
    dialog.kind === 'remove' ? controller.store.getState().records.find((record) => record.id === dialog.id)?.name : undefined
  const form = useManagedForm(
    { name: '', code: '' },
    {
      mode: 'button',
      command: true,
      parse: (values) => {
        const state = controller.store.getState()
        if (state.reauthentication)
          return state.emailSent
            ? contractResult(EmailCodeInput.pick({ code: true }).safeParse({ code: values.code }), values)
            : { values }
        return state.dialog?.kind === 'add' && state.enrollment === 'name'
          ? contractResult(PasskeyRegistrationInput.safeParse({ name: values.name }), values)
          : { values }
      },
      submit: async (values) => {
        const state = controller.store.getState()
        if (state.reauthentication) {
          await controller.verifyEmail(values.code)
          return false as const
        }
        if (state.dialog?.kind === 'add' && state.enrollment === 'device') {
          await controller.prepare()
          return false as const
        }
        if (state.dialog?.kind === 'add') await controller.add(values.name)
        else if (state.dialog?.kind === 'remove') await controller.remove(state.dialog.id)
        return false as const
      },
    },
  )
  return (
    <Dialog.Root
      open
      onOpenChange={(open) => {
        if (!open) controller.close()
      }}
    >
      <Dialog.Content
        className={
          dialog.kind === 'remove' && !reauthentication
            ? 'w-120 max-w-[calc(100vw-2rem)] rounded-xl bg-inset md:top-[28.2svh] md:right-2 md:bottom-auto md:left-61'
            : 'w-120 max-w-[calc(100vw-2rem)] rounded-xl bg-inset'
        }
      >
        <Form.Root model={form}>
          <Dialog.Header className="border-0 px-6 pt-6 pb-0">
            <Dialog.Title className="text-ui-15 leading-6 font-semibold">
              {reauthentication
                ? 'Verify your identity'
                : dialog.kind === 'add'
                  ? enrollment === 'name'
                    ? 'Name your passkey'
                    : 'Create a passkey'
                  : `Revoke access for "${passkeyName ?? 'passkey'}"?`}
            </Dialog.Title>
            {dialog.kind === 'add' || reauthentication ? <Dialog.Close /> : null}
          </Dialog.Header>
          <Dialog.Body
            className={
              dialog.kind === 'remove' && !reauthentication ? 'grid gap-4 px-6 pt-1 pb-1' : 'grid gap-4 px-6 pt-1 pb-4'
            }
          >
            <Dialog.Description className="text-ui-15 leading-6">
              {reauthentication
                ? 'Verify with a passkey or an email code, then confirm your change.'
                : dialog.kind === 'add'
                  ? enrollment === 'name'
                    ? 'Give your passkey a name so you can recognize it later.'
                    : 'Follow your device’s instructions to create a passkey.'
                  : 'You cannot undo this action.'}
            </Dialog.Description>
            {reauthentication ? (
              emailSent ? (
                <>
                  <CodeEntry name="code" aria-label="Email verification code" disabled={busy} />
                  <Form.Error name="code" />
                  <Button variant="link" onClick={() => void controller.sendEmail()} loading={busy}>
                    Send a new code
                  </Button>
                </>
              ) : (
                <>
                  <Button onClick={() => void controller.verifyPasskey()} loading={busy}>
                    Use passkey
                  </Button>
                  <Button variant="ghost" onClick={() => void controller.sendEmail()} loading={busy}>
                    Send email code
                  </Button>
                </>
              )
            ) : dialog.kind === 'add' && enrollment === 'name' ? (
              <label className="grid gap-2 text-sm">
                Passkey name
                <TextField name="name" placeholder="For example, Personal laptop" readOnly={busy} />
                <Form.Error name="name" />
              </label>
            ) : null}
            {busy ? (
              <p role="status" className="text-sm text-muted-foreground">
                {dialog.kind === 'add' && enrollment === 'name' ? 'Saving passkey…' : 'Complete verification on your device…'}
              </p>
            ) : null}
            {error ? (
              <p role="alert" className="text-sm text-destructive">
                {error}
              </p>
            ) : null}
          </Dialog.Body>
          <Dialog.Footer
            className={dialog.kind === 'remove' && !reauthentication ? 'border-0 px-6 pt-4 pb-6' : 'border-0 px-6 pt-0 pb-5'}
          >
            <Dialog.Cancel variant="neutral" className="h-8 px-3">
              Cancel
            </Dialog.Cancel>
            {!reauthentication || emailSent ? (
              <Button
                type="submit"
                variant={dialog.kind === 'remove' && !reauthentication ? 'destructive' : 'primary'}
                loading={busy}
                className="px-3 text-primary-foreground"
              >
                {reauthentication
                  ? 'Verify email code'
                  : dialog.kind === 'add'
                    ? enrollment === 'name'
                      ? 'Save'
                      : 'Try again'
                    : 'Revoke'}
              </Button>
            ) : null}
          </Dialog.Footer>
        </Form.Root>
      </Dialog.Content>
    </Dialog.Root>
  )
}

function credentialDescription(registeredAt: string, lastUsedAt: string | null) {
  const added = new Date(Number(registeredAt) * 1000).toLocaleDateString()
  const used = lastUsedAt ? `Last used ${new Date(Number(lastUsedAt) * 1000).toLocaleDateString()}` : 'Not used yet'
  return `Added ${added} · ${used}`
}
