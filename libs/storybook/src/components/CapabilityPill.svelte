<script lang="ts">
  import CapabilityIcon from './CapabilityIcon.svelte'
  import { capabilityLabels, type CapabilityKind } from './capability-data'
  let { capability }: { capability: CapabilityKind } = $props()
</script>

<span class="inline-flex items-center gap-1 rounded bg-elevated px-1 py-0.5 text-ui-11">
  <CapabilityIcon {capability} />{capabilityLabels[capability]}
</span>
