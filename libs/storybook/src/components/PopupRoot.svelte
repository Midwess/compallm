<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'

  let { children, ...props }: ComponentProps<typeof UI.Root> = $props()
</script>

<UI.Root {...props}>{@render children?.()}</UI.Root>
