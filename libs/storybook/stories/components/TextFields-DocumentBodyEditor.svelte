<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Document body" wide>
    <Fields.DocumentBodyEditor aria-label="Document body" placeholder="Start writing your document…"
    ></Fields.DocumentBodyEditor>
  </Sample>
</Preview>
