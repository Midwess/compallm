<script lang="ts">
  let { title = 'Svelte foundations' }: { title?: string } = $props()
</script>

<section class="rounded-control border border-border bg-surface p-6 font-sans text-foreground">
  <h1 class="text-xl font-semibold">{title}</h1>
  <p class="mt-2 text-sm text-muted-foreground">Shared typography, spacing, and semantic colors.</p>
  <div class="mt-4 rounded-control bg-primary px-4 py-2 text-primary-foreground">Primary surface</div>
  <code class="mt-4 block font-mono text-sm">Svelte + Tailwind</code>
</section>
