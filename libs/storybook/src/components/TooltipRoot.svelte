<script lang="ts">
  import { Tooltip as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'

  let { children, ...props }: Omit<ComponentProps<typeof UI.Root>, 'children'> & { children?: import('svelte').Snippet } =
    $props()
</script>

<UI.Provider><UI.Root {...props}>{@render children?.()}</UI.Root></UI.Provider>
