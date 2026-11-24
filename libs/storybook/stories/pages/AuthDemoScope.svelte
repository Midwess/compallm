<script lang="ts">
  import { onMount, untrack } from 'svelte'
  import { setAuthController } from '../../src/context'
  import { DemoAuthController, type AuthDemoOptions } from './demo-auth-controller'
  import AccessLoginPage from '../../src/pages/auth/AccessLoginPage.svelte'
  import AccessVerificationDialog from '../../src/pages/auth/AccessVerificationDialog.svelte'
  import Button from '../../src/components/Button.svelte'
  import BookOpenIcon from 'phosphor-svelte/lib/BookOpenIcon'
  import FingerprintIcon from 'phosphor-svelte/lib/FingerprintIcon'
  let { popup = false, ...options }: AuthDemoOptions & { popup?: boolean } = $props()
  const controller = untrack(() => new DemoAuthController(options))
  setAuthController(controller)
  onMount(controller.start)
  let open = $state(true)
</script>

{#if popup}<main class="min-h-svh bg-background p-6 text-foreground sm:p-12">
    <div class="mx-auto max-w-200">
      <div class="mb-14 flex items-center gap-2 text-sm font-medium"><BookOpenIcon size={20} aria-hidden="true" />Compallm</div>
      <div class="mb-8 text-xs text-muted-foreground">Account settings <span class="mx-2">/</span> Security &amp; access</div>
      <h1 class="text-2xl font-medium tracking-tight">Security &amp; access</h1>
      <p class="mt-2 text-sm text-muted-foreground">Choose how you access your account.</p>
      <section class="mt-8 rounded-xl border border-border bg-inset p-5">
        <div class="flex flex-wrap items-center justify-between gap-4">
          <div class="flex items-center gap-3">
            <FingerprintIcon size={22} aria-hidden="true" />
            <div>
              <h2 class="text-sm font-medium">Passkeys</h2>
              <p class="mt-1 text-xs text-muted-foreground">Sign in with your fingerprint, face, or device.</p>
            </div>
          </div>
          <Button
            variant="neutral"
            onclick={(event) => {
              event.currentTarget.focus({ preventScroll: true })
              controller.backToMethods()
              open = true
            }}
          >
            New passkey
          </Button>
        </div>
      </section>
    </div>
    <AccessVerificationDialog
      bind:open
      accountEmail="owner@compallm.com"
      onOpenChange={(next) => {
        if (!next) controller.cancelVerification()
      }}
    />
  </main>
{:else}<AccessLoginPage mode={options.mode === 'signup' ? 'signup' : 'login'} />{/if}
