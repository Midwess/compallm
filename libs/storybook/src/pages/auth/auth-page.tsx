import { zodResolver } from '@hookform/resolvers/zod'
import { FormError } from '../../components/form'
import { useEffect, useContext, useId, useRef, type RefObject } from 'react'
import { CheckCircleIcon } from '@phosphor-icons/react/dist/csr/CheckCircle'
import { Button, AccountPrompt, TextLink } from '../../components/buttons'
import { EmailField, CodeEntry } from '../../components/fields'
import { FormFieldLabel } from '../../components/forms'
import { Toast, FeedbackSlot } from '../../components/feedback'
import { pagePaths } from '../paths'
import { AuthLayout } from './layout'
import { useFormStore } from '../../components/form'
import { AuthControllerContext } from '../../context'
import type { AuthState } from './types'
import { Form, useManagedForm, contractResult } from '../../components/form'
import { emailRequestSchema, otpRequestSchema } from '@compallm/contract/auth'

type Mode = 'signup' | 'signin'

function useAuthController() {
  const controller = useContext(AuthControllerContext)
  if (!controller) throw new Error('Authentication UI requires an AuthControllerContext value.')
  return controller
}

function useAuthState<T>(selector: (state: AuthState) => T, names: readonly string[] = []): T {
  return useFormStore(useAuthController().store, selector, names)
}

const errors = {
  'invalid-email': 'Enter a valid email address.',
  'email-registered': 'Email already registered. Sign in instead.',
  'incorrect-code': 'Incorrect code. Check it and try again.',
  'expired-code': 'This code has expired. Request a new code.',
  throttled: 'Too many requests. Please wait before trying again.',
  unavailable: 'Something went wrong. Please try again.',
}

export function AuthenticationPage({ mode }: { mode: Mode }) {
  return (
    <AuthLayout centered={mode === 'signin'}>
      <AuthContent mode={mode} />
    </AuthLayout>
  )
}

function AuthContent({ mode }: { mode: Mode }) {
  const step = useAuthState((state) => state.step)
  if (step === 'complete') return <Completion mode={mode} />
  if (mode === 'signin' && (step === 'choose' || step === 'passkey')) return <PasskeyLogin />
  return (
    <div data-route={pagePaths[mode]} className={mode === 'signin' ? 'flex flex-col gap-6 text-center' : 'flex flex-col gap-6'}>
      <h1 className={mode === 'signin' ? 'm-0 text-lg font-medium' : 'm-0 text-ui-28 leading-10 font-medium tracking-[-0.5px]'}>
        {step === 'email'
          ? mode === 'signup'
            ? 'Create your account.'
            : 'Log in to Compallm'
          : mode === 'signup'
            ? 'Verify your email.'
            : 'Check your email.'}
      </h1>
      {step === 'email' ? <EmailForm mode={mode} /> : <VerificationForm />}
    </div>
  )
}

function Completion({ mode }: { mode: Mode }) {
  return (
    <div className="grid gap-4" role="status">
      <CheckCircleIcon size={40} className="text-success" aria-hidden="true" />
      <h1 className="m-0 text-ui-28 leading-10 font-medium tracking-[-0.5px]">
        {mode === 'signup' ? 'Your account is ready.' : 'You’re signed in.'}
      </h1>
      <p className="m-0 text-ui-15 leading-6 text-muted-foreground">You can now continue to your workspace.</p>
    </div>
  )
}

function EmailForm({ mode }: { mode: Mode }) {
  const controller = useAuthController()
  const input = useRef<HTMLInputElement>(null)
  const id = useId()
  const model = useManagedForm(
    { email: controller.store.getState().email },
    {
      mode: 'button',
      command: true,
      changed: () => controller.clearError(),
      resolver: zodResolver(emailRequestSchema(mode)),
      submit: async (values) => {
        await controller.requestCode(values.email)
        return controller.store.getState().step === 'code' ? { email: controller.store.getState().email } : false
      },
    },
  )
  return (
    <Form.Root
      model={model}
      className="flex flex-col gap-4 text-left"
      noValidate
      aria-label={mode === 'signup' ? 'Create account' : 'Sign in'}
    >
      <div className="grid gap-1">
        <FormFieldLabel htmlFor={`${id}-email`}>Email address</FormFieldLabel>
        <EmailInput id={`${id}-email`} errorId={`${id}-error`} inputRef={input} />
        <FormError name="email" />
      </div>
      <ContinueButton />
      <FeedbackSlot className="h-12 shrink-0">
        <ErrorMessage />
      </FeedbackSlot>
      {mode === 'signin' ? (
        <Button variant="link" onClick={() => controller.backToMethods?.()}>
          Back to log in
        </Button>
      ) : null}
      <AccountLink mode={mode} />
    </Form.Root>
  )
}

export function EmailInput({ id, inputRef }: { id: string; errorId: string; inputRef: RefObject<HTMLInputElement | null> }) {
  const disabled = useAuthState((state) => state.pending !== null)
  const invalid = useAuthState((state) => state.error !== null || state.serverError !== null)
  return (
    <EmailField
      type="text"
      inputMode="email"
      ref={inputRef}
      id={id}
      name="email"
      required
      placeholder="you@example.com"
      disabled={disabled}
      aria-invalid={invalid || undefined}
      aria-describedby="email-error"
    />
  )
}

export function ContinueButton() {
  const pending = useAuthState((state) => state.pending === 'request')
  const remaining = useAuthState((state) => state.remaining)
  return (
    <Button className="w-full" variant="primary" size="authentication" type="submit" loading={pending} disabled={remaining > 0}>
      {pending ? 'Continuing…' : remaining > 0 ? `Try again in ${remaining}s` : 'Continue'}
    </Button>
  )
}

function AccountLink({ mode }: { mode: Mode }) {
  const controller = useAuthController()
  const target = mode === 'signup' ? 'signin' : 'signup'
  if (mode === 'signin')
    return (
      <p className="m-0 flex items-center justify-center gap-1 text-xsm leading-4 text-muted-foreground">
        Don’t have an account?
        <TextLink
          className="font-normal text-foreground"
          href={controller.links?.signup ?? pagePaths.signup}
          onClick={(event) => {
            if (
              controller.navigate &&
              !event.metaKey &&
              !event.ctrlKey &&
              !event.shiftKey &&
              !event.altKey &&
              event.button === 0
            ) {
              event.preventDefault()
              controller.navigate(controller.links?.signup ?? pagePaths.signup)
            }
          }}
        >
          Sign up
        </TextLink>
      </p>
    )
  return (
    <AccountPrompt
      className="mt-2 min-h-11"
      action={
        <TextLink
          className="inline-flex min-h-11 items-center"
          href={controller.links?.[target] ?? pagePaths[target]}
          onClick={(event) => {
            if (
              !controller.navigate ||
              event.defaultPrevented ||
              event.button !== 0 ||
              event.metaKey ||
              event.ctrlKey ||
              event.shiftKey ||
              event.altKey ||
              event.currentTarget.target === '_blank'
            )
              return
            event.preventDefault()
            controller.navigate(event.currentTarget.getAttribute('href')!)
          }}
        >
          {mode === 'signup' ? 'Sign in' : 'Sign up'}
        </TextLink>
      }
    >
      {mode === 'signup' ? 'Already have an account?' : 'New to Compallm?'}
    </AccountPrompt>
  )
}

function ErrorMessage() {
  const controller = useAuthController()
  const error = useAuthState((state) => state.error)
  const serverError = useAuthState((state) => state.serverError)
  return serverError || error ? (
    <div className="fixed right-6 bottom-6 z-50 max-w-[calc(100vw-3rem)]">
      <Toast tone="error" message={serverError ?? errors[error!]} onDismiss={controller.clearError} />
    </div>
  ) : null
}

function VerificationForm() {
  const controller = useAuthController()
  const model = controller.store.form
  useEffect(
    () =>
      model.configure({
        mode: 'complete',
        command: true,
        changed: () => controller.clearError(),
        parse: (values) => contractResult(otpRequestSchema('signin').safeParse({ code: values.code }), values),
        fields: {
          code: { mode: 'complete', complete: (value) => otpRequestSchema('signin').safeParse({ code: value }).success },
        },
        submit: async (values) => {
          await controller.verifyCode(values.code ?? '')
          return controller.store.getState().step === 'complete' ? {} : false
        },
      }),
    [controller, model],
  )
  return (
    <Form.Root model={model}>
      <Recipient />
      <CodeInput />
      <VerificationFeedback />
      <div className="mt-6 flex w-full flex-col items-start gap-1">
        <ResendControl />
        <BackAction />
      </div>
    </Form.Root>
  )
}

function Recipient() {
  const email = useAuthState((state) => state.email)
  return (
    <p className="mb-2 text-sm text-muted-foreground">
      Enter the code sent to <strong className="font-medium break-all text-foreground">{email}</strong>
    </p>
  )
}

export function CodeInput() {
  const disabled = useAuthState((state) => state.pending !== null)
  const invalid = useAuthState((state) => state.error !== null || state.serverError !== null)
  return (
    <>
      <CodeEntry
        aria-label="Email code"
        name="code"
        autoFocus
        disabled={disabled}
        aria-invalid={invalid || undefined}
        aria-describedby="code-error"
      />
      <FormError name="code" />
    </>
  )
}

function VerificationFeedback() {
  const controller = useAuthController()
  const checking = useAuthState((state) => state.pending === 'verify')
  const retryable = useAuthState((state) => state.error === 'unavailable' || state.serverError !== null)
  return (
    <FeedbackSlot className="mt-2 h-20 text-sm leading-5" aria-live="polite">
      {checking ? <p role="status">Checking code…</p> : null}
      <ErrorMessage />
      {retryable ? (
        <Button variant="link" onClick={() => void controller.retry()}>
          Try again
        </Button>
      ) : null}
    </FeedbackSlot>
  )
}

export function ResendControl() {
  const controller = useAuthController()
  const remaining = useAuthState((state) => state.remaining)
  const pending = useAuthState((state) => state.pending)
  return (
    <div className="flex flex-wrap items-center gap-1 text-xsm text-muted-foreground">
      <span>Didn’t receive a code?</span>
      <Button
        className="h-6"
        variant="link"
        onClick={() => void controller.resendCode()}
        loading={pending === 'resend'}
        disabled={pending !== null || remaining > 0}
      >
        {pending === 'resend' ? 'Requesting code…' : remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}
      </Button>
    </div>
  )
}

function BackAction() {
  const controller = useAuthController()
  const disabled = useAuthState((state) => state.pending !== null)
  return (
    <Button
      className="h-6 text-muted-foreground"
      variant="link"
      onClick={() => controller.useDifferentEmail()}
      disabled={disabled}
    >
      Use a different email
    </Button>
  )
}

function PasskeyLogin() {
  const controller = useAuthController()
  const pending = useAuthState((s) => s.pending)
  const error = useAuthState((s) => s.serverError)
  return (
    <div className="text-center">
      <h1 className="m-0 mb-6 text-lg font-medium leading-5.5">Log in to Compallm</h1>
      <div className="grid gap-4">
        <Button
          variant="neutral"
          size="standard"
          className="h-11 w-full rounded-lg text-xsm"
          onClick={() => void controller.useEmailCode?.()}
        >
          Continue with email
        </Button>
        <Button
          variant="neutral"
          size="standard"
          className="h-11 w-full rounded-lg text-xsm disabled:opacity-100"
          loading={pending === 'passkey'}
          onClick={() => void controller.verifyPasskey?.()}
        >
          {pending === 'passkey' ? 'Waiting for passkey' : 'Log in with passkey'}
        </Button>
      </div>
      {error ? (
        <p role="alert" className="mt-4 text-left text-xs leading-4.5 text-destructive">
          {error}
        </p>
      ) : null}
      <div className="mt-8">
        <AccountLink mode="signin" />
      </div>
    </div>
  )
}
