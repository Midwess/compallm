<script lang="ts">
  import { cx } from './shared'
  let { name, src, size = 40 }: { name: string; src?: string | null; size?: number } = $props()
  let loaded = $state(false)
  let initial = $derived(Array.from(name.trim())[0]?.toUpperCase() || '?')
  const tones = [
    'bg-primary/25 text-primary',
    'bg-success/25 text-success',
    'bg-warning/25 text-warning',
    'bg-destructive/25 text-destructive',
    'bg-muted-foreground/25 text-foreground',
  ]
  let tone = $derived(tones[(initial.codePointAt(0) ?? 0) % tones.length])
  $effect(() => {
    void src
    loaded = false
  })
</script>

<span
  role="img"
  aria-label={name}
  class={cx(
    'relative inline-flex flex-none items-center justify-center overflow-hidden rounded-full text-xsm size-(--avatar-size) font-medium [&>img]:size-full [&>img]:object-cover',
    tone,
  )}
  style={`--avatar-size:${size}px`}
>
  <span aria-hidden="true">{initial}</span>{#if src}<img
      {src}
      alt=""
      class={cx('absolute inset-0', !loaded && 'opacity-0')}
      onload={() => (loaded = true)}
      onerror={() => (loaded = false)}
    />{/if}
</span>
