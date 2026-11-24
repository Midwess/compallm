<script lang="ts">
  import Preload from '../../components/Preload.svelte'
  import { List } from '../../components/list'
  import { PermissionRequest } from '../../components/permission'

  import { Button } from '../../components/buttons'
  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getOrganizationController } from '../../context'

  let {
    kind,
  }: {
    kind: 'models' | 'providers'
  } = $props()

  const controller = untrack(() => getOrganizationController())
  if (!controller) throw new Error('Organization catalog requires OrganizationControllerContext.')
  let items = $derived(readStore(controller.store, (state) => state[kind]))
  let providers = $derived(kind === 'providers')
</script>

<PermissionRequest actions={[providers ? 'provider.read' : 'model.read']}>
  {#snippet children(allowed)}{#if allowed}<section class="mx-auto grid max-w-4xl gap-8 px-8 py-10">
        <header class="flex items-center justify-between gap-4">
          <h1 class="text-2xl font-medium">
            {#if providers}Providers{:else}Models{/if}
          </h1>
          {#if providers}<PermissionRequest actions={['provider.create']}>
              {#snippet children(allowed)}{#if allowed}<Button onclick={controller.onCreateProvider}>
                    Add provider
                  </Button>{:else}{/if}{/snippet}
            </PermissionRequest>{:else}{/if}
        </header>
        {#if items.length}<List.Root>
            <List.Items {items} getKey={(item) => item.id} class="rounded-lg bg-inset px-4">
              {#snippet children(item)}<Preload
                  href={item.href}
                  class="block border-b border-divider px-4 py-4 text-xsm hover:bg-hover"
                  onclick={(event) => {
                    if (
                      event.defaultPrevented ||
                      event.button !== 0 ||
                      event.metaKey ||
                      event.ctrlKey ||
                      event.shiftKey ||
                      event.altKey
                    )
                      return
                    event.preventDefault()
                    controller.onOpenDestination(item.id)
                  }}
                >
                  {item.name}
                </Preload>{/snippet}
            </List.Items>
          </List.Root>{:else}<div class="grid gap-2 rounded-lg bg-inset px-6 py-8">
            <h2 class="text-sm font-medium">
              {#if providers}No providers available{:else}No models available{/if}
            </h2>
            <p class="text-xsm text-muted-foreground">
              {#if providers}Providers will appear here when available.{:else}Models will appear here when available.{/if}
            </p>
          </div>{/if}
      </section>{:else}<p role="status">You do not have access to this page.</p>{/if}{/snippet}
</PermissionRequest>
