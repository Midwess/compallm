<script lang="ts">
  import { Button } from '../../src/components/buttons'
  import { Checkbox, Switch } from '../../src/components/selection'

  import { HelperText } from '../../src/components/feedback'
  import { SettingsPageTitle } from '../../src/components/settings'
</script>

<div class="grid gap-6 p-6">
  <div class="grid gap-2">
    <SettingsPageTitle>Theme balance</SettingsPageTitle><HelperText>
      Switch the palette in the toolbar to compare surfaces, text, borders, and shadows.
    </HelperText>
  </div>
  <div class="grid gap-6 lg:grid-cols-3">
    <section class="grid content-start gap-3 rounded-xl border border-divider bg-sidebar p-4">
      <h2 class="text-sm font-medium">Navigation</h2>
      <span class="rounded-lg px-3 py-2 text-xsm text-muted-foreground">Default destination</span><span
        class="rounded-lg bg-hover px-3 py-2 text-xsm"
      >
        Hovered destination
      </span><span class="rounded-lg bg-selection px-3 py-2 text-xsm text-selection-foreground">Selected destination</span><span
        class="rounded-lg bg-tab-selection px-3 py-2 text-xsm"
      >
        Selected tab
      </span>
    </section>
    <section class="overflow-hidden rounded-xl border border-border bg-surface">
      <h2 class="border-b border-divider bg-toolbar px-4 py-3 text-sm font-medium">Workspace</h2>
      <div class="grid gap-4 p-4 text-xsm">
        <p>Primary text on the content surface</p>
        <p class="text-muted-foreground">Supporting text stays readable without dominating the page.</p>
        <label class="grid gap-2">
          Input border and focus <input
            placeholder="Type a value…"
            class="h-8 min-w-0 rounded-lg border border-input bg-inset px-3 text-foreground\nplaceholder:text-muted-foreground focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-ring"
          />
        </label>
        <div class="flex flex-wrap gap-3">
          <Button variant="primary">Primary action</Button><Button>Neutral action</Button><Button variant="destructive">
            Delete
          </Button><Button disabled>Disabled</Button>
        </div>
        <div class="flex flex-wrap items-center gap-4">
          <label class="inline-flex items-center gap-2"><Checkbox defaultChecked></Checkbox>Selected option </label><label
            class="inline-flex items-center gap-2"
          >
            <Switch defaultChecked></Switch>Enabled
          </label>
        </div>
        <div class="flex flex-wrap gap-3">
          <span class="text-success">Success</span><span class="text-warning">Warning</span><span class="text-destructive">
            Error
          </span><Button variant="link">Link</Button>
        </div>
        <p class="rounded-lg bg-composer p-3">Composer surface</p>
        <p class="w-fit rounded-full bg-error-background px-3 py-1 text-xs text-error-foreground shadow-floating">
          Field validation message
        </p>
      </div>
    </section>
    <section class="grid content-start gap-6 rounded-xl bg-surface p-6">
      <h2 class="text-sm font-medium">Elevation</h2>
      <div class="grid gap-2 rounded-control border border-border bg-menu p-2 text-xsm shadow-menu">
        <span class="px-3 py-2 text-muted-foreground">Menu item</span><span class="rounded-lg bg-menu-hover px-3 py-2">
          Hovered item
        </span><span class="rounded-lg bg-menu-selection px-3 py-2 text-selection-foreground">Selected item</span>
      </div>
      <div class="grid gap-2 rounded-xl border border-border bg-elevated p-4 text-xsm shadow-floating">
        <p>Floating notification</p>
        <p class="text-muted-foreground">Shadow color and strength come from the active palette.</p>
      </div>
      <div class="max-h-20 overflow-auto rounded-lg bg-inset p-3 text-xsm text-muted-foreground">
        {#each Array.from({ length: 8 }) as _, index (index)}<p>Themed scrollbar · row {index + 1}</p>{/each}
      </div>
    </section>
  </div>
</div>
