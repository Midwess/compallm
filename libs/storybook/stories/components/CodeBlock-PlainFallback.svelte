<script lang="ts">
  import { CodePreview } from '../../src/components/code-block'
  const source = '\u003cscript>never execute displayed code\u003c/script>\n<&>'
</script>

<CodePreview code={source} language="unknown language" />
