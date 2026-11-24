<script lang="ts">
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Property rows">
    <Data.PropertyRow label="Priority"><Data.PropertyChip>No priority</Data.PropertyChip></Data.PropertyRow><Data.PropertyRow
      label="Lead"
    >
      <Data.PropertyChip>Add lead</Data.PropertyChip>
    </Data.PropertyRow>
  </Sample>
</Preview>
