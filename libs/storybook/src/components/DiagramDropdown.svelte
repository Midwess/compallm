<script lang="ts">
  import Dropdown from './Dropdown.svelte'
  import type { ComponentProps } from 'svelte'
  let {
    value = $bindable(),
    defaultValue = 'Default',
    label = 'Diagram style',
    defaultOpen = false,
    ...props
  }: Omit<ComponentProps<typeof Dropdown>, 'options'> = $props()
</script>

<Dropdown
  {...props}
  {label}
  {defaultValue}
  {defaultOpen}
  bind:value
  options={['Default', 'TUI', 'Hairline', 'Hand-drawn'].map((label) => ({ value: label, label }))}
/>
