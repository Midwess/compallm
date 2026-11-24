<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Context, added, removed" wide>
    <Content.DiffLine lineNumber={10}>const response = await loadItems();</Content.DiffLine><Content.DiffLine
      lineNumber={11}
      change="added"
    >
      return response.data;
    </Content.DiffLine><Content.DiffLine lineNumber={12} change="removed">return response.json();</Content.DiffLine>
  </Sample>
</Preview>
