import { useContext } from 'react'
import { useStore } from 'zustand'
import { PasskeyControllerContext } from '../../context'
import { AuthLayout } from '../auth/layout'
import { Button } from '../../components/buttons'
import { TextField, CodeEntry } from '../../components/fields'
import { Form, useManagedForm, contractResult } from '../../components/form'
import { Dialog } from '../../components/dialog'
import { PasskeyRegistrationInput, EmailReauthenticationInput } from '@compallm/contract/validation'
import { Section, Rows, Row, Empty } from '../settings/shared'
export type { PasskeyController, PasskeyState } from './types'
function useController() {
  const controller = useContext(PasskeyControllerContext)
  if (!controller) throw new Error('PasskeyControllerContext is required.')
  return controller
}
export function SecurityEnhancementPage({ onSkip }: { onSkip: () => void }) {
  const controller = useController()
  return (
    <AuthLayout>
      <div className="grid gap-6">
        <h1 className="m-0 text-ui-28 leading-10 font-medium">Enhance your security.</h1>
        <p className="m-0 text-sm leading-6 text-muted-foreground">
          Add a passkey to sign in with your fingerprint, face, or security key. You can also set this up later in settings.
        </p>
        <Button variant="primary" size="authentication" onClick={controller.openAdd}>
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
  return (
    <Section
      title="Passkeys"
      description="Sign in with your fingerprint, face, or security key."
      action={
        status === 'ready' && records.length ? (
          <Button variant="ghost" onClick={controller.openAdd}>
            Add passkey
          </Button>
        ) : undefined
      }
    >
      {status === 'loading' ? (
        <p role="status" className="px-4 text-sm text-muted-foreground">
          Loading passkeys…
        </p>
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
            {records.map((record, index) => (
              <Row
                key={record.id}
                label={record.name}
                description={credentialDescription(record.registeredAt, record.lastUsedAt)}
                last={index === records.length - 1}
              >
                <Button variant="ghost" onClick={() => controller.openRemove(record.id)}>
                  Remove
                </Button>
              </Row>
            ))}
          </Rows>
        ) : (
          <Empty title="No passkeys yet" action={<Button onClick={controller.openAdd}>Add passkey</Button>}>
            Add a passkey for a simpler sign-in experience.
          </Empty>
        )
      ) : null}
    </Section>
  )
}
export function PasskeyDialogs() {
  const controller = useController()
  const dialog = useStore(controller.store, (s) => s.dialog)
  return dialog ? <PasskeyDialogContent /> : null
}
function PasskeyDialogContent() {
  const controller = useController()
  const busy = useStore(controller.store, (s) => s.busy)
  const error = useStore(controller.store, (s) => s.error)
  const reauthentication = useStore(controller.store, (s) => s.reauthentication)
  const emailSent = useStore(controller.store, (s) => s.emailSent)
  const dialog = useStore(controller.store, (s) => s.dialog)!
  const form = useManagedForm(
    { name: '', code: '' },
    {
      mode: 'button',
      command: true,
      parse: (values) => {
        const state = controller.store.getState()
        if (state.reauthentication)
          return state.emailSent
            ? contractResult(EmailReauthenticationInput.pick({ code: true }).safeParse({ code: values.code }), values)
            : { values }
        return state.dialog?.kind === 'add'
          ? contractResult(PasskeyRegistrationInput.safeParse({ name: values.name }), values)
          : { values }
      },
      submit: async (values) => {
        const state = controller.store.getState()
        if (state.reauthentication) {
          await controller.verifyEmail(values.code)
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
      <Dialog.Content className="w-110 rounded-xl bg-elevated">
        <Form.Root model={form}>
          <Dialog.Header>
            <Dialog.Title>
              {reauthentication ? 'Verify your identity' : dialog.kind === 'add' ? 'Add a passkey' : 'Remove passkey?'}
            </Dialog.Title>
            <Dialog.Close />
          </Dialog.Header>
          <Dialog.Body className="grid gap-4">
            <Dialog.Description>
              {reauthentication
                ? 'Verify with a passkey or an email code, then confirm your change.'
                : dialog.kind === 'add'
                  ? 'Give this passkey a name, then follow your device’s instructions.'
                  : 'You will no longer be able to sign in with this passkey. Existing sessions stay signed in.'}
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
            ) : dialog.kind === 'add' ? (
              <label className="grid gap-2 text-sm">
                Passkey name
                <TextField name="name" placeholder="For example, Personal laptop" />
                <Form.Error name="name" />
              </label>
            ) : null}
            {busy ? (
              <p role="status" className="text-sm text-muted-foreground">
                Complete verification on your device…
              </p>
            ) : null}
            {error ? (
              <p role="alert" className="text-sm text-destructive">
                {error}
              </p>
            ) : null}
          </Dialog.Body>
          <Dialog.Footer>
            <Dialog.Cancel variant="ghost">Cancel</Dialog.Cancel>
            {!reauthentication || emailSent ? (
              <Button
                type="submit"
                variant={dialog.kind === 'remove' && !reauthentication ? 'destructive' : 'primary'}
                loading={busy}
              >
                {reauthentication ? 'Verify email code' : dialog.kind === 'add' ? 'Continue' : 'Remove'}
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
