<script lang="ts">
</script>

<div class="grid gap-6 p-6 sm:grid-cols-2">
  <section class="grid gap-3 rounded-xl bg-sidebar p-4">
    <h2 class="text-sm font-medium">Sidebar</h2>
    <span class="rounded-lg px-3 py-2 text-xsm text-muted-foreground">Default destination</span><span
      class="rounded-lg bg-hover px-3 py-2 text-xsm"
    >
      Hovered destination
    </span><span class="rounded-lg bg-selection px-3 py-2 text-xsm text-selection-foreground">Selected destination</span>
  </section>
  <section class="overflow-hidden rounded-xl border border-border bg-surface">
    <h2 class="border-b border-divider bg-toolbar px-4 py-3 text-sm font-medium">Workspace toolbar</h2>
    <div class="grid gap-3 p-4 text-xsm">
      <p>Content surface</p>
      <span class="rounded-lg bg-inset px-3 py-2 text-muted-foreground">Inset surface</span><span
        class="rounded-lg bg-elevated px-3 py-2"
      >
        Elevated control
      </span><span class="w-fit rounded-lg bg-primary px-3 py-2 text-primary-foreground">Primary action</span>
    </div>
  </section>
</div>
