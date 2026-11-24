<script lang="ts">
  import Content from './LoadingRegionContent.svelte'
  import LoadingScope from './LoadingScope.svelte'
  import type { LoadingController } from '../app'
  import type { ElementProps } from './shared'
  let {
    ready,
    controller,
    pending = false,
    refreshing = false,
    label,
    children,
    ...props
  }: ElementProps & {
    ready: boolean
    controller?: LoadingController
    pending?: boolean
    refreshing?: boolean
    label: string
  } = $props()
  let busy = $derived(controller ? controller.store.getState().loading && !controller.store.getState().error : pending)
</script>

<div {...props} aria-busy={busy || undefined}>
  {#if busy}<span role="status" class="sr-only">{label}</span>{/if}<LoadingScope {ready} {refreshing}>
    <Content>{@render children?.()}</Content>
  </LoadingScope>
</div>
