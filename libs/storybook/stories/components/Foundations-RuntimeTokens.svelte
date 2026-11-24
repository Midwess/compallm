<script lang="ts">
  import { Button } from '../../src/components/buttons'
</script>

<div class="grid gap-6 p-6 sm:grid-cols-2">
  {#each [{ name: 'Default', style: {} }, { name: 'Scoped color and larger text', style: { '--background': 'oklch(0.32 0.04 250)', '--foreground': 'oklch(0.96 0.01 250)', '--primary': 'oklch(0.72 0.15 160)', '--shadow-color': 'oklch(0.18 0.04 250)', '--ui-font-scale': '1.25' } as Record<string, string> }] as { name, style } (name)}<section
      style={Object.entries(style)
        .map(([key, value]) => `${key}:${value}`)
        .join(';')}
      class="grid gap-4 rounded-lg bg-background p-6 text-foreground"
    >
      <h2 class="text-ui-20 font-medium">{name}</h2>
      <p class="text-xsm">Semantic colors and type scale apply at runtime.</p>
      <span class="w-fit rounded bg-primary px-3 py-2 text-xs text-primary-foreground">Primary action</span><Button>
        Control shadow
      </Button>
    </section>{/each}
</div>
