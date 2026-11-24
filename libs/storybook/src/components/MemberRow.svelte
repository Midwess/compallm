<script lang="ts">
  import Avatar from './Avatar.svelte'
  import Badge from './Badge.svelte'
  import Content from './Content.svelte'
  import type { Snippet } from 'svelte'
  import type { Content as Value } from './shared'
  let { name, email, role, children }: { name: string; email?: string; role: Value; children?: Snippet } = $props()
</script>

<div
  class="flex min-h-16 flex-wrap items-center gap-1.5 bg-inset p-2 max-[480px]:gap-1 [&_strong]:text-xsm [&_strong]:font-medium"
>
  <Avatar {name} />
  <div class="min-w-0 flex-1">
    <strong>{name}</strong>{#if email}<p class="text-ui-12 leading-4.5 text-muted-foreground">{email}</p>{/if}
  </div>
  <Badge><Content value={role} /></Badge>{@render children?.()}
</div>
