<script lang="ts">
  import { Button } from '../../components/buttons'
  import { OnboardingPanel } from './panel'
  import { untrack } from 'svelte'
  import type { Content } from '../../components/shared'
  import { readStore } from '../page-state.svelte'
  import { getOnboardingController } from '../../context'
  import ContentValue from '../../components/Content.svelte'

  let {
    title,
    description,
    progress,
    children,
    action,
  }: {
    title: string
    description: string
    progress: number
    children: Content
    action: Content
  } = $props()
  const __onboarding = getOnboardingController()
  const controller = untrack(() => getOnboardingController())
  let step = $derived(readStore(__onboarding.store, (s) => s.step))
  let embedded = $derived(readStore(__onboarding.store, (s) => s.embedded))
  let artworkColor = $derived(readStore(__onboarding.store, (s) => s.artworkColor))
  let pending = $derived(readStore(__onboarding.store, (s) => s.pending))
  let canBack = $derived(['profile', 'workspace', 'work', 'preferences', 'introduction'].includes(step))
</script>

<div class={embedded ? 'grid place-items-center' : 'grid min-h-dvh place-items-center bg-background p-4 sm:p-8'}>
  <OnboardingPanel {title} {description} {progress} {artworkColor} {action}>
    {#snippet footer()}<div
        aria-hidden={!canBack}
        inert={!canBack}
        class={[
          'mt-2 flex h-8 items-center',
          step === 'work' ? 'justify-between' : 'justify-center',
          canBack ? '' : 'invisible',
        ].join(' ')}
      >
        <Button variant="ghost" class="text-disabled" onclick={controller.back} disabled={pending}>
          Back
        </Button>{#if step === 'work'}<Button
            variant="ghost"
            class="text-disabled"
            onclick={controller.skip}
            disabled={pending}
          >
            I don’t want to share
          </Button>{:else}{/if}
      </div>{/snippet}<ContentValue value={children} />
  </OnboardingPanel>
</div>
