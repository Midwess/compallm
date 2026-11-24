<script lang="ts">
  import { untrack } from 'svelte'
  import { ChoicePicker } from '../../src/components/choice-picker'
  import Button from '../../src/components/Button.svelte'
  let {
    empty = false,
    disabled = false,
    composed = false,
  }: { empty?: boolean; disabled?: boolean; composed?: boolean } = $props()
  let value = $state<string[]>(untrack(() => (composed ? ['viewer'] : [])))
  let open = $state(false)
  const options = [
    { id: 'operator', label: 'Model operator', description: 'Manage and use models.' },
    { id: 'viewer', label: 'Usage viewer', description: 'View usage reports.' },
    { id: 'disabled', label: 'Unavailable role', disabled: true },
  ]
</script>

<div class="flex min-h-72 flex-col items-start gap-4 bg-background p-6 text-foreground">
  {#if composed}<ChoicePicker.Root {options} bind:value bind:open>
      <ChoicePicker.Trigger>Add roles</ChoicePicker.Trigger><ChoicePicker.Content>
        <div class="p-2 pb-0"><ChoicePicker.Input aria-label="Search roles" placeholder="Search roles…" /></div>
        <ChoicePicker.Empty>No roles found.</ChoicePicker.Empty><ChoicePicker.List>
          {#snippet children(option)}<ChoicePicker.Item value={option} />{/snippet}
        </ChoicePicker.List>
      </ChoicePicker.Content>
    </ChoicePicker.Root><output>{value.join(', ') || 'No roles selected'}</output><Button onclick={() => (value = [])}>
      Reset selection
    </Button>{:else}<ChoicePicker.Control
      label="Add role"
      searchLabel="Search roles"
      placeholder="Search roles…"
      emptyMessage="No roles found."
      options={empty ? [] : options}
      bind:value
      {disabled}
    />
    <p class="text-xs text-muted-foreground">{value.length} selected</p>{/if}
</div>
