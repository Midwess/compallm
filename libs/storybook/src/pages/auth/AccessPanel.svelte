<script lang="ts">
  import { getAuthController } from '../../context'
  import BookOpenIcon from 'phosphor-svelte/lib/BookOpenIcon'
  import FingerprintIcon from 'phosphor-svelte/lib/FingerprintIcon'
  import EnvelopeSimpleIcon from 'phosphor-svelte/lib/EnvelopeSimpleIcon'
  import ArrowLeftIcon from 'phosphor-svelte/lib/ArrowLeftIcon'
  import CheckIcon from 'phosphor-svelte/lib/CheckIcon'
  import Button from '../../components/Button.svelte'
  import EntranceMotion from '../../components/EntranceMotion.svelte'
  import { Dialog } from '../../components/dialog'
  import EmailField from '../../components/EmailField.svelte'
  import CodeEntry from '../../components/CodeEntry.svelte'
  import { Form } from '../../components/form'
  let {
    intent = 'login',
    accountEmail,
    title,
    description,
  }: {
    intent?: 'login' | 'signup' | 'verify'
    accountEmail?: string
    title?: string
    description?: string
  } = $props()
  const controller = getAuthController()
  const model = controller.form
  let step = $derived(controller.store.getState().step)
  let pending = $derived(controller.store.getState().pending)
  let email = $derived(controller.store.getState().email)
  let last = $derived(controller.store.getState().lastLoginMethod)
  let remaining = $derived(controller.store.getState().remaining)
  let serverError = $derived(controller.store.getState().serverError)
  let failure = $derived(serverError ?? controller.store.getState().error)
  let complete = $derived(step === 'complete')
  let code = $derived(step === 'code')
  let verifying = $derived(intent === 'verify')
  let enteringEmail = $derived(step === 'email' && !verifying)
  let signup = $derived(intent === 'signup')
  let heading = $derived(
    complete
      ? verifying
        ? 'You’re verified'
        : signup
          ? 'Your account is ready'
          : 'Welcome back'
      : code
        ? 'Check your email'
        : (title ?? (verifying ? 'Verify it’s you' : signup ? 'Create your account' : 'Log in to Compallm')),
  )
  let message = $derived(
    complete
      ? verifying
        ? 'You can now continue with your change.'
        : 'Your workspace is ready for you.'
      : code
        ? 'Enter the 6-digit code we sent to your email.'
        : (description ??
          (verifying
            ? 'Confirm your identity to make changes to your passkeys.'
            : 'Your models, conversations, and ideas. Together.')),
  )
  let methods = $derived(last === 'email' ? (['email', 'passkey'] as const) : (['passkey', 'email'] as const))
  const id = $props.id()
  async function chooseEmail() {
    await controller.useEmailCode()
    if (verifying && controller.store.getState().step !== 'code')
      await controller.requestCode(accountEmail ?? controller.store.getState().email)
  }
  function navigate(event: MouseEvent & { currentTarget: EventTarget & HTMLAnchorElement }) {
    if (
      !controller.navigate ||
      event.defaultPrevented ||
      event.button !== 0 ||
      event.metaKey ||
      event.ctrlKey ||
      event.shiftKey ||
      event.altKey
    )
      return
    event.preventDefault()
    controller.navigate(event.currentTarget.getAttribute('href')!)
  }
</script>

<div class="w-full text-center">
  {#if !verifying}<div
      class="mx-auto mb-6 flex size-12 items-center justify-center rounded-2xl border border-border bg-surface text-foreground"
    >
      {#if complete}<CheckIcon size={24} aria-hidden="true" />{:else}<BookOpenIcon size={24} aria-hidden="true" />{/if}
    </div>{/if}
  {#key heading}<EntranceMotion subtle>
      {#if verifying}<Dialog.Title class="mb-4 text-sm font-medium">Verify your identity</Dialog.Title>{:else}<h1
          class="m-0 text-2xl font-medium tracking-tight"
        >
          {heading}
        </h1>{/if}
    </EntranceMotion>{/key}
  {#if !verifying}{#key message}<EntranceMotion subtle order={1}>
        <p class="mt-3 mb-0 text-sm leading-6 text-muted-foreground">{message}</p>
      </EntranceMotion>{/key}{/if}
  {#if !verifying && !complete && (accountEmail || code)}<div
      class="mt-5 inline-flex max-w-full items-center gap-2 rounded-full border border-border bg-surface px-3 py-1.5 text-xs"
    >
      <span class="flex size-5 shrink-0 items-center justify-center rounded-full bg-primary/15 text-primary">
        <EnvelopeSimpleIcon size={12} aria-hidden="true" />
      </span><span class="truncate">{accountEmail ?? email}</span>
    </div>{/if}
  {#if !complete}<div class={verifying ? '' : 'mt-7'}>
      {#if enteringEmail || code}<Form.Root
          {model}
          class={verifying ? 'grid min-w-0 grid-cols-1 gap-3 text-left' : 'grid min-w-0 grid-cols-1 gap-4 text-left'}
        >
          {#if code}<div class="grid w-full min-w-0 grid-cols-1 justify-items-center gap-2">
              <CodeEntry name="code" aria-label="Email verification code" disabled={pending !== null} /><Form.Error
                name="code"
              />
            </div>
          {:else}<div class="grid gap-2">
              <label for={id} class="text-xs font-medium">Email address</label><EmailField
                {id}
                name="email"
                placeholder="you@example.com"
                readonly={verifying}
                disabled={pending !== null}
              /><Form.Error name="email" />
            </div>{/if}
          <EntranceMotion order={0} class="text-center">
            <Button
              size={verifying ? 'compact' : 'standard'}
              class={verifying ? 'h-8 w-full px-3' : 'h-12 w-fit rounded-full px-6 text-sm'}
              type="submit"
              loading={pending !== null}
            >
              {code ? (pending === 'verify' ? 'Verifying…' : 'Verify email') : 'Continue with email'}
            </Button>
          </EntranceMotion>
          {#if code}<EntranceMotion order={1} class="flex items-center justify-center gap-1 text-xs text-muted-foreground">
              Didn’t get a code?<Button
                variant="link"
                disabled={remaining > 0 || pending !== null}
                onclick={() => void controller.resendCode()}
              >
                {remaining > 0 ? `Resend in ${remaining}s` : 'Resend code'}
              </Button>
            </EntranceMotion>{/if}
          {#if !signup || code}<EntranceMotion order={code ? 2 : 1} class="text-center">
              <Button
                variant="link"
                class="mx-auto text-xs text-muted-foreground"
                disabled={pending === 'request' || pending === 'verify' || pending === 'resend'}
                onclick={signup ? controller.useDifferentEmail : controller.backToMethods}
              >
                <ArrowLeftIcon size={14} aria-hidden="true" />
                {signup ? 'Use a different email' : 'Other login methods'}
              </Button>
            </EntranceMotion>{/if}
        </Form.Root>
      {:else}<div class={verifying ? 'grid w-full gap-2' : 'mx-auto grid w-max max-w-full gap-3'}>
          {#each methods as method, index (method)}<EntranceMotion order={index} subtle={verifying} class="grid min-w-0">
              <Button
                size={verifying ? 'compact' : 'standard'}
                variant={index === 0 ? 'primary' : 'neutral'}
                class={verifying
                  ? 'h-8 w-full justify-center gap-2 px-3'
                  : 'relative h-12 w-full justify-center gap-3 rounded-full px-6 text-sm'}
                loading={method === 'passkey' ? pending === 'passkey' : pending === 'request' || pending === 'resend'}
                onclick={() => (method === 'passkey' ? void controller.verifyPasskey() : void chooseEmail())}
              >
                {#if method === 'passkey'}<FingerprintIcon
                    size={verifying ? 16 : 20}
                    aria-hidden="true"
                  />{:else}<EnvelopeSimpleIcon size={verifying ? 16 : 20} aria-hidden="true" />{/if}
                <span>
                  {method === 'passkey'
                    ? pending === 'passkey'
                      ? 'Waiting for passkey…'
                      : 'Continue with passkey'
                    : 'Continue with email'}
                </span>
              </Button>
              {#if last === method}<p
                  class={verifying ? 'mt-1 mb-2 text-xs leading-4 text-disabled' : 'mt-2 mb-2 text-xs leading-5 text-disabled'}
                >
                  You used {method === 'passkey' ? 'passkey' : 'email'} last time
                </p>{/if}
            </EntranceMotion>{/each}
          {#if pending === 'passkey'}<p role="status" class="mt-2 mb-0 text-xs leading-5 text-muted-foreground">
              Complete the prompt on your device.<br />You can use email instead at any time.
            </p>{/if}
        </div>{/if}
      {#if failure}<p role="alert" class="mt-4 mb-0 text-center text-xs leading-5 text-destructive">
          {serverError ??
            (failure === 'incorrect-code'
              ? 'Incorrect code. Check it and try again.'
              : failure === 'expired-code'
                ? 'This code has expired. Request a new code.'
                : failure === 'throttled'
                  ? 'Please wait before requesting another code.'
                  : 'Verification was unsuccessful. Please try again.')}
        </p>{/if}
    </div>{/if}
  {#if !verifying}<div class="mt-7 pt-5 text-xs leading-5 text-muted-foreground">
      <span>
        {signup ? 'Already have an account?' : 'New to Compallm?'}
        <a
          class="ml-1 text-foreground hover:underline"
          href={signup ? (controller.links?.signin ?? '/login') : (controller.links?.signup ?? '/signup')}
          onclick={navigate}
        >
          {signup ? 'Log in' : 'Create an account'}
        </a>
      </span>
    </div>{/if}
</div>
