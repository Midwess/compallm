import { useContext, useId, type ReactNode } from 'react'
import { BookOpenIcon } from '@phosphor-icons/react/dist/csr/BookOpen'
import { FingerprintIcon } from '@phosphor-icons/react/dist/csr/Fingerprint'
import { EnvelopeSimpleIcon } from '@phosphor-icons/react/dist/csr/EnvelopeSimple'
import { ArrowLeftIcon } from '@phosphor-icons/react/dist/csr/ArrowLeft'
import { CheckIcon } from '@phosphor-icons/react/dist/csr/Check'
import { AuthControllerContext } from '../../context'
import { Button } from '../../components/buttons'
import { LoginShader } from './login-shader'
import { EntranceMotion } from '../../components/motion'
import { Dialog } from '../../components/dialog'
import { EmailField, CodeEntry } from '../../components/fields'
import { Form, useFormStore, useManagedForm, contractResult } from '../../components/form'
import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'

export interface AccessPanelProps {
  intent?: 'login' | 'signup' | 'verify'
  accountEmail?: string
  title?: string
  description?: string
}

export function AccessPanel({ intent = 'login', accountEmail, title, description }: AccessPanelProps) {
  const controller = useContext(AuthControllerContext)
  if (!controller) throw new Error('AccessPanel requires AuthControllerContext.')
  const step = useFormStore(controller.store, (s) => s.step)
  const pending = useFormStore(controller.store, (s) => s.pending)
  const email = useFormStore(controller.store, (s) => s.email)
  const last = useFormStore(controller.store, (s) => s.lastLoginMethod)
  const remaining = useFormStore(controller.store, (s) => s.remaining)
  const failure = useFormStore(controller.store, (s) => s.serverError ?? s.error)
  const id = useId()
  const complete = step === 'complete'
  const code = step === 'code'
  const verifying = intent === 'verify'
  const enteringEmail = step === 'email' && !verifying
  const signup = intent === 'signup'
  const mode = signup ? 'signup' : 'signin'
  const heading = complete
    ? verifying
      ? 'You’re verified'
      : signup
        ? 'Your account is ready'
        : 'Welcome back'
    : code
      ? 'Check your email'
      : (title ?? (verifying ? 'Verify it’s you' : signup ? 'Create your account' : 'Log in to Compallm'))
  const message = complete
    ? verifying
      ? 'You can now continue with your change.'
      : 'Your workspace is ready for you.'
    : code
      ? 'Enter the 6-digit code we sent to your email.'
      : (description ??
        (verifying
          ? 'Confirm your identity to make changes to your passkeys.'
          : 'Your models, conversations, and ideas. Together.'))
  const model = useManagedForm(
    { email: accountEmail ?? email, code: '' },
    {
      mode: 'button',
      command: true,
      changed: controller.clearError,
      parse: (values) =>
        code
          ? contractResult(otpRequestSchema(mode).safeParse({ code: values.code }), values)
          : contractResult(emailRequestSchema(mode).safeParse({ email: values.email }), values),
      submit: async (values) => {
        if (code) await controller.verifyCode(values.code)
        else await controller.requestCode(values.email)
        return false as const
      },
    },
  )
  const chooseEmail = async () => {
    await controller.useEmailCode?.()
    if (verifying && controller.store.getState().step !== 'code')
      await controller.requestCode(accountEmail ?? controller.store.getState().email)
  }
  const methods = last === 'email' ? (['email', 'passkey'] as const) : (['passkey', 'email'] as const)
  const errorMessage = failure ? (
    <p role="alert" className="m-0 text-center text-xs leading-5 text-destructive">
      {controller.store.getState().serverError ??
        (failure === 'incorrect-code'
          ? 'Incorrect code. Check it and try again.'
          : failure === 'expired-code'
            ? 'This code has expired. Request a new code.'
            : failure === 'throttled'
              ? 'Please wait before requesting another code.'
              : 'Verification was unsuccessful. Please try again.')}
    </p>
  ) : null
  return (
    <div className="w-full text-center">
      {!verifying ? (
        <div
          className="mx-auto mb-6 flex size-12 items-center justify-center
          rounded-2xl border border-border bg-surface text-foreground"
        >
          {complete ? <CheckIcon size={24} aria-hidden="true" /> : <BookOpenIcon size={24} aria-hidden="true" />}
        </div>
      ) : null}
      <div className={verifying ? undefined : 'grid gap-2'}>
        <EntranceMotion key={heading} subtle>
          {verifying ? (
            <Dialog.Title className="mb-4 text-sm font-medium">Verify your identity</Dialog.Title>
          ) : (
            <h1 className="m-0 text-2xl font-medium tracking-tight">{heading}</h1>
          )}
        </EntranceMotion>
        {!verifying ? (
          <EntranceMotion key={message} subtle order={1}>
            <p className="m-0 text-sm leading-6 text-muted-foreground">{message}</p>
          </EntranceMotion>
        ) : null}
      </div>
      {!verifying && !complete && (accountEmail || code) ? (
        <div className="mt-2 flex max-w-full items-center justify-center gap-2 text-xs text-muted-foreground">
          <span className="flex size-5 shrink-0 items-center justify-center">
            <EnvelopeSimpleIcon size={12} aria-hidden="true" />
          </span>
          <span className="min-w-0 break-all">{accountEmail ?? email}</span>
        </div>
      ) : null}
      {!complete ? (
        <div className={verifying ? '' : 'pt-6'}>
          {enteringEmail || code ? (
            <Form.Root
              model={model}
              className={verifying ? 'grid min-w-0 grid-cols-1 gap-3 text-left' : 'grid min-w-0 grid-cols-1 gap-4 text-left'}
              noValidate
            >
              {code ? (
                <div className="grid w-full min-w-0 grid-cols-1 justify-items-center gap-2">
                  <CodeEntry name="code" aria-label="Email verification code" disabled={pending !== null} />
                  <Form.Error name="code" />
                </div>
              ) : (
                <div className="grid gap-2">
                  <label htmlFor={id} className="text-xs font-medium">
                    Email address
                  </label>
                  <EmailField id={id} name="email" placeholder="you@example.com" readOnly={verifying} />
                  <Form.Error name="email" />
                </div>
              )}
              <div className="grid gap-2">
                <EntranceMotion order={0} className="text-center">
                  <Button
                    size="compact"
                    className={code ? 'h-8 w-fit rounded-full px-4' : 'h-8 w-full rounded-full px-3'}
                    type="submit"
                    loading={pending !== null}
                  >
                    {code ? (pending === 'verify' ? 'Verifying…' : 'Verify email') : 'Continue with email'}
                  </Button>
                </EntranceMotion>
                {errorMessage}
              </div>
              {code ? (
                <EntranceMotion
                  order={1}
                  className="flex flex-wrap items-center justify-center gap-1 text-xs text-muted-foreground"
                >
                  Didn’t get a code?
                  <Button
                    variant="link"
                    disabled={remaining > 0 || pending !== null}
                    onClick={() => void controller.resendCode()}
                  >
                    {remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}
                  </Button>
                </EntranceMotion>
              ) : null}
              {!signup || code ? (
                <EntranceMotion order={code ? 2 : 1} className="text-center">
                  <Button
                    variant="link"
                    className="mx-auto text-xs text-muted-foreground"
                    onClick={signup ? controller.useDifferentEmail : controller.backToMethods}
                  >
                    <ArrowLeftIcon size={14} aria-hidden="true" /> {signup ? 'Use a different email' : 'Other login methods'}
                  </Button>
                </EntranceMotion>
              ) : null}
            </Form.Root>
          ) : (
            <div className={verifying ? 'grid w-full gap-2' : 'mx-auto grid w-max max-w-full gap-4'}>
              {methods.map((method, index) => (
                <EntranceMotion key={method} order={index} subtle={verifying} className="grid min-w-0">
                  <Button
                    size={verifying ? 'compact' : 'standard'}
                    variant={index === 0 ? 'primary' : 'neutral'}
                    className={
                      verifying
                        ? 'h-8 w-full justify-center gap-2 px-3'
                        : 'relative h-12 w-full justify-center gap-3 rounded-full px-6 text-sm'
                    }
                    loading={method === 'passkey' ? pending === 'passkey' : pending === 'request' || pending === 'resend'}
                    onClick={() => (method === 'passkey' ? void controller.verifyPasskey?.() : void chooseEmail())}
                  >
                    {method === 'passkey' ? (
                      <FingerprintIcon size={verifying ? 16 : 20} aria-hidden="true" />
                    ) : (
                      <EnvelopeSimpleIcon size={verifying ? 16 : 20} aria-hidden="true" />
                    )}
                    <span>
                      {method === 'passkey'
                        ? pending === 'passkey'
                          ? 'Waiting for passkey…'
                          : 'Continue with passkey'
                        : 'Continue with email'}
                    </span>
                  </Button>
                  {last === method ? (
                    <p
                      className={
                        verifying
                          ? 'mt-1 mb-2 text-xs leading-4 text-disabled'
                          : 'm-0 pt-2 pb-2 text-xs leading-5 text-disabled'
                      }
                    >
                      You used {method === 'passkey' ? 'passkey' : 'email'} last time
                    </p>
                  ) : null}
                </EntranceMotion>
              ))}
              {errorMessage}
              {pending === 'passkey' ? (
                <p role="status" className="mt-2 mb-0 text-xs leading-5 text-muted-foreground">
                  Complete the prompt on your device.
                  <br />
                  You can use email instead at any time.
                </p>
              ) : null}
            </div>
          )}
        </div>
      ) : null}
      {!verifying ? (
        <div className="pt-6 text-xs leading-5 text-muted-foreground">
          <span>
            {signup ? 'Already have an account?' : 'New to Compallm?'}{' '}
            <a
              className="ml-1 text-foreground hover:underline"
              href={signup ? (controller.links?.signin ?? '/login') : (controller.links?.signup ?? '/signup')}
            >
              {signup ? 'Log in' : 'Create an account'}
            </a>
          </span>
        </div>
      ) : null}
    </div>
  )
}

export function AccessLayout({ children }: { children: ReactNode }) {
  return (
    <main className="relative isolate flex min-h-svh flex-col overflow-hidden bg-background px-6 font-sans text-foreground">
      <div className="pointer-events-none absolute inset-0 -z-10 bg-radial from-primary/10 to-background">
        <LoginShader />
        <div className="absolute inset-0 bg-linear-to-b from-background/98 from-30% via-background/70 via-60% to-transparent" />
      </div>
      <section aria-label="Account access" className="m-auto w-full max-w-88 py-12">
        {children}
      </section>
      <p className="pb-6 text-center text-xs text-muted-foreground">A little space for your next big idea.</p>
    </main>
  )
}

export function AccessLoginPage({ mode = 'login' }: { mode?: 'login' | 'signup' }) {
  return (
    <AccessLayout>
      <AccessPanel intent={mode} />
    </AccessLayout>
  )
}

export function AuthVerificationHost() {
  const controller = useContext(AuthControllerContext)
  if (!controller) throw new Error('AuthVerificationHost requires AuthControllerContext.')
  const active = useFormStore(controller.store, (state) => state.verificationActive)
  return active ? (
    <AccessVerificationDialog
      open
      onOpenChange={(open) => {
        if (!open) controller.cancelVerification?.()
      }}
    />
  ) : null
}

export function AccessVerificationDialog({
  open,
  onOpenChange,
  ...props
}: Omit<AccessPanelProps, 'intent'> & {
  open: boolean
  onOpenChange(open: boolean): void
}) {
  const controller = useContext(AuthControllerContext)
  if (!controller) throw new Error('AccessVerificationDialog requires AuthControllerContext.')
  const complete = useFormStore(controller.store, (state) => state.step === 'complete')
  return (
    <Dialog.Root open={open && !complete} onOpenChange={onOpenChange}>
      <Dialog.Content className="relative w-72 max-w-[calc(100vw-2rem)] rounded-xl bg-inset p-4">
        <AccessPanel {...props} intent="verify" />
        <div className="mt-2 text-center">
          <Dialog.Cancel variant="link" className="h-6 text-xs text-muted-foreground">
            Cancel
          </Dialog.Cancel>
        </div>
      </Dialog.Content>
    </Dialog.Root>
  )
}
