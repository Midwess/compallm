<script lang="ts">
  import { HelperText } from '../../src/components/feedback'
  import { SettingsPageTitle } from '../../src/components/settings'

  const spaces = [
    ['2', 'w-0.5'],
    ['4', 'w-1'],
    ['6', 'w-1.5'],
    ['8', 'w-2'],
    ['10', 'w-2.5'],
    ['12', 'w-3'],
    ['16', 'w-4'],
    ['24', 'w-6'],
    ['32', 'w-8'],
    ['48', 'w-12'],
    ['64', 'w-16'],
  ] as const
</script>

<div class="grid max-w-xl gap-4 p-6">
  <SettingsPageTitle>Resolved spacing</SettingsPageTitle><HelperText>
    Use half steps on Tailwind’s default scale, starting at 2 px, and preserve justified exceptions.
  </HelperText>{#each spaces as [pixels, width] (pixels)}<div class="flex items-center gap-4 text-xs">
      <span class="w-12 text-muted-foreground">{pixels}px</span><span
        class={`${width} h-4 shrink-0 rounded-sm bg-primary`}
        aria-hidden="true"
      ></span>
    </div>{/each}
</div>
