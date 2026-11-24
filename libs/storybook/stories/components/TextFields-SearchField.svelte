<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Search"><Fields.SearchField aria-label="Search" placeholder="Search…"></Fields.SearchField></Sample><Sample
    label="Disabled"
  >
    <Fields.SearchField aria-label="Disabled search" disabled placeholder="Search…"></Fields.SearchField>
  </Sample>
</Preview>
