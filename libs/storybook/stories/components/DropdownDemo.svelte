<script lang="ts">
  import { Dropdown, Select, Dialog } from '../../src'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  let { scenario = 'Dropdown' }: { scenario?: string } = $props()
  const options = [
    { value: 'typescript', label: 'TypeScript' },
    { value: 'rust', label: 'Rust' },
    { value: 'python', label: 'Python' },
    { value: 'unavailable', label: 'Unavailable', disabled: true },
  ]
  let value = $state('typescript')
  let composed = $state('rust')
  let visible = $state(true)
</script>

<Preview>
  {#if scenario === 'SelectComposition'}<Sample label="Shared select parts">
      {#if visible}<Select.Root items={options} bind:value={composed}>
          <Select.Trigger aria-label="Composed language" class="w-60"><Select.Value /></Select.Trigger>
          <Select.Content aria-label="Composed language">
            {#each options as item (item.value)}<Select.Item value={item.value} disabled={item.disabled}>
                {item.label}
              </Select.Item>{/each}
          </Select.Content>
        </Select.Root>{/if}
      <output aria-label="Composed selection">{composed}</output><button
        type="button"
        class="text-xs text-primary"
        onclick={() => (visible = !visible)}
      >
        {visible ? 'Remove select' : 'Restore select'}
      </button>
    </Sample>
  {:else if scenario === 'SelectInDialog'}<Dialog.Root>
      <Dialog.Trigger>Open select dialog</Dialog.Trigger><Dialog.Content>
        <Dialog.Header><Dialog.Title>Choose language</Dialog.Title><Dialog.Close /></Dialog.Header>
        <Dialog.Body><Select.Control aria-label="Dialog language" {options} defaultValue="rust" /></Dialog.Body><Dialog.Footer>
          <Dialog.Cancel />
        </Dialog.Footer>
      </Dialog.Content>
    </Dialog.Root>
  {:else}<Sample
      label={scenario === 'RepeatedSelection' ? 'Choose repeatedly, close, and reopen' : 'Keyboard and pointer selection'}
    >
      <Dropdown label="Language" {options} bind:value /><output aria-label="Selected language">{value}</output>
    </Sample>{#if scenario === 'Dropdown'}<Sample label="Expanded">
        <Dropdown label="Expanded language" {options} defaultOpen />
      </Sample><Sample label="Disabled"><Dropdown label="Disabled language" {options} disabled /></Sample>{/if}{/if}
</Preview>
