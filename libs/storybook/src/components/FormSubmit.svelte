<script lang="ts">
  import Button, { type ButtonProps } from './Button.svelte'
  import { currentForm } from './form-context.svelte'
  let { children, disabled, ...props }: ButtonProps = $props()
  const form = currentForm()
</script>

<Button {...props} type="submit" disabled={disabled || form?.pending} loading={form?.pending}>{@render children?.()}</Button>
