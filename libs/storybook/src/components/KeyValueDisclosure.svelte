<script lang="ts">
  import type { Snippet } from 'svelte'
  import Content from './KeyValueContent.svelte'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'
  import { disclosure } from './motion'
  let {
    children,
    open = $bindable(false),
    onOpenChange,
  }: { children?: Snippet; open?: boolean; onOpenChange?: (value: boolean) => void } = $props()
</script>

<details bind:open use:disclosure class="group/headers w-full min-w-0 max-w-full" ontoggle={() => onOpenChange?.(open)}>
  <summary class="flex h-7 w-fit cursor-pointer list-none items-center gap-1 text-sm font-medium">
    {#if children}{@render children()}{:else}Headers{/if}<CaretDownIcon size={8} />
  </summary>
  <div data-disclosure-body class="min-w-0 pt-3"><Content /></div>
</details>
