import { useContext, useId, useRef, type FormEvent, type RefObject } from 'react'
import { CheckCircleIcon } from '@phosphor-icons/react/dist/csr/CheckCircle'
import { Button, AccountPrompt, TextLink } from '../../components/buttons'
import { EmailField, CodeEntry } from '../../components/fields'
import { FormFieldLabel } from '../../components/forms'
import { RichMessage } from '../../components/rich-message'
import { InlineError, FeedbackSlot } from '../../components/feedback'
import { pagePaths } from '../paths'
import { AuthLayout } from './layout'
import { useStore } from 'zustand'
import { AuthControllerContext } from '../../context'
import type { AuthState } from './types'
import { emailRequestSchema } from '@compallm/contract/auth'
import type { EmailInput as EmailRequestInput } from '@compallm/contract/graphql'

type Mode = 'signup' | 'signin'

function useAuthController() {
  const controller = useContext(AuthControllerContext)
  if (!controller) throw new Error('Authentication UI requires an AuthControllerContext value.')
  return controller
}

function useAuthState<T>(selector: (state: AuthState) => T): T {
  return useStore(useAuthController().store, selector)
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
    <AuthLayout>
      <AuthContent mode={mode} />
    </AuthLayout>
  )
}

function AuthContent({ mode }: { mode: Mode }) {
  const step = useAuthState((state) => state.step)
  if (step === 'complete') return <Completion mode={mode} />
  return (
    <div data-route={pagePaths[mode]} className="flex flex-col gap-6">
      <h1 className="m-0 text-[28px] leading-10 font-medium tracking-[-0.5px]">
        {step === 'email'
          ? mode === 'signup'
            ? 'Create your account.'
            : 'Welcome back.'
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
      <h1 className="m-0 text-[28px] leading-10 font-medium tracking-[-0.5px]">
        {mode === 'signup' ? 'Your account is ready.' : 'You’re signed in.'}
      </h1>
      <p className="m-0 text-[15px] leading-6 text-muted">You can now continue to your workspace.</p>
    </div>
  )
}

function EmailForm({ mode }: { mode: Mode }) {
  const controller = useAuthController()
  const input = useRef<HTMLInputElement>(null)
  const id = useId()
  function submit(event: FormEvent<HTMLFormElement>) {
    event.preventDefault()
    if (!emailRequestSchema(mode).safeParse({ email: controller.store.getState().email } satisfies EmailRequestInput).success) {
      controller.onInvalidEmail()
      input.current?.focus()
      return
    }
    void controller.requestCode()
  }
  return (
    <form
      className="flex flex-col gap-4"
      onSubmit={submit}
      noValidate
      aria-label={mode === 'signup' ? 'Create account' : 'Sign in'}
    >
      <div className="grid gap-1">
        <FormFieldLabel htmlFor={`${id}-email`}>Email address</FormFieldLabel>
        <EmailInput id={`${id}-email`} errorId={`${id}-error`} inputRef={input} />
      </div>
      <ContinueButton />
      <FeedbackSlot className="h-12 shrink-0">
        <ErrorMessage id={`${id}-error`} />
      </FeedbackSlot>
      <AccountLink mode={mode} />
    </form>
  )
}

export function EmailInput({
  id,
  errorId,
  inputRef,
}: {
  id: string
  errorId: string
  inputRef: RefObject<HTMLInputElement | null>
}) {
  const controller = useAuthController()
  const email = useAuthState((state) => state.email)
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
      value={email}
      onChange={(event) => controller.onEmailChanged(event.target.value)}
      disabled={disabled}
      aria-invalid={invalid || undefined}
      aria-describedby={invalid ? errorId : undefined}
    />
  )
}

export function ContinueButton() {
  const pending = useAuthState((state) => state.pending === 'request')
  const remaining = useAuthState((state) => state.remaining)
  return (
    <Button className="w-full" variant="primary" size="authentication" type="submit" loading={pending} disabled={remaining > 0}>
      {pending ? 'Sending code…' : remaining > 0 ? `Try again in ${remaining}s` : 'Continue'}
    </Button>
  )
}

function AccountLink({ mode }: { mode: Mode }) {
  const controller = useAuthController()
  const target = mode === 'signup' ? 'signin' : 'signup'
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

function ErrorMessage({ id }: { id: string }) {
  const controller = useAuthController()
  const error = useAuthState((state) => state.error)
  const serverError = useAuthState((state) => state.serverError)
  return serverError || error ? (
    <InlineError id={id}>
      <RichMessage message={serverError ?? errors[error!]} navigate={controller.navigate} />
    </InlineError>
  ) : null
}

function VerificationForm() {
  const id = useId()
  return (
    <div>
      <Recipient />
      <CodeInput errorId={`${id}-error`} />
      <VerificationFeedback errorId={`${id}-error`} />
      <div className="mt-6 flex w-full flex-col items-start gap-1">
        <ResendControl />
        <BackAction />
      </div>
    </div>
  )
}

function Recipient() {
  const email = useAuthState((state) => state.email)
  return (
    <p className="mb-2 text-sm text-muted">
      Enter the code sent to <strong className="font-medium break-all text-text">{email}</strong>
    </p>
  )
}

export function CodeInput({ errorId }: { errorId: string }) {
  const controller = useAuthController()
  const code = useAuthState((state) => state.code)
  const disabled = useAuthState((state) => state.pending !== null)
  const invalid = useAuthState((state) => state.error !== null || state.serverError !== null)
  return (
    <CodeEntry
      aria-label="Email code"
      value={code}
      onValueChange={(code) => controller.onCodeChanged(code)}
      autoFocus
      disabled={disabled}
      aria-invalid={invalid || undefined}
      aria-describedby={invalid ? errorId : undefined}
    />
  )
}

function VerificationFeedback({ errorId }: { errorId: string }) {
  const controller = useAuthController()
  const checking = useAuthState((state) => state.pending === 'verify')
  const retryable = useAuthState((state) => state.error === 'unavailable' || state.serverError !== null)
  return (
    <FeedbackSlot className="mt-2 h-20 text-sm leading-5" aria-live="polite">
      {checking ? <p role="status">Checking code…</p> : null}
      <ErrorMessage id={errorId} />
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
    <div className="flex flex-wrap items-center gap-1 text-xsm text-muted">
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
    <Button className="h-6 text-muted" variant="link" onClick={() => controller.useDifferentEmail()} disabled={disabled}>
      Use a different email
    </Button>
  )
}
