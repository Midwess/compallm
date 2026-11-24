<script lang="ts">
  import type { HTMLInputAttributes } from 'svelte/elements'
  import { fieldModel } from './form-context.svelte'
  let { name, onchange, ...props }: Omit<HTMLInputAttributes, 'type' | 'value'> = $props()
  const form = fieldModel(() => name)
</script>

<input
  {...props}
  {name}
  type="file"
  onchange={(e) => {
    if (form && name) form.change(name, e.currentTarget.files?.[0] ?? null)
    else onchange?.(e)
    e.currentTarget.value = ''
  }}
/>
