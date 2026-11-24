<script lang="ts">
  import Dropdown from './Dropdown.svelte'
  import type { ComponentProps } from 'svelte'
  let {
    value = $bindable(),
    defaultValue = 'Dark',
    label = 'Theme',
    defaultOpen = true,
    ...props
  }: Omit<ComponentProps<typeof Dropdown>, 'options'> = $props()
</script>

<Dropdown
  {...props}
  {label}
  {defaultValue}
  {defaultOpen}
  bind:value
  options={['System preference', 'Light', 'Pure Light', 'Dark', 'Magic Blue', 'Classic Dark', 'Custom'].map((label) => ({
    value: label,
    label,
  }))}
/>
