<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Changed file"><Content.ChangedFile path="src/response.ts" added={2} removed={1}></Content.ChangedFile></Sample>
</Preview>
