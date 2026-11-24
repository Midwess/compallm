<script lang="ts">
  import Badge from './Badge.svelte'
  import CircleIcon from 'phosphor-svelte/lib/CircleIcon'
  import type { ComponentProps } from 'svelte'
  let { children, ...props }: ComponentProps<typeof Badge> = $props()
</script>

<Badge {...props}><CircleIcon size={12} aria-hidden="true" />{@render children?.()}</Badge>
