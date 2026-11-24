<script lang="ts">
  import { OnboardingDialog } from '../../../src/pages/onboarding'
  import { Button } from '../../../src/components/buttons'

  import { DemoOnboardingController } from './demo-controller'
  import { untrack } from 'svelte'

  import { setOnboardingController } from '../../../src/context'

  let {
    firstPresentation = false,
    error = null,
  }: {
    firstPresentation?: boolean
    error?: string | null
  } = $props()

  let open = $state(true)
  let controller = $state(untrack(() => new DemoOnboardingController({ embedded: true, step: 'profile', error })))
  setOnboardingController(controller)
</script>

<Button onclick={() => (open = true)}>Open onboarding</Button><OnboardingDialog
  {open}
  onOpenChange={(value) => (open = value)}
  {firstPresentation}
></OnboardingDialog>
