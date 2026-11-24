<script lang="ts">
  import * as Tooltip from './tooltip-parts'
  import { Tooltip as UI } from 'bits-ui'
  import Content from './Content.svelte'
  import type { Content as ContentValue } from './shared'
  import type { Snippet } from 'svelte'
  let {
    children,
    message,
    label,
    content,
  }: { children?: Snippet; message?: ContentValue; label?: ContentValue; content?: ContentValue } = $props()
</script>

<UI.Provider delayDuration={300}>
  <Tooltip.Root>
    <Tooltip.Trigger>
      {#snippet child({ props })}<span {...props} tabindex={0} role="button">{@render children?.()}</span>{/snippet}
    </Tooltip.Trigger><Tooltip.Content><Content value={message ?? content ?? label} /></Tooltip.Content>
  </Tooltip.Root>
</UI.Provider>
