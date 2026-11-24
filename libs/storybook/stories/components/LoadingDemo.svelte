<script lang="ts">
  import { untrack } from 'svelte'
  import { LoadingRegion, Skeleton, FormField, SectionSurface, Button, Table } from '../../src'
  let { scenario = 'FirstLoad' }: { scenario?: string } = $props()
  let loaded = $state(untrack(() => ['Loaded', 'Refreshing', 'Empty'].includes(scenario)))
  let error = $state(untrack(() => scenario === 'RecoverableError'))
  let name = $state(untrack(() => (loaded ? 'My connection' : '')))
  function toggle() {
    loaded = !loaded
    if (loaded && !name) name = 'My connection'
  }
</script>

<div class="mx-auto grid max-w-xl gap-4 p-6">
  <h1 class="text-2xl font-medium">Connection</h1>
  <Button onclick={toggle}>{loaded ? 'Show first load' : 'Show loaded content'}</Button>
  {#if error}<p role="alert">
      Could not load the connection. <Button
        onclick={() => {
          error = false
          loaded = true
          name = 'My connection'
        }}
      >
        Retry
      </Button>
    </p>{/if}
  <LoadingRegion
    ready={loaded}
    pending={(!loaded || scenario === 'Refreshing') && !error}
    refreshing={loaded && scenario === 'Refreshing'}
    label="Loading connection"
  >
    <SectionSurface class="grid gap-4">
      <FormField.Root>
        <FormField.Label>Name</FormField.Label><FormField.Input bind:value={name} placeholder="Enter connection name" />
      </FormField.Root>
      <Skeleton><p class="h-5 text-sm">Connection details</p></Skeleton>
      <Table.Root>
        <Table.Content columns={[{ id: 'model' }, { id: 'status' }]}>
          <Table.Header><Table.Row><Table.Head>Model</Table.Head><Table.Head>Status</Table.Head></Table.Row></Table.Header>
          <Table.Body
            items={loaded && scenario !== 'Empty' ? [{ id: 'model', name: 'Example model' }] : []}
            getKey={(item) => item.id}
          >
            {#snippet children(item)}<Table.Row>
                <Table.Cell>{item.name}</Table.Cell><Table.Cell>Enabled</Table.Cell>
              </Table.Row>{/snippet}
          </Table.Body>
        </Table.Content>
      </Table.Root>
      {#if loaded && scenario === 'Empty'}<p class="text-sm text-muted-foreground">No models available.</p>{/if}
    </SectionSurface>
  </LoadingRegion>
</div>
