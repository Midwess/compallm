<script lang="ts">
  import type { Snippet } from 'svelte'
  import LoginShader from './LoginShader.svelte'
  let { children }: { children?: Snippet } = $props()
</script>

<main class="relative isolate flex min-h-svh flex-col overflow-hidden bg-background px-6 font-sans text-foreground">
  <div class="pointer-events-none absolute inset-0 -z-10 bg-radial from-primary/10 to-background">
    <LoginShader />
    <div class="absolute inset-0 bg-linear-to-b from-background/98 from-30% via-background/70 via-60% to-transparent"></div>
  </div>
  <section aria-label="Account access" class="m-auto w-full max-w-88 py-12">{@render children?.()}</section>
  <p class="pb-6 text-center text-xs text-muted-foreground">A little space for your next big idea.</p>
</main>
