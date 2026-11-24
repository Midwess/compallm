<script lang="ts">
  import { zodResolver } from '../components/form'
  import { LoadingRegion, Skeleton } from '../components/loading'
  import { EmailCodeInput } from '@compallm/contract/validation'

  import { Avatar } from '../components/content'
  import { Toast } from '../components/feedback'
  import { OnboardingPanel } from './onboarding/panel'
  import { Button } from '../components/buttons'
  import { TextField, CodeEntry } from '../components/fields'
  import { Form, managedForm } from '../components/form'
  import { untrack } from 'svelte'

  import { readStore } from './page-state.svelte'
  import { getInvitationController } from '../context'

  const unavailableDescription = 'Ask your team for a fresh invitation.\nWe’ll keep a place for you.'
  const controller = untrack(() => getInvitationController())
  if (!controller) throw new Error('InvitationControllerContext is required.')
  let state = $derived(readStore(controller.store))
  const form = untrack(() =>
    managedForm(
      { code: '' },
      {
        command: true,
        resolver: zodResolver(EmailCodeInput.pick({ code: true })),
        submit: async (values) => ((await controller.confirm(values.code)) ? values : false),
      },
    ),
  )
  let context = $derived(state.context)
  let code = $derived(state.step === 'code')
  let unavailable = $derived(!state.loading && !context)
</script>

{#snippet title()}{#if unavailable}This invitation is unavailable{:else}{#if code}{#if context?.signup}Let’s us create your new
        account{:else}Check your email{/if}{:else}{#if state.step === 'confirmed'}You’re ready to join{:else}{#if context}<span
            class="grid gap-3"
          >
            <span class="text-base font-normal text-muted-foreground">You’re invited by</span><span
              class="flex w-fit min-w-0 max-w-full items-center gap-2.5"
            >
              <span
                class="line-clamp-2 min-w-0 text-2xl leading-8 font-medium underline
decoration-foreground/25 decoration-1 underline-offset-4"
              >
                {context.organizationName}
              </span><Avatar name={context.organizationName} size={28}></Avatar>
            </span>
          </span>{:else}<span class="grid gap-3">
            <span class="text-base font-normal text-muted-foreground">You’re invited by</span><Skeleton loading>
              <span class="block h-8 w-48 rounded-lg"></span>
            </Skeleton>
          </span>{/if}{/if}{/if}{/if}{/snippet}
{#snippet description()}{#if unavailable}{unavailableDescription}{:else}{#if code}<span>
        We sent a six-digit code to <span class="text-on-overlay">{context?.email}</span>.
      </span>{:else}{/if}{/if}{/snippet}
<div data-theme="basic-dark" class="dark grid min-h-dvh place-items-center bg-background p-4 font-sans text-foreground sm:p-8">
  <LoadingRegion
    ready={!!context || !state.loading}
    pending={state.loading}
    label="Loading invitation"
    class="min-w-0 w-full max-w-3xl"
  >
    <Form.Root model={form} class="min-w-0 w-full max-w-3xl">
      <OnboardingPanel {title} {description} progress={code ? 0.18 : state.step === 'confirmed' ? 0.24 : 0.08}>
        {#snippet action()}{#if context && !state.loading}<Button
              type={code ? 'submit' : 'button'}
              variant="primary"
              size="authentication"
              class="h-12 w-full text-base"
              loading={state.busy}
              onclick={code
                ? undefined
                : () => {
                    void (state.step === 'confirmed' ? controller.confirm('') : controller.continue())
                  }}
            >
              {#if code}Accept invitation{:else}{#if state.step === 'confirmed'}Enter organization{:else}Continue{/if}{/if}
            </Button>{:else}{#if unavailable && controller.reload}<Button
                variant="primary"
                size="authentication"
                class="h-12 w-full text-base"
                onclick={controller.reload}
              >
                Try again
              </Button>{:else}<Skeleton>
                <div class="h-12 w-full rounded-xl"></div>
              </Skeleton>{/if}{/if}{/snippet}{#snippet footer()}<div class="mt-2 flex h-8 items-center justify-center">
            {#if code}<Button
                variant="ghost"
                class="text-disabled"
                loading={state.busy}
                onclick={() => {
                  void controller.continue()
                }}
              >
                Resend verification code
              </Button>{:else}{/if}
          </div>{/snippet}{#if context && !state.loading}{#if code}<div class="grid gap-3">
              <CodeEntry name="code" aria-label="Email verification code" disabled={state.busy}></CodeEntry><Form.Error
                name="code"
              ></Form.Error>
            </div>{:else}<div class="grid gap-6">
              <div class="grid gap-2">
                <label for="invitation-recipient" class="text-sm text-muted-foreground">Your email </label><TextField
                  id="invitation-recipient"
                  aria-label="Invitation recipient"
                  variant="constant"
                  value={context.email}
                  class="h-10 text-base"
                ></TextField>
              </div>
            </div>{/if}{:else}{#if state.loading}<div class="grid gap-2">
              <label for="invitation-recipient" class="text-sm text-muted-foreground">Your email </label><TextField
                id="invitation-recipient"
                aria-label="Invitation recipient"
                variant="constant"
                value=""
                class="h-10 text-base"
              ></TextField>
            </div>{:else}{/if}{/if}
      </OnboardingPanel>
    </Form.Root>
  </LoadingRegion>{#if state.error && context}<div class="fixed right-6 bottom-6 z-50 max-w-[calc(100vw-3rem)]">
      <Toast tone="error" message={state.error} onDismiss={() => controller.store.setState({ error: null })}></Toast>
    </div>{:else}{/if}
</div>
