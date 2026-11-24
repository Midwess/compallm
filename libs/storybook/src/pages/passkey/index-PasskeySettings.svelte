<script lang="ts">
  import { LoadingRegion } from '../../components/loading'
  import { List } from '../../components/list'

  import { Button } from '../../components/buttons'

  import { Section, Rows, Empty } from '../settings/shared'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getPasskeyController } from '../../context'
  import { PasskeyRow } from './index'

  const controller = untrack(() => getPasskeyController())
  let ready = $derived(readStore(controller.store, (s) => s.status !== 'denied' && (s.loaded ?? s.status === 'ready')))
  let records = $derived(readStore(controller.store, (s) => s.records))
  let status = $derived(readStore(controller.store, (s) => s.status))
  let error = $derived(readStore(controller.store, (s) => s.error))
  let currentPasskeyId = $derived(readStore(controller.store, (s) => s.currentPasskeyId))
  let busy = $derived(readStore(controller.store, (s) => s.busy))
</script>

<Section title="Passkeys" description="Passkeys are a secure way to sign in to your Compallm account.">
  {#snippet action()}{#if status === 'ready' && records.length}<Button
        variant="ghost"
        onclick={controller.openAdd}
        loading={busy}
      >
        New passkey
      </Button>{:else}{undefined}{/if}{/snippet}{#if !ready && status === 'loading'}<LoadingRegion
      ready={false}
      pending
      label="Loading passkeys"
    >
      <Rows>
        <List.Root>
          <List.Items items={[]} getKey={() => ''}>{#snippet children(_item, _index)}<span></span>{/snippet}</List.Items>
        </List.Root>
      </Rows>
    </LoadingRegion>{:else}{/if}{#if status === 'error' || status === 'denied'}<Empty
      title={status === 'denied' ? 'Passkey access denied' : 'Couldn’t load passkeys'}
    >
      {#snippet action()}{#if status === 'error'}<Button onclick={() => void controller.load()}>
            Retry
          </Button>{:else}{undefined}{/if}{/snippet}{error}
    </Empty>{:else}{/if}{#if ready}{#if records.length}<Rows>
        <ul class="m-0 list-none p-0">
          {#each records as record, index (index)}<PasskeyRow
              {record}
              current={record.id === currentPasskeyId}
              last={index === records.length - 1}
            ></PasskeyRow>{/each}
        </ul>
      </Rows>{:else}<Empty title="No passkeys registered">
        {#snippet action()}<Button onclick={controller.openAdd} loading={busy}>New passkey</Button>{/snippet}Add a passkey for a
        simpler sign-in experience.
      </Empty>{/if}{:else}{/if}
</Section>
