<script lang="ts">
  import MagnifyingGlassIcon from 'phosphor-svelte/lib/MagnifyingGlassIcon'
  import { cx, type ElementProps } from './shared'
  let { children, class: className, ...props }: ElementProps = $props()
</script>

<div {...props} class={cx('relative min-w-0', className)}>
  <MagnifyingGlassIcon
    aria-hidden="true"
    class="pointer-events-none absolute top-1/2 left-1.5 size-3.5 -translate-y-1/2 text-muted-foreground"
  />{@render children?.()}
</div>
