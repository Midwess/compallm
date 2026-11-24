<script lang="ts">
  import { Select as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import { cx } from './shared'
  let { children, class: className, ...props }: ComponentProps<typeof UI.GroupHeading> = $props()
</script>

<UI.GroupHeading {...props} class={cx('p-1.5 text-ui-12 font-medium text-muted-foreground', className)}>
  {@render children?.()}
</UI.GroupHeading>
