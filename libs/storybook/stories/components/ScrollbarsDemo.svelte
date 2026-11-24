<script lang="ts">
  import { SettingsPageTitle, MultilineField } from '../../src'
</script>

<div class="grid max-w-208 gap-6 p-6">
  <SettingsPageTitle>Native scrollbars</SettingsPageTitle>
  <p class="text-xsm text-muted-foreground">The shared stylesheet styles every scroll container, including Safari.</p>
  <div class="grid gap-4 sm:grid-cols-2">
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div tabindex="0" role="region" aria-label="Vertical scrolling example" class="h-48 overflow-auto rounded-lg bg-inset p-4">
      {#each Array(18) as _, i (i)}<p class="py-2 text-xsm">Scrollable item {i + 1}</p>{/each}
    </div>
    <!-- svelte-ignore a11y_no_noninteractive_tabindex -->
    <div
      tabindex="0"
      role="region"
      aria-label="Both directions scrolling example"
      class="h-48 overflow-auto rounded-lg bg-inset p-4"
    >
      <div class="h-96 w-160 text-xsm">Horizontal and vertical scrolling share the same transparent track.</div>
    </div>
  </div>
  <MultilineField
    aria-label="Scrollable native field"
    class="h-32 resize-none text-xsm"
    value={Array.from({ length: 16 }, (_, i) => `Native textarea line ${i + 1}`).join('\n')}
  />
</div>
