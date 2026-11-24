<script lang="ts">
  import { SettingsPageTitle } from '../../src/components/settings'

  const colors = [
    ['Background', 'bg-background', '--background'],
    ['Surface', 'bg-surface', '--card'],
    ['Inset', 'bg-inset', '--inset'],
    ['Composer', 'bg-composer', '--composer'],
    ['Elevated', 'bg-elevated', '--elevated'],
    ['Menu', 'bg-menu', '--popover'],
    ['Divider', 'bg-divider', '--divider'],
    ['Border', 'bg-border', '--border'],
    ['Input border', 'bg-input', '--input'],
    ['Text', 'bg-foreground', '--foreground'],
    ['Muted', 'bg-muted-foreground', '--muted-foreground'],
    ['Disabled', 'bg-disabled', '--disabled'],
    ['Primary', 'bg-primary', '--primary'],
    ['Primary hover', 'bg-primary-hover', '--primary-hover'],
    ['Focus', 'bg-ring', '--ring'],
    ['On primary', 'bg-primary-foreground', '--primary-foreground'],
    ['Success', 'bg-success', '--success'],
    ['Warning', 'bg-warning', '--warning'],
    ['Error', 'bg-destructive', '--destructive'],
    ['Shadow', 'bg-shadow', '--shadow-color'],
  ] as const
</script>

<div class="grid gap-6 p-6">
  <SettingsPageTitle>Compallm colors</SettingsPageTitle>
  <div class="grid grid-cols-[repeat(auto-fit,minmax(160px,1fr))] gap-4">
    {#each colors as [name, background, token] (name)}<figure class="grid gap-2">
        <div class={`${background} h-16 rounded-lg border-(length:--border-hairline) border-input`}></div>
        <figcaption class="text-xsm">{name}<span class="block text-xs text-muted-foreground">{token}</span></figcaption>
      </figure>{/each}
  </div>
</div>
