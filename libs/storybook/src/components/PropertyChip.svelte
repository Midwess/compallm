<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import Content from './Content.svelte'
  import { cx, type Content as Value } from './shared'
  let { icon, children, class: className, ...props }: ButtonProps & { icon?: Value } = $props()
</script>

<Button variant="ghost" size="content" {...props} class={cx('text-xsm', className)}>
  <Content value={icon} />{@render children?.()}
</Button>
