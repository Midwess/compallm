<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Code text" wide>
    <Content.CodePreview
      language="TypeScript"
      code={'async function loadItems() {\n  const response = await fetch("/items");\n  return response.json();\n}'}
    ></Content.CodePreview>
  </Sample>
</Preview>
