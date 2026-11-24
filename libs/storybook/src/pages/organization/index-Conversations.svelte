<script lang="ts">
  import ChatCircleIcon from 'phosphor-svelte/lib/ChatCircleIcon'

  import { SidebarFolder, SidebarItem } from '../../components/sidebar'

  import { untrack } from 'svelte'

  import { readStore } from '../page-state.svelte'
  import { getOrganizationController } from '../../context'

  const __organization = getOrganizationController()
  const controller = untrack(() => getOrganizationController())
  let conversations = $derived(readStore(__organization.store, (state) => state.conversations))
  let selected = $derived(readStore(__organization.store, (state) => state.selectedDestination))
</script>

<SidebarFolder label="Conversations" onAdd={() => controller.onOpenChat()} addLabel="New conversation">
  {#snippet icon()}<ChatCircleIcon></ChatCircleIcon>{/snippet}{#each conversations as item (item)}<SidebarItem
      label={item.name}
      href={item.href}
      selected={selected === item.id}
      onNavigate={() => controller.onOpenChat(item.id)}
    >
      {#snippet icon()}<ChatCircleIcon></ChatCircleIcon>{/snippet}
    </SidebarItem>{/each}
</SidebarFolder>
