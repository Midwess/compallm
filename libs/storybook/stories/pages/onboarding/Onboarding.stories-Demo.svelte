<script lang="ts">
  import { OnboardingPage, type OnboardingState } from '../../../src/pages/onboarding'

  import { DemoOnboardingController } from './demo-controller'
  import { untrack } from 'svelte'

  import { setOnboardingController } from '../../../src/context'

  let {
    initial = {},
    reject = false,
  }: {
    initial?: Partial<OnboardingState>
    reject?: boolean
  } = $props()

  let controller = $state(untrack(() => new DemoOnboardingController(initial, reject)))
  $effect(() => {
    void [controller]
    return untrack(() => () => controller.stop())
  })
  setOnboardingController(controller)
</script>

<OnboardingPage></OnboardingPage>
