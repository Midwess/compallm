<script lang="ts">
  import { Popup } from './popup'
  import MagnifyingGlassIcon from 'phosphor-svelte/lib/MagnifyingGlassIcon'
  import CaretDownIcon from 'phosphor-svelte/lib/CaretDownIcon'
  import ToolbarAction from './ToolbarAction.svelte'
  import Content from './Content.svelte'
  import type { Content as Value } from './shared'
  let {
    workspace,
    mark,
    onWorkspaceClick,
    onSearch,
    onSettings,
    onSignOut,
    organizations = [],
    selectedOrganizationId,
    onOrganizationChange,
  }: {
    workspace: string
    mark?: Value
    onWorkspaceClick?: () => void
    onSearch: () => void
    onSettings: () => void
    onSignOut?: () => void
    organizations?: { id: string; name: string }[]
    selectedOrganizationId?: string
    onOrganizationChange?: (id: string) => void
  } = $props()
</script>

<div class="mt-3 flex h-10 w-full min-w-0 shrink-0 items-center justify-between gap-2 pl-2 pr-1">
  <Popup.Root>
    <Popup.Trigger
      aria-label={`Switch workspace: ${workspace}`}
      onclick={onWorkspaceClick}
      class="h-8 w-auto min-w-0 flex-1 justify-start gap-1 -ml-2 rounded-xl px-2 text-xs leading-4 text-foreground"
    >
      {#if mark}<Content value={mark} />{:else}<span
          aria-hidden="true"
          class="flex aspect-square h-4.5 pb-0.5 shrink-0 items-center justify-center rounded-md bg-brand-mark text-xs leading-none font-bold text-brand-mark-foreground"
        >
          c
        </span>{/if}<span class="truncate font-medium leading-4">{workspace}</span><CaretDownIcon
        size={10}
        class="ml-0.5 shrink-0 text-muted-foreground"
        aria-hidden="true"
      />
    </Popup.Trigger><Popup.Content aria-label="Workspace options" align="start" sideOffset={8} class="w-55">
      <Popup.Item onSelect={onSettings}>Settings</Popup.Item><Popup.Sub>
        <Popup.SubTrigger>Switch organization</Popup.SubTrigger><Popup.SubContent aria-label="Switch organization">
          <Popup.RadioGroup value={selectedOrganizationId ?? ''} onValueChange={(id) => onOrganizationChange?.(id)}>
            {#each organizations as organization (organization.id)}<Popup.RadioItem value={organization.id}>
                {organization.name}
              </Popup.RadioItem>{/each}
          </Popup.RadioGroup>
        </Popup.SubContent>
      </Popup.Sub>{#if onSignOut}<Popup.Item onSelect={onSignOut}>Sign out</Popup.Item>{/if}
    </Popup.Content>
  </Popup.Root><ToolbarAction label="Search workspace" class="size-8 shrink-0 rounded-full" onclick={onSearch}>
    <MagnifyingGlassIcon size={14} />
  </ToolbarAction>
</div>
