<script lang="ts">
  import { Dialog, AlertDialog, Button, FormField, Popup, Table, Switch } from '../../src'
  import FunnelIcon from 'phosphor-svelte/lib/FunnelIcon'
  let { scenario = 'DialogComposition' }: { scenario?: string } = $props()
  let open = $state(false)
  let count = $state(0)
  let status = $state('all')
  let modelType = $state('all')
  let capabilities = $state<string[]>([])
  let outsideClicks = $state(0)
</script>

{#if scenario === 'DialogComposition' || scenario === 'DialogRerender'}
  <Dialog.Root bind:open>
    <Dialog.Trigger>{scenario === 'DialogRerender' ? 'Open updating dialog' : 'Open dialog'}</Dialog.Trigger>
    <Dialog.Content>
      <Dialog.Header>
        <Dialog.Title>{scenario === 'DialogRerender' ? 'Update details' : 'Edit details'}</Dialog.Title><Dialog.Close />
      </Dialog.Header>
      {#if scenario === 'DialogComposition'}<Dialog.Discard />{/if}
      <Dialog.Body>
        {#if scenario === 'DialogRerender'}<output aria-label="Dialog update count">{count}</output><Button
            onclick={() => count++}
          >
            Update dialog content
          </Button>
        {:else}<FormField.Root>
            <FormField.Label>Name</FormField.Label><FormField.Input autofocus placeholder="Enter name" />
          </FormField.Root>{/if}
      </Dialog.Body>
      <Dialog.Footer><Dialog.Cancel /><Button variant="primary" onclick={() => (open = false)}>Save</Button></Dialog.Footer>
    </Dialog.Content>
  </Dialog.Root>
{:else if scenario === 'AlertComposition'}<AlertDialog.Root bind:open>
    <AlertDialog.Trigger>Confirm discard</AlertDialog.Trigger><AlertDialog.Content>
      <AlertDialog.Header><AlertDialog.Title>Discard changes?</AlertDialog.Title></AlertDialog.Header><AlertDialog.Body>
        <AlertDialog.Description>Your unsaved changes will be lost.</AlertDialog.Description>
      </AlertDialog.Body>
      <AlertDialog.Footer>
        <Button autofocus onclick={() => (open = false)}>Keep editing</Button><Button
          variant="destructive"
          onclick={() => (open = false)}
        >
          Discard changes
        </Button>
      </AlertDialog.Footer>
    </AlertDialog.Content>
  </AlertDialog.Root>
{:else if scenario === 'CategorizedTable'}<div class="max-h-64 max-w-160 overflow-auto rounded-lg bg-inset p-4">
    <Table.Root class="overflow-visible">
      <Table.Content class="border-separate border-spacing-0" aria-label="Categorized table example">
        <Table.Header class="sticky top-0 z-20 bg-inset">
          <Table.Row><Table.Head>Name</Table.Head><Table.Head>Scope</Table.Head><Table.Head>Allow</Table.Head></Table.Row>
        </Table.Header>
        {#each ['Models', 'Providers', 'Groups'] as category (category)}<Table.Category label={category} count="0 / 5">
            {#snippet action()}<Switch aria-label={'Allow ' + category} />{/snippet}
            {#each Array(5) as _, i (i)}<Table.Row>
                <Table.Cell>{category} action {i + 1}</Table.Cell><Table.Cell>Sample scope</Table.Cell><Table.Cell>
                  <Switch aria-label={category + ' action ' + (i + 1)} />
                </Table.Cell>
              </Table.Row>{/each}
          </Table.Category>{/each}
      </Table.Content>
    </Table.Root>
  </div>
{:else if scenario.startsWith('Filter')}<div
    class={scenario === 'FilterAtContainerEdge'
      ? 'flex h-80 flex-col items-end justify-end gap-4 overflow-hidden'
      : 'grid gap-4'}
  >
    <Popup.Root>
      <Popup.IconTrigger aria-label="Filter example">
        <FunnelIcon aria-hidden="true" class="size-3.5" />
      </Popup.IconTrigger><Popup.Content aria-label="Filter example">
        <Popup.Sub>
          <Popup.SubTrigger>Selection</Popup.SubTrigger><Popup.SubContent aria-label="Selection">
            <Popup.RadioGroup value={status} onValueChange={(value) => (status = value)}>
              {#each ['all', 'enabled', 'disabled'] as value (value)}<Popup.RadioItem {value}>
                  {value === 'all' ? 'All policies' : value === 'enabled' ? 'Enabled' : 'Disabled'}
                </Popup.RadioItem>{/each}
            </Popup.RadioGroup>
          </Popup.SubContent>
        </Popup.Sub>
        <Popup.Sub>
          <Popup.SubTrigger>Model type</Popup.SubTrigger><Popup.SubContent aria-label="Model type">
            <Popup.RadioGroup value={modelType} onValueChange={(value) => (modelType = value)}>
              {#each ['all', 'OpenAI Completion', 'OpenAI Response', 'Anthropic'] as value (value)}<Popup.RadioItem {value}>
                  {value === 'all' ? 'All model types' : value}
                </Popup.RadioItem>{/each}
            </Popup.RadioGroup>
          </Popup.SubContent>
        </Popup.Sub>
        <Popup.Sub>
          <Popup.SubTrigger>Capabilities</Popup.SubTrigger><Popup.SubContent aria-label="Capabilities">
            {#each ['Vision', 'Tools', 'Reasoning'] as value (value)}<Popup.CheckboxItem
                checked={capabilities.includes(value)}
                onCheckedChange={(checked) =>
                  (capabilities = checked ? [...capabilities, value] : capabilities.filter((item) => item !== value))}
              >
                {value}
              </Popup.CheckboxItem>{/each}
          </Popup.SubContent>
        </Popup.Sub><Popup.Separator /><Popup.Item
          onSelect={() => {
            status = 'all'
            modelType = 'all'
            capabilities = []
          }}
        >
          Clear all filters
        </Popup.Item>
      </Popup.Content>
    </Popup.Root>
    <p class="text-xs text-muted-foreground" aria-live="polite">
      Selection: {status} · Model type: {modelType} · Capabilities: {capabilities.join(', ') || 'all'}
    </p>
    <Button onclick={() => outsideClicks++}>Outside action · {outsideClicks}</Button>
  </div>
{:else if scenario === 'ChildComposition'}<div class="grid justify-items-start gap-4">
    <Dialog.Root>
      <Dialog.Trigger class="h-6.5">Open composed dialog</Dialog.Trigger><Dialog.Content>
        <Dialog.Header><Dialog.Title>Composed controls</Dialog.Title><Dialog.Close /></Dialog.Header><Dialog.Body>
          <p>Close with Escape or the button. Outside clicks keep this dialog open.</p>
        </Dialog.Body><Dialog.Footer><Dialog.Cancel>Done</Dialog.Cancel></Dialog.Footer>
      </Dialog.Content>
    </Dialog.Root>
    <Popup.Root>
      <Popup.Trigger class="h-6.5 w-auto rounded-full bg-elevated px-1.5 text-foreground shadow-control">
        Composed menu
      </Popup.Trigger><Popup.Content aria-label="Composed actions">
        <Popup.Item onSelect={() => count++}>Count selection</Popup.Item><Popup.Item>Custom menu item</Popup.Item>
      </Popup.Content>
    </Popup.Root>
    <Button disabled>
      {#snippet child(props)}<a
          class={props.class}
          aria-disabled={props['aria-disabled']}
          href="#disabled"
          onclick={(event) => {
            if (props.disabled) event.preventDefault()
          }}
        >
          Disabled child link
        </a>{/snippet}
    </Button>
    <Dialog.Root>
      <Dialog.Trigger onclick={(event) => event.preventDefault()}>Cancel opening</Dialog.Trigger><Dialog.Content>
        <Dialog.Title>This dialog stays closed</Dialog.Title>
      </Dialog.Content>
    </Dialog.Root>
    <output aria-live="polite">Selections: {count}</output>
  </div>{/if}
