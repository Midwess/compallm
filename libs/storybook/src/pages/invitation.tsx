import { useContext } from 'react'
import { zodResolver } from '@hookform/resolvers/zod'
import { EmailCodeInput } from '@compallm/contract/validation'
import { useStore } from 'zustand'
import type { StoreApi } from 'zustand/vanilla'
import type { InvitationContext } from '@compallm/contract/graphql'
import { InvitationControllerContext } from '../context'
import { Button } from '../components/buttons'
import { TextField, CodeEntry } from '../components/fields'
import { Form, useManagedForm } from '../components/form'

export interface InvitationController {
  store: StoreApi<{
    context: Pick<InvitationContext, 'organizationName' | 'email' | 'expiresAt' | 'signup'> | null
    loading: boolean
    step: 'continue' | 'code' | 'confirmed'
    error: string | null
    busy: boolean
  }>
  continue(): Promise<boolean>
  confirm(code: string): Promise<boolean>
  reload?(): void
}

export function InvitationPage() {
  const controller = useContext(InvitationControllerContext)
  if (!controller) throw new Error('InvitationControllerContext is required.')
  const state = useStore(controller.store)
  const form = useManagedForm(
    { code: '' },
    {
      resolver: zodResolver(EmailCodeInput.pick({ code: true })),
      submit: async (values) => ((await controller.confirm(values.code)) ? values : false),
    },
  )
  return (
    <main className="mx-auto flex min-h-screen max-w-lg flex-col justify-center gap-5 p-8">
      <h1 className="text-2xl font-semibold">
        {state.context ? `Join ${state.context.organizationName}` : 'Organization invitation'}
      </h1>
      {state.error ? <p role="alert">{state.error}</p> : null}
      {!state.context && !state.loading && controller.reload ? <Button onClick={controller.reload}>Retry</Button> : null}
      {state.context && !state.loading ? (
        <>
          <TextField aria-label="Invitation recipient" variant="constant" value={state.context.email} />
          <p className="text-sm text-muted-foreground">
            Expires {new Date(state.context.expiresAt * 1000).toLocaleString()}. Continue to verify this email and{' '}
            {state.context.signup ? 'create your account' : 'sign in'}.
          </p>
          {state.step === 'continue' ? (
            <Button aria-busy={state.busy} onClick={() => void controller.continue()}>
              Continue
            </Button>
          ) : state.step === 'confirmed' ? (
            <Button onClick={() => void controller.confirm('')}>Enter organization</Button>
          ) : (
            <>
              <Form.Root model={form}>
                <CodeEntry name="code" aria-label="Email verification code" />
                <Form.Error name="code" />
                <Button type="submit" aria-busy={state.busy}>
                  Accept invitation
                </Button>
              </Form.Root>
              <Button onClick={() => void controller.continue()}>Resend verification code</Button>
            </>
          )}
        </>
      ) : null}
    </main>
  )
}
