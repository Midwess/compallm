<script lang="ts">
  import Content from './Content.svelte'
  import type { Snippet } from 'svelte'
  import type { Content as Value } from './shared'
  let { title, action, children }: { title: Value; action?: Value; children?: Snippet } = $props()
</script>

<section class="grid justify-items-center gap-2 p-8 text-center">
  <h3 class="text-sm font-medium"><Content value={title} /></h3>
  <div class="text-xsm text-muted-foreground">{@render children?.()}</div>
  <Content value={action} />
</section>
