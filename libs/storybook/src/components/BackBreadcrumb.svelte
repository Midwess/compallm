<script lang="ts">
  import ArrowLeftIcon from 'phosphor-svelte/lib/ArrowLeftIcon'
  import type { Snippet } from 'svelte'
  let { href, children }: { href: string; children?: Snippet } = $props()
</script>

<a
  {href}
  class="inline-flex min-h-7 items-center gap-1 text-xsm focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
>
  <ArrowLeftIcon size={14} aria-hidden="true" />{@render children?.()}
</a>
