<script lang="ts">
  import ToolbarAction from './ToolbarAction.svelte'
  import DotsSixVerticalIcon from 'phosphor-svelte/lib/DotsSixVerticalIcon'
  import type { ComponentProps } from 'svelte'
  let props: ComponentProps<typeof ToolbarAction> = $props()
</script>

<ToolbarAction {...props} class="w-6 cursor-grab rounded-full active:cursor-grabbing">
  <DotsSixVerticalIcon size={14} />
</ToolbarAction>
