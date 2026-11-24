<script lang="ts">
  import { onDestroy } from 'svelte'
  import { PageProgress, Dialog, Button } from '../../src'
  import Preview from './Preview.svelte'
  let { scenario = 'Requests' }: { scenario?: string } = $props()
  let pending = $state(0)
  let result = $state('')
  const timers: ReturnType<typeof setTimeout>[] = []
  onDestroy(() => timers.forEach(clearTimeout))
  function request(duration: number, failed = false) {
    pending++
    result = ''
    timers.push(
      setTimeout(() => {
        pending--
        result = failed ? 'Request failed; loading finished.' : 'Request completed.'
      }, duration),
    )
  }
</script>

<PageProgress active={scenario === 'Requests' ? pending > 0 : scenario !== 'Idle'} />
{#if scenario === 'Requests'}<Preview>
    <section class="grid gap-3">
      <p class="text-muted-foreground">Pending requests: {pending}</p>
      <Button onclick={() => request(2500)}>Start request</Button><Button onclick={() => request(30000)}>Slow request</Button>
      <Button
        onclick={() => {
          request(1500)
          request(4000)
        }}
      >
        Start overlapping requests
      </Button><Button
        onclick={() => {
          request(800)
          timers.push(setTimeout(() => request(800), 950))
        }}
      >
        Start consecutive requests
      </Button>
      <Button onclick={() => request(50)}>Fast request</Button><Button onclick={() => request(50, true)}>
        Failed request
      </Button>{#if result}<p role="status" class="text-muted-foreground">{result}</p>{/if}
    </section>
  </Preview>
{:else if scenario === 'AboveDialog'}<Preview>
    <Dialog.Root>
      <Dialog.Trigger>Customize sidebar</Dialog.Trigger><Dialog.Content>
        <Dialog.Header><Dialog.Title>Customize sidebar</Dialog.Title><Dialog.Close /></Dialog.Header><Dialog.Body>
          The request progress bar remains above this dialog.
        </Dialog.Body>
      </Dialog.Content>
    </Dialog.Root>
  </Preview>{/if}
