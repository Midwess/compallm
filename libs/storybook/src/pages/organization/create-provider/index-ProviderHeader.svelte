<script lang="ts">
  import { PageHeader, Title, Functions } from '../../../components/page-header'

  import { untrack } from 'svelte'

  import { readStore } from '../../page-state.svelte'
  import { getCreateProviderController } from '../../../context'
  import { ProviderActions } from './index'

  const __provider = getCreateProviderController()
  const controller = untrack(() => getCreateProviderController())
  let step = $derived(readStore(__provider.store, (state) => state.step))
  let intent = $derived(readStore(__provider.store, (state) => state.intent))
  let readOnly = $derived(readStore(__provider.store, (state) => state.readOnly))
  let name = $derived(
    readStore(__provider.store, (state) => (state.intent === 'edit' ? state.savedProvider?.name : state.name), ['name']),
  )
</script>

{#if step === 'choose'}<PageHeader><Title>Choose provider</Title></PageHeader>{:else}{#if intent !== 'create'}<PageHeader>
      <Title>{name}</Title>{#if !readOnly}<Functions><ProviderActions></ProviderActions></Functions>{:else}{/if}
    </PageHeader>{:else}<PageHeader>
      <Title class="flex items-center gap-6">
        <button
          type="button"
          onclick={controller.onChooseProvider}
          class="cursor-pointer text-muted-foreground hover:text-foreground focus-visible:outline-ring"
        >
          Choose provider
        </button><span aria-hidden="true" class="text-muted-foreground">/ </span><span>Configure</span>
      </Title>
    </PageHeader>{/if}{/if}
