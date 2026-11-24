<script lang="ts">
  import * as Buttons from '../../src/components/buttons'
  import Preview from './Preview.svelte'
  import Sample from './Sample.svelte'
</script>

<Preview>
  <Sample label="Content · 166 × 28">
    <Buttons.ContentButton>Use custom configuration</Buttons.ContentButton><Buttons.ContentButton disabled>
      Disabled
    </Buttons.ContentButton>
  </Sample>
</Preview>
