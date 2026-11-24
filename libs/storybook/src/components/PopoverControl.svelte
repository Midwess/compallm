<script lang="ts">
  import * as Popover from './popover-parts'
  import type { Snippet } from 'svelte'
  let { label, children }: { label: string; children?: Snippet } = $props()
</script>

<Popover.Root>
  <Popover.Trigger aria-label={label}>
    {label}
  </Popover.Trigger><Popover.Content>{@render children?.()}</Popover.Content>
</Popover.Root>
