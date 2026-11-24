<script lang="ts">
  import { untrack } from 'svelte'
  import { Sidebar, SidebarHeader, SidebarSection, SidebarFolder, SidebarItem, Button } from '../../src'
  import HardDrivesIcon from 'phosphor-svelte/lib/HardDrivesIcon'
  import ChartBarIcon from 'phosphor-svelte/lib/ChartBarIcon'
  import KeyIcon from 'phosphor-svelte/lib/KeyIcon'
  import CubeIcon from 'phosphor-svelte/lib/CubeIcon'
  import LightningIcon from 'phosphor-svelte/lib/LightningIcon'
  import BrainIcon from 'phosphor-svelte/lib/BrainIcon'
  import UsersIcon from 'phosphor-svelte/lib/UsersIcon'
  import NotebookIcon from 'phosphor-svelte/lib/NotebookIcon'
  import PlugsIcon from 'phosphor-svelte/lib/PlugsIcon'
  import ChatCircleIcon from 'phosphor-svelte/lib/ChatCircleIcon'
  let { scenario = 'Expanded' }: { scenario?: string } = $props()
  let selected = $state(untrack(() => (scenario === 'Selected' ? '#members' : '')))
  let organization = $state('compallm')
  let action = $state('')
  let settings = $state(false)
  let width = $state(244)
  let feedback = $state('')
  let items = $state(['First model'])
  let workspace = $derived(
    scenario === 'LongWorkspaceName' ? 'Research and development organization with a long workspace name' : 'Compallm',
  )
  let organizations = $derived([
    { id: 'compallm', name: workspace },
    { id: 'research', name: 'Research team' },
  ])
  const sections = [
    {
      label: 'Providers',
      add: 'Add provider',
      items: [
        ['OpenAI', '#provider-openai', HardDrivesIcon],
        ['Anthropic', '#provider-anthropic', HardDrivesIcon],
        ['Gemini', '#provider-gemini', HardDrivesIcon],
      ],
    },
    {
      label: 'Routers',
      add: 'Add router',
      items: [
        ['Stats', '#stats', ChartBarIcon],
        ['API key', '#api-key', KeyIcon],
      ],
    },
    {
      label: 'Workspace',
      add: '',
      items: [
        ['Members', '#members', UsersIcon],
        ['Notebooks', '#notebooks', NotebookIcon],
        ['Plugins', '#plugins', PlugsIcon],
      ],
    },
    {
      label: 'Conversations',
      add: 'New conversation',
      items: [
        ['Exploring a new idea', '#new-idea', ChatCircleIcon],
        ['Research notes', '#research', ChatCircleIcon],
        ['Model comparison', '#comparison', ChatCircleIcon],
      ],
    },
  ] as const
</script>

<div
  data-ui="component-preview"
  class={scenario === 'LongWorkspaceName'
    ? 'h-80 bg-background font-sans text-foreground'
    : 'flex h-svh bg-background font-sans text-foreground'}
>
  {#if scenario === 'Resizable'}<Sidebar bind:width onResizeFeedback={(kind, next) => (feedback = `${kind}: ${next}px`)}>
      <SidebarSection label="Workspace">
        <SidebarItem label="Models" href="#">{#snippet icon()}<CubeIcon />{/snippet}</SidebarItem>
      </SidebarSection>
    </Sidebar>
    <div class="min-w-0 flex-1 p-6">
      <h1 class="text-lg">Resizable sidebar</h1>
      <p class="mt-3 text-xs text-muted-foreground">Drag the divider or focus it and use Left/Right. Double-click to reset.</p>
      <output class="mt-3 block text-xs">{width}px · {feedback}</output>
    </div>
  {:else if scenario === 'NestedDisclosureAndAdding'}<Sidebar>
      <SidebarSection label="Workspace">
        <SidebarFolder
          label="Models"
          defaultOpen={false}
          onAdd={() => (items = [...items, `Model ${items.length + 1}`])}
          addLabel="Add sample model"
        >
          {#snippet icon()}<CubeIcon />{/snippet}{#each items as label (label)}<SidebarItem {label} href="#">
              {#snippet icon()}<CubeIcon />{/snippet}
            </SidebarItem>{/each}
        </SidebarFolder>
      </SidebarSection><SidebarSection label="Following category">
        <SidebarItem label="Sample destination" href="#">{#snippet icon()}<CubeIcon />{/snippet}</SidebarItem>
      </SidebarSection>
    </Sidebar>
  {:else}{#if !settings}<Sidebar>
        <SidebarHeader
          workspace={organizations.find((item) => item.id === organization)!.name}
          {organizations}
          selectedOrganizationId={organization}
          onOrganizationChange={(id) => (organization = id)}
          onWorkspaceClick={() => (action = 'Workspace switcher')}
          onSearch={() => (action = 'Workspace search')}
          onSettings={() => {
            action = 'Settings'
            if (scenario === 'SettingsNavigation') settings = true
          }}
          onSignOut={() => (action = 'Sign out')}
        />
        {#each sections as section (section.label)}<SidebarSection
            label={section.label}
            defaultOpen={scenario !== 'Collapsed'}
            onAdd={section.add ? () => (action = section.add) : undefined}
            addLabel={section.add || undefined}
          >
            {#each section.items as [label, href, Icon] (href)}<SidebarItem
                {label}
                {href}
                selected={selected === href}
                onNavigate={(href) => (selected = href)}
              >
                {#snippet icon()}<Icon />{/snippet}
              </SidebarItem>{/each}
            {#if section.label === 'Routers'}<SidebarFolder
                label="Models"
                defaultOpen={scenario !== 'Collapsed'}
                onAdd={() => (action = 'Add model')}
                addLabel="Add model"
              >
                {#snippet icon()}<CubeIcon />{/snippet}
                <SidebarItem
                  label="Fast Chat"
                  href="#fast-chat"
                  selected={selected === '#fast-chat'}
                  onNavigate={(href) => (selected = href)}
                >
                  {#snippet icon()}<LightningIcon />{/snippet}
                </SidebarItem><SidebarItem
                  label="claude-3.5-sonnet"
                  href="#sonnet"
                  selected={selected === '#sonnet'}
                  onNavigate={(href) => (selected = href)}
                >
                  {#snippet icon()}<BrainIcon />{/snippet}
                </SidebarItem>
              </SidebarFolder>{/if}
          </SidebarSection>{/each}
      </Sidebar>{/if}
    {#if scenario === 'SettingsNavigation'}<main class="min-w-0 flex-1 p-6">
        <h1 class="mb-4 text-lg font-medium">{settings ? 'Settings' : 'Workspace'}</h1>
        {#if settings}<Button onclick={() => (settings = false)}>Back to workspace</Button>{:else}<p
            class="text-xs text-muted-foreground"
          >
            Open the workspace menu, hover Switch organization, then select Settings.
          </p>{/if}
      </main>{/if}{/if}
  <output aria-label="Sidebar action" class="sr-only">{action}</output>
</div>
