<script lang="ts">
  import * as Content from '../../src/components/content'

  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Section heading" wide>
    <Content.SectionHeading description="A short description of this section.">Section title</Content.SectionHeading>
  </Sample>
</Preview>
