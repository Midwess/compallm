<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Multiline">
    <Fields.MultilineField aria-label="Description" placeholder="Write a description…"></Fields.MultilineField>
  </Sample><Sample label="Filled">
    <Fields.MultilineField aria-label="Filled description" value="A shared workspace for your team."></Fields.MultilineField>
  </Sample><Sample label="Disabled">
    <Fields.MultilineField aria-label="Disabled description" disabled></Fields.MultilineField>
  </Sample>
</Preview>
