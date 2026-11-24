<script lang="ts">
  let { value }: { value: number } = $props()
  let percent = $derived(Number.isFinite(value) ? Math.min(100, Math.max(0, value)) : 0)
</script>

<span
  class="inline-flex items-center gap-1 text-ui-12 text-muted-foreground [&>progress]:h-1 [&>progress]:w-8 [&>progress]:accent-primary"
>
  <progress max={100} value={percent} aria-label="Project completion"></progress>{percent}%
</span>
