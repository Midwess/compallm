<script lang="ts">
  import Content from './Content.svelte'
  import type { Snippet } from 'svelte'
  import type { Content as Value } from './shared'
  let { title, children }: { title: Value; children?: Snippet } = $props()
</script>

<header class="[&>h1]:text-ui-24 [&>h1]:leading-8 [&>h1]:font-medium [&>div]:mt-1 [&>div]:text-xsm">
  <h1><Content value={title} /></h1>
  <div class="text-muted-foreground">{@render children?.()}</div>
</header>
