<script lang="ts">
  import { Select as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.Group> = $props()
</script>

<UI.Group {...props} class={cx('', className)}>{@render children?.()}</UI.Group>
