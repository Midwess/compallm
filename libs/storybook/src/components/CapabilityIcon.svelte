<script lang="ts">
  import { capabilityArtwork, type CapabilityKind } from './capability-data'
  import { cx } from './shared'
  import SparkleIcon from 'phosphor-svelte/lib/SparkleIcon'
  let { capability, class: className }: { capability: CapabilityKind; class?: string } = $props()
  let art = $derived(capabilityArtwork[capability])
  let Icon = $derived(art.icon)
</script>

<span
  aria-hidden="true"
  data-capability={capability}
  class={cx('relative inline-flex size-4 shrink-0 items-center justify-center', art.tone, className)}
>
  <Icon size={14} />{#if capability === 'image_generation' || capability === 'forced_tool_choice'}<SparkleIcon
      size={7}
      class="absolute -top-0.5 -right-0.5"
    />{/if}
</span>
