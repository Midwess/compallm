import { useContext } from 'react'
import { zodResolver } from '@hookform/resolvers/zod'
import { EmailCodeInput } from '@compallm/contract/validation'
import { useStore } from 'zustand'
import type { StoreApi } from 'zustand/vanilla'
import type { InvitationContext } from '@compallm/contract/graphql'
import { InvitationControllerContext } from '../context'
import { Avatar } from '../components/content'
import { Toast } from '../components/feedback'
import { OnboardingPanel } from './onboarding/panel'
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
      command: true,
      resolver: zodResolver(EmailCodeInput.pick({ code: true })),
      submit: async (values) => ((await controller.confirm(values.code)) ? values : false),
    },
  )
  const context = state.context
  const code = state.step === 'code'
  const unavailable = !state.loading && !context
  const title = unavailable ? (
    'This invitation is unavailable'
  ) : code ? (
    context?.signup ? (
      'Let’s us create your new account'
    ) : (
      'Check your email'
    )
  ) : state.step === 'confirmed' ? (
    'You’re ready to join'
  ) : context ? (
    <span className="grid gap-3">
      <span className="text-base font-normal text-muted-foreground">You’re invited by</span>
      <span className="flex w-fit min-w-0 max-w-full items-center gap-2.5">
        <span
          className="line-clamp-2 min-w-0 text-2xl leading-8 font-medium underline
decoration-foreground/25 decoration-1 underline-offset-4"
        >
          {context.organizationName}
        </span>
        <Avatar name={context.organizationName} size={28} />
      </span>
    </span>
  ) : (
    'You’re invited by'
  )
  const description = unavailable ? (
    'Ask your team for a fresh invitation.\nWe’ll keep a place for you.'
  ) : code ? (
    <span>
      We sent a six-digit code to <span className="text-on-overlay">{context?.email}</span>.
    </span>
  ) : null
  return (
    <div
      data-theme="basic-dark"
      className="dark grid min-h-dvh place-items-center bg-background p-4 font-sans text-foreground sm:p-8"
    >
      <Form.Root model={form} className="w-full max-w-3xl">
        <OnboardingPanel
          title={title}
          description={description}
          progress={code ? 0.18 : state.step === 'confirmed' ? 0.24 : 0.08}
          action={
            context && !state.loading ? (
              <Button
                type={code ? 'submit' : 'button'}
                variant="primary"
                size="authentication"
                className="h-12 w-full text-base"
                loading={state.busy}
                onClick={
                  code
                    ? undefined
                    : () => {
                        void (state.step === 'confirmed' ? controller.confirm('') : controller.continue())
                      }
                }
              >
                {code ? 'Accept invitation' : state.step === 'confirmed' ? 'Enter organization' : 'Continue'}
              </Button>
            ) : unavailable && controller.reload ? (
              <Button variant="primary" size="authentication" className="h-12 w-full text-base" onClick={controller.reload}>
                Try again
              </Button>
            ) : (
              <div className="h-12" />
            )
          }
          footer={
            <div className="mt-2 flex h-8 items-center justify-center">
              {code ? (
                <Button
                  variant="ghost"
                  className="text-disabled"
                  loading={state.busy}
                  onClick={() => {
                    void controller.continue()
                  }}
                >
                  Resend verification code
                </Button>
              ) : null}
            </div>
          }
        >
          {context && !state.loading ? (
            code ? (
              <div className="grid gap-3">
                <CodeEntry name="code" aria-label="Email verification code" disabled={state.busy} />
                <Form.Error name="code" />
              </div>
            ) : (
              <div className="grid gap-6">
                <div className="grid gap-2">
                  <label htmlFor="invitation-recipient" className="text-sm text-muted-foreground">
                    Your email
                  </label>
                  <TextField
                    id="invitation-recipient"
                    aria-label="Invitation recipient"
                    variant="constant"
                    value={context.email}
                    className="h-10 text-base"
                  />
                </div>
              </div>
            )
          ) : null}
        </OnboardingPanel>
      </Form.Root>
      {state.error && context ? (
        <div className="fixed right-6 bottom-6 z-50 max-w-[calc(100vw-3rem)]">
          <Toast tone="error" message={state.error} onDismiss={() => controller.store.setState({ error: null })} />
        </div>
      ) : null}
    </div>
  )
}
