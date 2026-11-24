<script lang="ts">
  import * as Navigation from '../../src/components/navigation'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Back navigation"><Navigation.BackBreadcrumb href="#documents">Documents</Navigation.BackBreadcrumb></Sample>
</Preview>
