<script lang="ts">
  import { getChoice, type ChoiceOption } from './choice-picker-context'
  import type { Snippet } from 'svelte'
  import { cx } from './shared'
  let { children, class: className }: { children: Snippet<[ChoiceOption]>; class?: string } = $props()
  const scope = getChoice()
</script>

<div class={cx('min-h-0 max-h-64 flex-1 overflow-y-auto overscroll-y-none p-1', className)}>
  {#if scope.open}{#each scope.options as item (item.id)}{@render children(item)}{/each}{/if}
</div>
