<script lang="ts">
  import type { Snippet } from 'svelte'
  import { cx } from '../../src/components/shared'
  let { label, children, wide = false }: { label: string; children?: Snippet; wide?: boolean } = $props()
</script>

<section class={cx('min-w-0 max-w-full has-[[role=tooltip]]:pt-8', wide ? 'w-180' : 'w-88')}>
  <h3 class="mb-2 text-[12px] leading-5 font-normal text-muted-foreground">{label}</h3>
  <div
    class="flex w-full flex-col items-start gap-2 [&>div:not([data-ui=dropdown])]:w-full [&>input]:w-full [&>textarea]:w-full [&>figure]:w-full"
  >
    {@render children?.()}
  </div>
</section>
