<script lang="ts">
  import { DropdownMenu as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'

  let { children, ...props }: ComponentProps<typeof UI.Sub> = $props()
</script>

<UI.Sub {...props}>{@render children?.()}</UI.Sub>
