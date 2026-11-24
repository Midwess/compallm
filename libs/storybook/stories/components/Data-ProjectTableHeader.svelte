<script lang="ts">
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Column headers" wide>
    <Data.ProjectTable label="Project columns"><Data.ProjectTableHeader></Data.ProjectTableHeader></Data.ProjectTable>
  </Sample>
</Preview>
