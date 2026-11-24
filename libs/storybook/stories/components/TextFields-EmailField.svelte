<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Authentication · 48 px">
    <Fields.EmailField aria-label="Email address" placeholder="you@example.com"></Fields.EmailField>
  </Sample><Sample label="Filled">
    <Fields.EmailField aria-label="Filled email" value="alex@example.com"></Fields.EmailField>
  </Sample><Sample label="Disabled"><Fields.EmailField aria-label="Disabled email" disabled></Fields.EmailField></Sample>
</Preview>
