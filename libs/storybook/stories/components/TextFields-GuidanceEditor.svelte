<script lang="ts">
  import * as Fields from '../../src/components/fields'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Guidance" wide>
    <Fields.GuidanceEditor aria-label="Personal guidance" placeholder="Enter personal guidance (optional)…"
    ></Fields.GuidanceEditor>
  </Sample>
</Preview>
