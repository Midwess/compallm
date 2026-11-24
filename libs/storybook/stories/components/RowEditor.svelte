<script lang="ts">
  import { untrack } from 'svelte'
  import { Form, managedForm } from '../../src/components/form'
  import TextField from '../../src/components/TextField.svelte'
  import Button from '../../src/components/Button.svelte'
  import { ChoicePicker } from '../../src/components/choice-picker'
  let { record }: { record: { id: string; name: string } } = $props()
  const form = untrack(() => managedForm({ name: record.name }, { submit: (values) => values }))
  let activated = $state(false)
  let open = $state(false)
  let selected = $state<string[]>([])
</script>

<Form.Root model={form}>
  <div class="flex items-center gap-2">
    <TextField name="name" aria-label={`Edit ${record.id}`} /><Button type="submit">
      Save
    </Button>{#if activated}<ChoicePicker.Control
        bind:open
        options={[{ id: 'group', label: 'Example group' }]}
        bind:value={selected}
        label={`Groups ${record.id}`}
      />{:else}<Button
        aria-label={`Groups ${record.id}`}
        onclick={() => {
          activated = true
          open = true
        }}
      >
        Groups
      </Button>{/if}
  </div>
</Form.Root>
