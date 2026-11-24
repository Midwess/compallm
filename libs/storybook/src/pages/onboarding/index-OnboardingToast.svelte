<script lang="ts">
  import { Toast } from '../../components/feedback'

  import { untrack } from 'svelte'

  let {
    message,
    onDismiss,
  }: {
    message: string
    onDismiss(): void
  } = $props()

  let host = $state<HTMLDivElement | null>(null)
  $effect(() => {
    void []
    return untrack(() => {
      const element = host
      const frame = requestAnimationFrame(() => element?.showPopover())
      return () => {
        cancelAnimationFrame(frame)
        element?.hidePopover()
      }
    })
  })
</script>

<div
  bind:this={host}
  popover="manual"
  class="fixed inset-auto top-4 right-4 m-0 max-w-[calc(100%-2rem)] border-0 bg-transparent p-0 text-foreground"
>
  <Toast tone="error" {message} {onDismiss}></Toast>
</div>
