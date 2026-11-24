<script lang="ts">
  import { Dialog } from '../../components/dialog'
  import { Button } from '../../components/buttons'

  import { OnboardingPage } from './index'

  let {
    open,
    onOpenChange,
    firstPresentation = false,
  }: {
    open: boolean
    onOpenChange(open: boolean): void
    firstPresentation?: boolean
  } = $props()
</script>

<Dialog.Root {open} {onOpenChange}>
  <Dialog.Content
    oncancel={(event) => event.preventDefault()}
    class={[
      'w-2xl max-w-[calc(100%-2rem)] rounded-3xl border-0 bg-transparent',
      firstPresentation ? 'backdrop:bg-background' : 'backdrop:bg-transparent',
    ].join(' ')}
  >
    <Dialog.Title class="sr-only">Welcome to your workspace</Dialog.Title><OnboardingPage></OnboardingPage><Button
      variant="ghost"
      class="absolute top-9 right-6"
      onclick={() => onOpenChange(false)}
    >
      Close
    </Button>
  </Dialog.Content>
</Dialog.Root>
