<script lang="ts">
  import {
    CapabilityIcon,
    CapabilityIndicator,
    capabilityLabels,
    type CapabilityKind,
  } from '../../src/components/capability-icon'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
  import Row from './Row.svelte'
</script>

<Preview>
  <Sample label="Capability SVGs · consistent Phosphor Regular artwork" wide>
    <div class="grid grid-cols-1 gap-4 p-3 sm:grid-cols-3">
      {#each Object.keys(capabilityLabels) as CapabilityKind[] as capability (capability)}<div class="flex items-center gap-3">
          <CapabilityIcon {capability} class="size-6"></CapabilityIcon><span class="text-xsm">
            {capabilityLabels[capability]}
          </span>
        </div>{/each}
    </div>
  </Sample><Sample label="Model-column size · focus or hover for labels" wide>
    <Row>
      {#each Object.keys(capabilityLabels) as CapabilityKind[] as capability (capability)}<CapabilityIndicator {capability}
        ></CapabilityIndicator>{/each}
    </Row>
  </Sample>
</Preview>
