<script lang="ts">
  import * as Data from '../../src/components/data'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Project row" wide>
    <Data.ProjectTable label="Projects">
      <Data.ProjectTableHeader></Data.ProjectTableHeader><tbody>
        <Data.ProjectTableRow name="Compallm" lead="Alex Morgan" progress={35}></Data.ProjectTableRow><Data.ProjectTableRow
          name="Example project"
        ></Data.ProjectTableRow>
      </tbody>
    </Data.ProjectTable>
  </Sample>
</Preview>
