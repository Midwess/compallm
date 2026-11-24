<script lang="ts">
  import CubeGrid from './CubeGrid.svelte'
  import Button from './Button.svelte'
  let {
    active,
    error,
    onRetry,
    reducedMotion = false,
  }: { active: boolean; error?: string; onRetry?: () => void; reducedMotion?: boolean } = $props()
</script>

{#if active}<div
    class="fixed inset-0 z-100 flex flex-col items-center justify-center gap-6 bg-inset bg-radial-[at_50%_42%] from-elevated to-inset to-70% px-6 text-foreground"
    aria-label="Workspace loading"
  >
    <CubeGrid stopped={!!error} {reducedMotion} />{#if error}<p
        role="alert"
        class="max-w-sm text-center text-xsm text-muted-foreground"
      >
        {error}
      </p>{:else}<span role="status" class="sr-only">Loading your workspace…</span>{/if}{#if error && onRetry}<Button
        onclick={onRetry}
      >
        Retry
      </Button>{/if}
  </div>{/if}
