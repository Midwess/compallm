<script lang="ts">
  import CapabilityIcon from './CapabilityIcon.svelte'
  import TooltipControl from './TooltipControl.svelte'
  import { capabilityLabels, type CapabilityKind } from './capability-data'
  let { capability }: { capability: CapabilityKind } = $props()
</script>

<TooltipControl message={capabilityLabels[capability]}><CapabilityIcon {capability} /></TooltipControl>
