<script lang="ts">
  import { untrack } from 'svelte'
  import type { Snippet } from 'svelte'
  import type { OnboardingController } from '../../onboarding/types'
  import { setOnboardingController, getSettingsController } from '../../../context'
  let { value, children }: { value: OnboardingController; children?: Snippet } = $props()
  setOnboardingController(untrack(() => value))
  const form = getSettingsController().pages.onboarding!.store.form
  $effect(() => {
    const welcome = form.get('draft.welcome') as string
    untrack(() => value.store.setState((state) => ({ organization: { ...state.organization, welcome } })))
  })
</script>

{@render children?.()}
