<script lang="ts">
  import * as Forms from '../../src/components/forms'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Form introduction" wide>
    <Forms.FormIntroduction title="Add a connection">Connect a service to your workspace.</Forms.FormIntroduction>
  </Sample>
</Preview>
