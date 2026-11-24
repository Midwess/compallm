<script lang="ts">
  import { getFieldContext } from '../context'
  import type { HTMLLabelAttributes } from 'svelte/elements'
  import { cx } from './shared'
  let { children, class: className, ...props }: HTMLLabelAttributes = $props()
  const id = getFieldContext()
</script>

<label {...props} for={id} class={cx('text-sm font-medium', className)}>{@render children?.()}</label>
