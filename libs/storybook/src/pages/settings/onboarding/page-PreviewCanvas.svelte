<script lang="ts">
  import { OnboardingPage } from '../../onboarding'

  import { untrack } from 'svelte'

  import { toStyle } from '../../page-state.svelte'

  let viewport = $state<HTMLDivElement | null>(null)
  let scale = $state(1)
  $effect(() => {
    void []
    return untrack(() => {
      const element = viewport
      if (!element) return
      const observer = new ResizeObserver(([entry]) => (scale = entry.contentRect.width / 672))
      observer.observe(element)
      return () => observer.disconnect()
    })
  })
</script>

<div
  class="rounded-4xl bg-linear-to-br from-muted-foreground/50 via-border to-elevated
p-3 shadow-menu ring-1 ring-border"
>
  <div
    class="relative overflow-hidden rounded-3xl p-6 ring-1 ring-background/70 sm:p-8"
    style={toStyle({
      backgroundColor: 'color-mix(in srgb, var(--primary) 9%, var(--background))',
      backgroundImage: 'radial-gradient(circle, color-mix(in srgb, var(--primary) 18%, transparent) 0.5px, transparent 1px)',
      backgroundSize: '16px 16px',
    })}
  >
    <div bind:this={viewport} class="relative aspect-21/13 w-full">
      <div
        class="absolute top-0 left-0 w-2xl origin-top-left rounded-3xl shadow-menu
ring-1 ring-border [&_main]:min-h-104"
        style={toStyle({ transform: `scale(${scale})` })}
      >
        <OnboardingPage></OnboardingPage>
      </div>
    </div>
  </div>
</div>
