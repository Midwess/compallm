<script lang="ts">
  import TextField from './TextField.svelte'
  import FormFieldLabel from './FormFieldLabel.svelte'
  import HelperText from './HelperText.svelte'
  import InlineError from './InlineError.svelte'
  import { cx } from './shared'
  import type { ComponentProps } from 'svelte'
  let {
    label,
    hint,
    error,
    id,
    class: className,
    ...props
  }: ComponentProps<typeof TextField> & { label: string; hint?: string; error?: string } = $props()
  const generated = $props.id()
  let inputId = $derived(id ?? generated)
</script>

<div class={cx('grid gap-0.5', className)}>
  <FormFieldLabel for={inputId}>{label}</FormFieldLabel><TextField
    {...props}
    id={inputId}
    aria-invalid={!!error || props['aria-invalid']}
    aria-describedby={[props['aria-describedby'], hint ? inputId + '-hint' : '', error ? inputId + '-error' : '']
      .filter(Boolean)
      .join(' ') || undefined}
  />{#if hint}<HelperText id={inputId + '-hint'}>{hint}</HelperText>{/if}{#if error}<InlineError id={inputId + '-error'}>
      {error}
    </InlineError>{/if}
</div>
