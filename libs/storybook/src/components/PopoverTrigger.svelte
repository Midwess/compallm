<script lang="ts">
  import { Popover as UI } from 'bits-ui'
  import type { ComponentProps } from 'svelte'
  import Button, { type ButtonProps } from './Button.svelte'
  let {
    children,
    child: renderChild,
    variant,
    size,
    loading = false,
    ...props
  }: ComponentProps<typeof UI.Trigger> & Pick<ButtonProps, 'variant' | 'size' | 'loading'> = $props()
</script>

<UI.Trigger {...props} disabled={props.disabled || loading}>
  {#snippet child({ props: trigger })}{#if renderChild}{@render renderChild({ props: trigger })}{:else}<Button
        {...trigger}
        {variant}
        {size}
        {loading}
      >
        {@render children?.()}
      </Button>{/if}{/snippet}
</UI.Trigger>
